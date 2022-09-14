use actix_web::cookie::{Cookie, Expiration};
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, HttpResponse};
use actix_web::{HttpRequest, Responder};
use database::models::user::{NewEmail, NewName, PasswordChange, RegisterLoginUserInput};

use database::db::PostgresPool;
use serde_json::json;

pub fn auth_endpoints(config: &mut web::ServiceConfig) {
    config
        .route("/auth/register", web::post().to(register))
        .route("/auth/login", web::post().to(login))
        .route("/auth/me", web::get().to(me))
        .route("/auth/status", web::get().to(status))
        .route("/auth/logout", web::post().to(logout))
        .route("/auth/changeName", web::post().to(change_name))
        .route("/auth/changeEmail", web::post().to(change_email))
        .route("/auth/changePassword", web::post().to(change_password));
}

async fn register(
    pool: web::Data<PostgresPool>,
    new_user: web::Json<RegisterLoginUserInput>,
) -> actix_web::Result<impl Responder> {
    let conn = pool.get().unwrap();

    // TODO: Validate length, passwords streghtness etc.
    // VALIDATE
    let user_with_the_same_email =
        database::models::user::queries::get_user_by_email(&conn, &new_user.email).ok();

    if user_with_the_same_email.is_some() {
        return Ok(HttpResponse::Conflict().body("User with that email already exists"));
    };

    let user_with_the_same_name =
        database::models::user::queries::get_user_by_name(&conn, &new_user.name).ok();

    if user_with_the_same_name.is_some() {
        return Ok(HttpResponse::Conflict().body("User with that name already exists"));
    };

    let new_user = database::models::user::mutations::register_user(&conn, new_user.into_inner());

    match new_user {
        Ok(user) => {
            let session_id = database::models::session::mutations::insert(&conn, user.id).unwrap();
            let cookie = Cookie::build("session_id", session_id.to_string())
                .http_only(true)
                // TODO: Move duration to some config
                // .max_age(actix_web::cookie::time::Duration::minutes(10))
                .expires(Expiration::Session)
                .same_site(actix_web::cookie::SameSite::None)
                .path("/")
                .secure(true)
                .finish();
            Ok(HttpResponse::Created().cookie(cookie).await.unwrap())
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Cannot create user",
        )),
    }
}

async fn login(
    pool: web::Data<PostgresPool>,
    login_input: web::Json<RegisterLoginUserInput>,
) -> actix_web::Result<impl Responder> {
    let conn = pool.get().unwrap();

    let user = database::models::user::queries::login_user(&conn, login_input.into_inner());

    match user {
        Ok(user) => {
            let session_id = database::models::session::mutations::insert(&conn, user.id).unwrap();
            let cookie = Cookie::build("session_id", session_id.to_string())
                .http_only(true)
                // TODO: Move duration to some config
                // .max_age(actix_web::cookie::time::Duration::minutes(10))
                .same_site(actix_web::cookie::SameSite::None)
                .expires(Expiration::Session)
                .path("/")
                // .domain("127.0.0.1")
                .secure(true)
                .finish();
            // session.insert("session_id", session_id).unwrap();
            // debug!("{:?}", session.entries());
            Ok(HttpResponse::Ok().cookie(cookie).await)
        }
        Err(_) => Err(actix_web::error::ErrorNotFound("User not found")),
    }
}

async fn me(
    request: HttpRequest,
    pool: web::Data<PostgresPool>,
) -> actix_web::Result<impl Responder> {
    // let session_id: Option<i32> = session.get("session_id").ok().unwrap_or(None);
    let session_id = request
        .cookie("session_id")
        .map(|v| v.value().parse().unwrap());

    match session_id {
        Some(session_id) => {
            let conn = pool.get().unwrap();
            let user_id = match database::models::session::queries::get_user_id(&conn, session_id) {
                Some(v) => v,
                None => return Err(ErrorUnauthorized("Your session expired")),
            };
            let user = database::models::user::queries::get_user_by_id(&conn, user_id).unwrap();

            // TODO: Prolong session cookie
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                    "name": user.name,
                    "email": user.email
                })))
        }
        None => Err(ErrorUnauthorized("You're not logged in!")),
    }
}

async fn status(
    request: HttpRequest,
    pool: web::Data<PostgresPool>,
) -> actix_web::Result<impl Responder> {
    // debug!("Request Cookies: {:?}", request.cookies());

    let session_id = request
        .cookie("session_id")
        .map(|v| v.value().parse().unwrap());

    let mut is_logged_in = false;

    if let Some(session_id) = session_id {
        let conn = pool.get().unwrap();
        let user_id = database::models::session::queries::get_user_id(&conn, session_id);
        is_logged_in = user_id.is_some();
    }

    let response = HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "loggedIn": is_logged_in,
        }));

    Ok(response)
}

async fn logout(
    request: HttpRequest,
    pool: web::Data<PostgresPool>,
) -> actix_web::Result<impl Responder> {
    let session_id = request
        .cookie("session_id")
        .map(|v| v.value().parse().unwrap());

    let response = match session_id {
        Some(session_id) => {
            let mut cookie = Cookie::build("session_id", "")
                .path("/")
                .http_only(true)
                .secure(true)
                .same_site(actix_web::cookie::SameSite::None)
                .expires(Expiration::Session)
                .finish();
            cookie.make_removal();

            let conn = pool.get().unwrap();

            database::models::session::mutations::remove_by_session_id(&conn, session_id);
            HttpResponse::Ok().cookie(cookie).finish()
        }
        None => HttpResponse::Ok().finish(),
    };

    Ok(response)
}

async fn change_name(
    request: HttpRequest,
    pool: web::Data<PostgresPool>,
    new_name_obj: web::Json<NewName>,
) -> actix_web::Result<impl Responder> {
    let session_id = request
        .cookie("session_id")
        .map(|v| v.value().parse::<i32>().unwrap());

    let session_id = match session_id {
        Some(v) => v,
        None => return Err(ErrorUnauthorized("You're not logged in!")),
    };

    let conn = pool.get().unwrap();
    let user_id = database::models::session::queries::get_user_id(&conn, session_id);

    let user_id = match user_id {
        Some(v) => v,
        None => return Err(ErrorUnauthorized("Your session expired!")),
    };

    let user_with_that_name =
        database::models::user::queries::get_user_by_name(&conn, &new_name_obj.new_name);

    if user_with_that_name.is_ok() {
        return Err(ErrorUnauthorized(
            "This name is already in use by other user!",
        ));
    }

    let res =
        database::models::user::mutations::change_name(&conn, user_id, &new_name_obj.new_name);

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "{:?}",
            e
        ))),
    }
}

async fn change_email(
    request: HttpRequest,
    pool: web::Data<PostgresPool>,
    new_email_obj: web::Json<NewEmail>,
) -> actix_web::Result<impl Responder> {
    let session_id = request
        .cookie("session_id")
        .map(|v| v.value().parse::<i32>().unwrap());

    let session_id = match session_id {
        Some(v) => v,
        None => return Err(ErrorUnauthorized("You're not logged in!")),
    };

    let conn = pool.get().unwrap();
    let user_id = database::models::session::queries::get_user_id(&conn, session_id);

    let user_id = match user_id {
        Some(v) => v,
        None => return Err(ErrorUnauthorized("Your session expired!")),
    };

    let user_with_that_email =
        database::models::user::queries::get_user_by_email(&conn, &new_email_obj.new_email);

    if user_with_that_email.is_ok() {
        return Err(ErrorUnauthorized(
            "This email is already in use by other user!",
        ));
    }

    let res =
        database::models::user::mutations::change_email(&conn, user_id, &new_email_obj.new_email);

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "{:?}",
            e
        ))),
    }
}

async fn change_password(
    request: HttpRequest,
    pool: web::Data<PostgresPool>,
    password_change_obj: web::Json<PasswordChange>,
) -> actix_web::Result<impl Responder> {
    let session_id = request
        .cookie("session_id")
        .map(|v| v.value().parse::<i32>().unwrap());

    let session_id = match session_id {
        Some(v) => v,
        None => return Err(ErrorUnauthorized("You're not logged in!")),
    };

    let conn = pool.get().unwrap();
    let user_id = database::models::session::queries::get_user_id(&conn, session_id);

    let user_id = match user_id {
        Some(v) => v,
        None => return Err(ErrorUnauthorized("Your session expired!")),
    };

    let correct_password = database::models::user::queries::is_that_user_have_that_passwd(
        &conn,
        user_id,
        &password_change_obj.old_password,
    );

    if !correct_password {
        return Err(ErrorUnauthorized("Provided old password is incorect!"));
    }

    let res = database::models::user::mutations::change_password(
        &conn,
        user_id,
        &password_change_obj.new_password,
    );

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "{:?}",
            e
        ))),
    }
}
