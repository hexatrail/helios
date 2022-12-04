use config::CliConfig;
use eyre::Result;

use helios::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Load the config from the global config file
    let config_path = home::home_dir().unwrap().join(".helios/helios.toml");
    let config = Config::from_file(&config_path, "mainnet", &CliConfig::default());
    println!("Constructed config: {config:#?}");

    Ok(())
}
