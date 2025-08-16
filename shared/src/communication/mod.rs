use std::{fmt::format, str::{from_utf8, Utf8Error}, sync::Arc, time::Duration};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum CBCATcpPayloadType {
    Error, // 00
    Data,  // 01
    Debug, // 02
    Reqwest, // 03
    Unknown  // else
}

impl CBCATcpPayloadType {
    pub fn convert_str(action: &str) -> Self {
        match &action {
            &"00" => CBCATcpPayloadType::Error,
            &"01" => CBCATcpPayloadType::Data,
            &"02" => CBCATcpPayloadType::Debug,
            &"03" => CBCATcpPayloadType::Reqwest,
            &_ => CBCATcpPayloadType::Unknown
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            CBCATcpPayloadType::Error => true,
            _ => false
        }
    }

    pub fn is_data(&self) -> bool {
        match self {
            CBCATcpPayloadType::Data => true,
            _ => false
        }
    }

    pub fn is_debug(&self) -> bool {
        match self {
            CBCATcpPayloadType::Debug => true,
            _ => false
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            CBCATcpPayloadType::Unknown => true,
            _ => false
        }
    }

    pub fn is_reqwest(&self) -> bool {
        match self {
            CBCATcpPayloadType::Reqwest => true,
            _ => false
        }
    }
}

// type=[][] length=[][][][][][] length*[]
#[derive(Debug)]
pub struct CBCATcpPayload {
    payload_type: CBCATcpPayloadType,
    payload_size: u32,
    payload_content: String,
}

impl CBCATcpPayload {
    pub fn spawn(
        payload_type: CBCATcpPayloadType,
        payload_content: String
    ) -> Self {
        Self {
            payload_type,
            payload_size: payload_content.len() as u32,
            payload_content
        }
    }

    // type=[][] length=[][][][][][] length*[]
    pub fn decode_response(
        payload: &Vec<u8>
    ) -> Result<CBCATcpPayload, Utf8Error> {
        let action = from_utf8(&payload[0..2])?;
        let content = from_utf8(&payload[8..payload.len()])?; 
        
        Ok(
            Self::spawn(match &action {
                &"00" => CBCATcpPayloadType::Error,
                &"01" => CBCATcpPayloadType::Data,
                &"02" => CBCATcpPayloadType::Debug,
                &"03" => CBCATcpPayloadType::Reqwest,
                &_ => CBCATcpPayloadType::Unknown
            }, content.to_string())
        )
    }

    pub async fn read(
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>
    ) -> Result<String, Utf8Error> {
        let mut header: [u8;8] = [0u8;8];
        let mut lock = stream.lock().await;
        lock.read_exact(&mut header).await?;
        let action = &header[0..2];
        let p_size = &header[3..8];

        let content: Vec<u8> = Vec::new();

        println!("{:?} {:?} {:?}", header, action, p_size);

        Ok(format!(""))
    }

    pub async fn send(
        &self,
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>
    ) -> Result<(), std::io::Error> {
        let data: Vec<u8> = self.build_response();
        let mut lock = stream.lock().await;

        for chunks in data
            .chunks(8) {
                lock.writable().await?;
                lock.write_all(chunks).await?;
                lock.flush().await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
        }

        lock.shutdown().await?;

        Ok(())
    }

    pub fn build_response(
        &self
    ) -> Vec<u8> {
        let action: String = (match self.payload_type {
            CBCATcpPayloadType::Error => 0u8,
            CBCATcpPayloadType::Data => 1u8,
            CBCATcpPayloadType::Debug => 2u8,
            CBCATcpPayloadType::Reqwest => 3u8,
            CBCATcpPayloadType::Unknown => 4u8 
        }).to_string();

        let mut action_byted = action.as_bytes().to_vec();
        if action_byted.len() == 1 {
            action_byted.insert(0, "0".as_bytes()[0]);
        }
        
        let size_stringified = self.payload_size.to_string();
        let mut size_byted = size_stringified.as_bytes().to_vec();
        
        for _ in 0..(6-size_byted.len()) {
            size_byted.insert(0, "0".as_bytes()[0]);
        }
        
        let payload = self.payload_content.as_bytes().to_vec();
        let final_payload = [&action_byted[..], &size_byted[..], &payload[..]].concat();
        
        final_payload
    }
}