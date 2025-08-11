use shared::{
    block::{CBCABlock}, 
    fchain::{CBCAChain, CBCAConfig}
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CBCAInstance {
    pub identifier: String,
    pub offers_chain: CBCAChain,
    pub messages_chain: CBCAChain,
    pub config: CBCAConfig,
    pub started: bool
}

impl CBCAInstance {
    pub fn spawn(
        config: CBCAConfig,
        identifier: &str 
    ) -> Self {
        Self {
            identifier: identifier.to_string(),
            offers_chain: CBCAChain::spawn(identifier.to_string()),
            messages_chain: CBCAChain::spawn(identifier.to_string()),
            config,
            started: false
        }
    }

    pub fn add_message(
        &mut self, 
        content: String, 
        author: String
    ) -> Result<String, serde_json::Error> {
        self.messages_chain.push(
            CBCABlock::block_creator_message(content, author, self.identifier.clone())
        )
    }

    pub fn add_offer(
        &mut self, 
        amount: f32, 
        author: String, 
        message: Option<String>
    ) -> Result<String, serde_json::Error> {
        self.offers_chain.push(
            CBCABlock::block_creator_offer(amount, author, self.identifier.clone(), message)
        )
    }

    pub fn display(&self) -> () {
        println!("{:?}", self);
    }
}