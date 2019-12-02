#[macro_use]
extern crate log;

use dotenv::dotenv;
use erp::app::AppConfig;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let actor_system = actix::System::new("inventory");
    let config = AppConfig::from_env().expect("should load app config from env");
    info!("Using {:?}", config);
    erp::app::launch(&config)?;
    println!("Launched app on {}", &config.bind_address);
    actor_system.run()?;
    Ok(())
}
