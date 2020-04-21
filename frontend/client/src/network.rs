// Name your user agent after your app?
static APP_USER_AGENT: &str = concat!("Airshipper/", env!("CARGO_PKG_VERSION"));

lazy_static::lazy_static! {
    // Base for config, profiles, ...
    pub static ref WEB_CLIENT: reqwest::Client = {
        reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(10))
            .build().expect("FATAL: Failed to build reqwest client!")
    };
}
