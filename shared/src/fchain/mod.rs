use crate::{block::CBCABlock, utils::hash_now};
use serde::{de::Error, Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, serde::Deserialize, Debug, Clone)]
pub struct CBCAConfig {
    limit_members: Option<u16>,
    private: bool,
    start_price: Option<f32>,
    duration: u32,
    description: String,
    name: String,
    pub hash: Option<String>,
    currency: String
}

impl CBCAConfig {
    pub fn spawn(
        limit_members: Option<u16>,
        private: bool,
        start_price: Option<f32>,
        duration: u32,
        description: String,
        name: String,
        currency: String
    ) -> Result<Self, serde_json::Error> {
        let mut config: Self = Self {
            limit_members,
            private,
            start_price,
            duration,
            description,
            name,
            hash: None,
            currency
        };

        let serialized: String = serde_json::to_string(&config)?;
        config.hash = Some(hash_now(serialized));

        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CBCAChain {
    instance_id: String,
    chain: Vec<CBCABlock>,
    pub hash: Option<String>,
}

impl CBCAChain {
    pub fn spawn(
        instance_id: String,
    ) -> Self {
        Self {
            instance_id,
            chain: Vec::new(),
            hash: None
        }
    }

    pub fn verify(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn hash_chain(
        &mut self
    ) -> Result<String, serde_json::Error> {
        let serialized = to_string(&self)?;
        let hash = hash_now(serialized);
        self.hash = Some(hash.clone());
        Ok(hash)
    }

    pub fn get_last_hash(&self) -> Option<String> {
        let last: &CBCABlock = self.chain.iter().last()?;
        last.get_hash()
    }

    pub fn push(
        &mut self, 
        block: CBCABlock
    ) -> Result<String, serde_json::Error> {
        let mut block_copy: CBCABlock = block;
        block_copy.previous_hash = self.get_last_hash();
        block_copy.hash_block()?;
        self.chain.push(block_copy.clone());
        self.hash_chain()?;

        match block_copy.get_hash() {
            Some(v) => Ok(v),
            None => Err(serde_json::Error::missing_field("block hash is missing.")),
        }
    }

    pub fn display(&self) -> () {
        println!("{:?}", self);
    } 
}