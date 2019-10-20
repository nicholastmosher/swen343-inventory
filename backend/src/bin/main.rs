use dotenv::dotenv;
use erp::app::AppConfig;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let actor_system = actix::System::new("inventory");
    let config = AppConfig::from_env().expect("should load app config from env");
    erp::app::launch(&config)?;
    println!("Launched app on {}", &config.bind_address);
    actor_system.run()?;
    Ok(())
}
