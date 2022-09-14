use env_logger::Env;

pub fn init_env_and_logging() {
    dotenv::dotenv().ok();
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
}
