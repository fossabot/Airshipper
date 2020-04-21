// How to send manual webhooks:
// curl --header "Content-Type: application/json" --request POST --data "@<FILE_WITH_WEBHOOK_DATA>" --header "X-Gitlab-Event: Pipeline Hook" --header "X-Gitlab-Token: <TOKEN>" http://<ADDRESS>
mod logger;

pub fn main() {
    // This is only going to be used on local developement
    // because in production we use docker containers anyway
    dotenv::from_filename("backend/.airshipper-env").ok();
    logger::init();
    server::rocket().launch().expect("Server failed!");
}
