use std::env;
use dotenv::dotenv;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let actor_system = actix::System::new("inventory");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS should be set");
    println!("Launching app on {}", &bind_address);
    erp::app::launch(database_url, bind_address)?;
    actor_system.run()?;
    Ok(())
}
