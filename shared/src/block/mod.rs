use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::{
    payload::{MPayload, OPayload, IPayload, Payload}, 
    utils::hash_now
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CBCABlockType {
    MESSAGE(MPayload),
    OFFER(OPayload),
    INSTANCE(IPayload)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CBCABlock {
    hash: Option<String>,
    payload: CBCABlockType,
    timestamp: i64,
    pub previous_hash: Option<String>
}

impl CBCABlock {
    pub fn spawn(
        payload: CBCABlockType, 
        timestamp: i64, 
        previous_hash: Option<String>
    ) -> Self {
        Self {
            hash: None,
            payload,
            timestamp,
            previous_hash
        }
    }
    
    pub fn get_hash(&self) -> Option<String> {
        self.hash.clone()
    }

    pub fn get_instance_id(&self) -> Option<String> {
        match &self.payload {
            CBCABlockType::MESSAGE(mpayload) => Some(mpayload.get_instance_id().to_string()),
            CBCABlockType::OFFER(opayload) => Some(opayload.get_instance_id().to_string()),
            _ => None
        }
    }

    fn get_payload(&self) -> CBCABlockType {
        self.payload.clone()
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_previous_hash(&self) -> Option<String> {
        self.previous_hash.clone()
    }

    pub fn hash_block(
        &mut self,
    ) -> Result<String, serde_json::Error> {
        let serialized_block: String = serde_json::to_string(&self)?;
        let hashed: String = hash_now(serialized_block);
        self.hash = Some(hashed.clone());
        Ok(hashed)
    }

    pub fn block_creator_offer(
        amount: f32, 
        author: String,
        instance_id: String,
        message: Option<String>
    ) -> CBCABlock {
        let block: CBCABlock = CBCABlock::spawn(
            CBCABlockType::OFFER(
                OPayload {
                    amount,
                    author,
                    instance_id,
                    message
                }
            ),
            Utc::now().timestamp(),
            None
        );

        block
    }

    pub fn block_creator_message(
        content: String, 
        author: String,
        instance_id: String,
    ) -> CBCABlock {
        let block: CBCABlock = CBCABlock::spawn(
            CBCABlockType::MESSAGE(
                MPayload {
                    content,
                    author,
                    instance_id
                }
            ),
            Utc::now().timestamp(),
            None
        );

        block
    }

}