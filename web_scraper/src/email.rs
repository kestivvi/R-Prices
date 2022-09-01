use database::models::price::{Availability, Price};
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn email_many(
    name: &str,
    url: &str,
    old_price: &Price,
    new_price: &Price,
    user_emails: &[String],
) {
    // Create body of the email
    // It's loaded file email.html with replaced variables

    let what_has_changed = if old_price.availability == new_price.availability {
        "Price"
    } else {
        "Availability"
    };

    let previous_state = if old_price.availability == Availability::Available {
        let old_price_rounded = format!("{:.2}", old_price.value.unwrap());
        format!("{} PLN", old_price_rounded)
    } else {
        old_price.availability.to_string()
    };

    let current_state = if new_price.availability == Availability::Available {
        let new_price_rounded = format!("{:.2}", new_price.value.unwrap());
        format!("{} PLN", new_price_rounded)
    } else {
        new_price.availability.to_string()
    };

    let single_part = include_str!("email.html")
        .replace("[WHAT_HAS_CHANGED]", what_has_changed)
        .replace("[PRODUCT]", name)
        .replace(
            "[PREVIOUS_DATE]",
            &old_price.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        )
        .replace("[PREVIOUS_STATE]", &previous_state)
        .replace(
            "[CURRENT_DATE]",
            &new_price.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        )
        .replace("[CURRENT_STATE]", &current_state)
        .replace("[URL]", url);

    let single_part = SinglePart::html(single_part);

    let subject = format!(
        "R Prices - {} - Then: {}, Now: {}",
        crop_string(name, 25),
        previous_state,
        current_state
    );

    let creds = Credentials::new(
        "kestivvi@gmail.com".to_string(),
        "wvldufqhmdxogmej".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    for user_email in user_emails {
        let email = Message::builder()
            .from("kestivvi <kestivvi@gmail.com>".parse().unwrap())
            .to(user_email.parse().unwrap())
            .subject(&subject)
            .singlepart(single_part.clone())
            .unwrap();

        match mailer.send(&email) {
            Ok(_) => log::info!("Email sent successfully! {}", subject),
            Err(e) => log::error!("Could not send email:\nEmail: {:?}\nError: {:?}", email, e),
        }
    }
}

fn crop_string(s: &str, length: usize) -> String {
    if s.len() <= length {
        s.to_owned()
    } else {
        format!("{}...", &s[..length])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(crop_string("Hello World", 5), "Hello...");
    }

    #[test]
    fn test2() {
        assert_eq!(crop_string("Hello", 5), "Hello");
    }

    #[test]
    fn test3() {
        assert_eq!(crop_string("", 8), "");
    }

    #[test]
    fn test4() {
        assert_eq!(crop_string("Hello World", 0), "...");
    }
}
