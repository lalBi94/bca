mod client;
mod cli;

use shared::fchain::CBCAConfig;
use crate::{cli::CBCACli, client::CBCAClient};
// async fn cli() -> Result<(), std::io::Error> {
//     let cli = CBCACli::spawn(Some("Bilal".to_string()));
//     cli.run_cli();
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let client: CBCAClient = CBCAClient::spawn(
        "127.0.0.1:8686".to_string(),
        "127.0.0.1:8687".to_string(),
        "127.0.0.1:8688".to_string()
    );

    let instance_id = client.send_instance(
        CBCAConfig::spawn(
            None, 
            false, 
            None, 
            30, 
            "Objet sympa".to_string(), 
            "Tableau Van Gogh".to_string(), 
            "EUR".to_string()
        )?
    ).await?;

    //client.send_message("Bilal".to_string(), "Bonjour".to_string(), instance_id.to_string()).await?;

    println!("{:?}", instance_id);

    Ok(())
}