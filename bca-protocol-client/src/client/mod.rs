use std::{sync::Arc, time::Duration};
use shared::{
    communication::CBCATcpError, fchain::CBCAConfig, payload::{IPayload, MPayload, OPayload}
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, 
    time::sleep
};
use shared::communication::{CBCATcpPayloadType, CBCATcpPayload};

pub struct CBCAClient { 
    ip_message: String,
    ip_instance: String,
    ip_offer: String
}

pub enum CBCAFlag {
    IPM, IPI, IPO
}

impl CBCAClient {
    pub fn spawn(
        ip_message: String,
        ip_instance: String,
        ip_offer: String
    ) -> Self {
        Self {
            ip_message,
            ip_instance,
            ip_offer
        }
    }
    
    async fn fetch(
        &self, 
        flag: CBCAFlag, 
        payload: String
    ) -> Result<String, std::io::Error> {
        let stream: tokio::net::TcpStream =
            tokio::net::TcpStream::connect(
                match flag {
                    CBCAFlag::IPM => &self.ip_message,
                    CBCAFlag::IPI => &self.ip_instance,
                    CBCAFlag::IPO => &self.ip_offer,
                }
            ).await?;

        let shared_stream = Arc::new(tokio::sync::Mutex::new(stream));
        let shared_stream_scope = Arc::clone(&shared_stream);

        let reqwest = CBCATcpPayload::spawn(CBCATcpPayloadType::Reqwest, payload);
        reqwest.send(shared_stream).await?;

        let result = CBCATcpPayload::read(shared_stream_scope, CBCATcpPayloadType::Data).await;

        match result {
            Ok(v) => {Ok(v)},
            Err(v) => {
                match v {
                    CBCATcpError::InvalidHeader(v) => {
                        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, v))
                    }
                }
            }
        }
    }

    pub async fn send_message(
        &self,
        author: String, 
        content: String, 
        identifier: String
    ) -> Result<String, std::io::Error> {
        let payload: MPayload = MPayload { 
            content, 
            author, 
            instance_id: identifier
        };

        let serialized: String = serde_json::to_string(&payload)?;
        let res: String = self.fetch(CBCAFlag::IPM, serialized).await?;
        
        Ok(res)
    }

    pub async fn send_offer(
        &self,
        amount: f32, 
        message: Option<String>, 
        identifier: String,
        author: String
    ) -> Result<String, std::io::Error> {
        let payload: OPayload = OPayload { 
            amount,
            author, 
            message,
            instance_id: identifier
        };

        let serialized: String = serde_json::to_string(&payload)?;
        let res: String = self.fetch(CBCAFlag::IPO, serialized).await?;
        
        Ok(res)
    }

    pub async fn send_instance(
        &self,
        config: CBCAConfig
    ) -> Result<String, std::io::Error> {
        let payload: IPayload = IPayload { 
            instance_id: uuid::Uuid::new_v4().to_string(), 
            config:  config
        };

        let serialized: String = serde_json::to_string(&payload)?;
        let res: String = self.fetch(CBCAFlag::IPI, serialized).await?;

        Ok(res)
    }
}