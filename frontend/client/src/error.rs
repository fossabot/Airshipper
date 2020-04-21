/// Uses `human_panic` to setup friendly panic messages
pub fn setup_panic_hook() {
    human_panic::setup_panic!(Metadata {
        name: "Airshipper".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: "Songtronix <contact@songtronix.com>".into(),
        homepage: "songtronix.com".into(),
    });
}
