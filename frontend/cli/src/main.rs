use cli::Action;
use client::{error, logger, profile::Profile, Level, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup panic Hook
    error::setup_panic_hook();

    // Get arguments
    let opts = cli::parse();
    dbg!(&opts);

    // Set Airshipper's Verbosity
    // v = DEBUG, vv = TRACE
    match opts.debug {
        0 => logger::init(Level::INFO),
        1 => logger::init(Level::DEBUG),
        _ => logger::init(Level::TRACE),
    }

    // Set Veloren's Verbosity
    // v = INFO, vv = DEBUG, vvv = TRACE
    match opts.verbose {
        0 => logger::set_voxygen_log_level(Level::WARN),
        1 => logger::set_voxygen_log_level(Level::INFO),
        2 => logger::set_voxygen_log_level(Level::DEBUG),
        _ => logger::set_voxygen_log_level(Level::TRACE),
    }

    let mut client = client::new()?.load().await?; // Shortcut: client::load() -> Result<Client>
    //                        \_Default Client returns immediately
    //                                \_Returns Future with Result<Client> which can be wrapped in a Command!
    match opts.action {
        // Default action: Update if possible and then start.
        None => {
            if client.check_update(Profile::Latest).await? {
                // Returns Result<bool> which can be acted upon by the GUI ( state transition to `CheckingForUpdates`)

                // use indicativ progressbar

                while let Some(progress) = client.update().next().await? {
                    //                                  \_ Returns Stream<Item = Result<UpdateProgress>>
                    //                                     USE async-stream FOR THAT SO WE CAN YIELD!
                    println!("PROGRESS! : {}", progress); // Has stuff like percentage, total bytes, speed
                }
            }

            while let Some(progress) = client.start(Profile::Latest, APP::VOXYGEN).next().await? {
                //                              \_ Returns Stream<Item = Result<ProcessUpdate>>
            }
            // TODO: ^ https://docs.rs/tokio/0.2.14/tokio/process/index.html
        },
        Some(action) => match action {
            Action::Start => {},
            Action::Update => {},
        },
    }

    Ok(())
}
