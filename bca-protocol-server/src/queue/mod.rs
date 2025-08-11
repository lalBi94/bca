use std::{ops::DerefMut, sync::Arc, time::Duration};

use shared::{
    block::CBCABlockType,
    payload::{IPayload, MPayload, OPayload, Payload}
};

use crate::{
    instance::CBCAInstance,
    manager::CBCAManager
};

#[derive(Debug, Clone)]
pub struct CBCAQueue {
    queue: Arc<tokio::sync::Mutex<Vec<CBCABlockType>>>,
    remaining: usize,
    manager: CBCAManager
}

impl CBCAQueue{
    pub fn spawn() -> Result<Self, std::io::Error> {
        Ok(
            Self {
                queue: Arc::new(tokio::sync::Mutex::new(Vec::new())),
                remaining: 0,
                manager: CBCAManager::spawn()?
            }
        )
    }

    pub async fn handle_add_message(
        &self, 
        payload: MPayload
    ) -> Result<(), std::io::Error> {
        self.manager.hard_push_msg(payload).await?;
        Ok(())
    }

    pub async fn handle_add_offer(
        &self, 
        payload: OPayload
    ) -> Result<(), std::io::Error> {
        self.manager.hard_push_offer(payload).await?;
        Ok(())
    }

    pub async fn handle_add_instance(
        &self,
        payload: IPayload
    ) -> Result<String, std::io::Error> {
        println!("recu");
        let instance: CBCAInstance = 
            CBCAInstance::spawn(payload.extract_config().clone(), payload.get_instance_id());
        let identifier: String = self.manager.hard_create(instance).await?;
        Ok(identifier)
    }

    pub async fn routine(
        &self
    ) -> Result<(), std::io::Error> {
        loop {
            if self.remaining > 0 {
                let poped: Option<CBCABlockType> = self.remove_wait_action().await;

                match poped {
                    Some(v) => {
                        let payload: CBCABlockType = v;

                        match payload {
                            CBCABlockType::MESSAGE(mpayload) => {
                                self.handle_add_message(mpayload).await?;
                            },
                            CBCABlockType::OFFER(opayload) => {
                                self.handle_add_offer(opayload).await?;
                            },
                            CBCABlockType::INSTANCE(ipayload) => {
                                let response: Result<String, std::io::Error> = self.handle_add_instance(ipayload).await;
                            }
                        }
                    },
                    None => {
                        println!("[E] error when pushing on chain.");
                    }
                }
            } else {
                println!("[...] no action...");
            }

            tokio::time::sleep(Duration::from_secs_f64(2.0)).await;
            tokio::task::yield_now().await;
        }
    }
    
    pub async fn add_wait_action(
        &self, 
        payload: CBCABlockType
    ) -> () {
        println!("recu");
        let mut lock = self.queue.lock().await;
        lock.push(payload);
    }

    pub async fn remove_wait_action(
        &self
    ) -> Option<CBCABlockType> {
        let mut lock = self.queue.lock().await;
        let block: Option<CBCABlockType> = lock.pop();
        block
    }

    pub fn display(&self) -> () {
        println!("{:#?}", self.queue);
    }
}