use std::{fmt::format, str::{from_utf8, Utf8Error}, sync::Arc, time::Duration};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum CBCATcpError {
    InvalidHeader(String)
}

#[derive(Debug)]
pub enum CBCATcpPayloadType {
    Error, // 00
    Data,  // 01
    Debug, // 02
    Reqwest, // 03
    Unknown  // else
}

impl CBCATcpPayloadType {
    pub fn from_str(action: &str) -> Self {
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

impl PartialEq for CBCATcpPayloadType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
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
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>,
        read_type: CBCATcpPayloadType
    ) -> Result<String, CBCATcpError> {
        let mut header: [u8;8] = [0u8;8];
        let mut lock = stream.lock().await;
        let _ = lock.read_exact(&mut header).await;

        let action_raw: Result<&str, Utf8Error> = from_utf8(&header[0..2]);
        let p_size_raw: Result<&str, Utf8Error> = from_utf8(&header[3..8]);

        if let Err(_) = action_raw {
            return Err(CBCATcpError::InvalidHeader(format!("invalid header, no action.")));
        }

        if let Err(_) = p_size_raw {
            return Err(CBCATcpError::InvalidHeader(format!("invalid header, no action.")));
        }

        let action: CBCATcpPayloadType = CBCATcpPayloadType::from_str(action_raw.unwrap());

        if !action.eq(&read_type) && !action.is_error() {
            return Err(CBCATcpError::InvalidHeader(format!("invalid header, incorrect action.")))
        }

        let p_size = usize::from_str_radix(p_size_raw.unwrap(), 10);

        if let Err(_) = p_size {
            return Err(CBCATcpError::InvalidHeader(format!("invalid header, payload size isn't a number.")));
        }
        
        let mut content: Vec<u8> = Vec::new();
        
        println!("action={:?} {:?}", action, action_raw);
        println!("size={:?} {:?}", p_size, p_size_raw);

        let p_parsed_size_chunks: usize = f64::ceil(p_size.unwrap() as f64/8.0) as usize;

        for i in 0..p_parsed_size_chunks {
            let mut temp_data: [u8; 8] = [0u8;8];
            let _ = lock.read_exact(&mut temp_data).await;

            let _ = temp_data.iter().for_each(|v| {
                if v != &0u8 {
                    content.push(*v);
                }
            });

            println!("{i} {:?}", temp_data);
        }

        let stringify = from_utf8(content.as_slice());
        
        match stringify {
            Ok(v) => Ok(v.to_string()),
            Err(_) => Err(CBCATcpError::InvalidHeader(format!("payload unreadable."))),
        }
    }

    pub async fn send(
        &self,
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>
    ) -> Result<(), std::io::Error> {
        let data: Vec<u8> = self.build_response();
        let mut lock = stream.lock().await;

        for chunk in data
            .chunks(8) {
                let mut buf = [0u8; 8]; 
                buf[..chunk.len()].copy_from_slice(chunk);
                lock.writable().await?;
                lock.write_all(&buf).await?;
                lock.flush().await?;
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