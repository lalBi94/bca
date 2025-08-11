use std::{env, path::PathBuf, sync::Arc};
use tokio::{fs::OpenOptions, io::{AsyncReadExt, AsyncWriteExt}};
use shared::{
    block::CBCABlock, 
    fchain::CBCAChain, 
    payload::{MPayload, OPayload}
};
use crate::instance::CBCAInstance;

#[derive(Debug, Clone)]
pub struct CBCAManager {
    access: Arc<tokio::sync::Mutex<()>>,
    current_path: std::path::PathBuf
}

impl CBCAManager {
    pub fn spawn() -> Result<Self, std::io::Error> {
        Ok(
            Self {
                access: Arc::new(tokio::sync::Mutex::new(())),
                current_path: env::current_dir()?.join("data")
            }
        )
    }

    pub async fn hard_push_msg(
        &self, 
        payload: MPayload
    ) -> Result<(), std::io::Error> {
        let _ = self.access.lock().await;
        
        let path: String = self.current_path
                .join(&payload.instance_id.replace("-", "."))
                .join("m.bca.json")
                .display()
                .to_string();

        let mut file_r = OpenOptions::new()
            .read(true)
            .open(&path)
            .await?;

        let mut buf: String = String::new();
        let _ = file_r.read_to_string(&mut buf).await?;

        if buf.len() == 0 {
            panic!("[E] empty buffer.");
        } else {
            drop(file_r);
        }

        let mut file_w = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .await?;

        let mut parsed: CBCAChain = serde_json::from_str(&buf)?;
        let capture: Option<String> = parsed.hash.clone();
        let nhash_block = parsed.push(
            CBCABlock::block_creator_message(payload.content, payload.author, payload.instance_id.clone())
        )?;
        let serialized: String = serde_json::to_string(&parsed)?;
        println!("[UP 1/2] pushing {:?} in {:?}.", nhash_block, &payload.instance_id);
        println!("[UP 2/2] chain hash changing from {:?} to {:?}.", capture, &parsed.hash);

        file_w.write_all(serialized.as_bytes()).await?;

        Ok(())
    }

    pub async fn hard_push_offer(
        &self, 
        payload: OPayload
    ) -> Result<(), std::io::Error> {
        let _ = self.access.lock().await;
        let path: String = self.current_path
            .join(&payload.instance_id.replace("-", "."))
            .join("o.bca.json")
            .display()
            .to_string();

        let mut file_r = OpenOptions::new()
            .read(true)
            .open(&path)
            .await?;

        let mut buf: String = String::new();
        let _ = file_r.read_to_string(&mut buf).await?;

        if buf.len() == 0 {
            panic!("[E] empty buffer.");
        } else {
            drop(file_r);
        }

        let mut file_w = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .await?;

        let mut parsed: CBCAChain = serde_json::from_str(&buf)?;
        let capture: Option<String> = parsed.hash.clone();
        let nhash_block = parsed.push(CBCABlock::block_creator_offer(payload.amount, payload.author, payload.instance_id.clone(), payload.message))?;
        let serialized: String = serde_json::to_string(&parsed)?;
        println!("[UP 1/2] pushing {:?} in {:?}.", nhash_block, &payload.instance_id);
        println!("[UP 2/2] chain hash changing from {:?} to {:?}.", capture, &parsed.hash);

        file_w.write_all(serialized.as_bytes()).await?;

        Ok(())
    }

    
    pub async fn hard_create(
        &self, 
        instance: CBCAInstance
    ) -> Result<String, std::io::Error> {
        let _ =  self.access.lock().await;

        println!("{:?}", instance);

        let path: PathBuf = self.current_path
            .join(instance.identifier.replace("-", "."));

        println!("[CREATE 1/4] {}", &path.display().to_string());
        tokio::fs::create_dir(&path).await?;

        let offer_chain_path: String = path.join("o.bca.json").display().to_string();
        println!("[CREATE 2/4] {}", &offer_chain_path);
        let mut file = tokio::fs::File::create(offer_chain_path).await?;
        let _ = file.write_all(
            serde_json::to_string(&instance.offers_chain)?.as_bytes()
        ).await?;

        let message_chain_path: String = path.join("m.bca.json").display().to_string();
        println!("[CREATE 3/4] {}", &message_chain_path);
        let mut file = tokio::fs::File::create(message_chain_path).await?;
        let _ = file.write_all(
            serde_json::to_string(&instance.messages_chain)?.as_bytes()
        ).await?;

        let config_path: String = path.join("c.bca.json").display().to_string();
        println!("[CREATE 4/4] {}", &config_path);
        let mut file = tokio::fs::File::create(config_path).await?;
        let _ = file.write_all(
            serde_json::to_string(&instance.config)?.as_bytes()
        ).await?;

        Ok(instance.identifier)
    }
}
