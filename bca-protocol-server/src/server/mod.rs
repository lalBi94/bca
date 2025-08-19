use std::str::from_utf8;
use std::time::Duration;
use std::{sync::Arc};
use std::io::Error;
use shared::communication::{self, CBCATcpPayload, CBCATcpPayloadType};
use shared::payload::{IPayload, MPayload, OPayload};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpSocket};

use crate::queue::CBCAQueue;

pub struct CBCAServer {
    addr_instance: CBCARoutineAddr,
    addr_offer: CBCARoutineAddr,
    addr_message: CBCARoutineAddr,
    shared_queue: CBCAQueue
}

pub struct CBCARoutineAddr {
    ip: String,
    port: String
}

impl CBCARoutineAddr {
    pub fn spawn(ip: String, port: String) -> Self {
        Self { ip, port }
    }

    pub fn get_full_addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl CBCAServer {
    pub fn spawn(
        addr_message: CBCARoutineAddr,
        addr_instance: CBCARoutineAddr,
        addr_offer: CBCARoutineAddr
    ) -> Result<Self, std::io::Error> {
        Ok(
            Self {
                addr_instance,
                addr_offer,
                addr_message,
                shared_queue: CBCAQueue::spawn()?
            }
        )
    }

    pub async fn run_routines(
        &self
    ) -> Result<(), std::io::Error> {
        let _ = tokio::join!(
            biased;
            self.routine_instance(),
            self.routine_message(),
            self.routine_offer(),
            self.shared_queue.routine()
        );

        Ok(())
    }

    pub async fn handle_instance(
        &self, 
        raw_payload: String,
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>
    ) -> Result<(), std::io::Error> {
        let instance: IPayload = serde_json::from_str(&raw_payload)?;
        let identifier: Result<String, std::io::Error>= self.shared_queue.handle_add_instance(instance).await;
        
        let response: CBCATcpPayload = CBCATcpPayload::spawn(
            if let Ok(_) = identifier { 
                CBCATcpPayloadType::Data
            } else { 
                CBCATcpPayloadType::Error
            }, 
            if let Ok(v) = identifier { 
                v 
            } else { 
                "queue doesnt push an instance.".to_string() 
            }
        );

        let lock_for_res: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = Arc::clone(&stream);
        response.send(lock_for_res).await?;

        Ok(())
    }

    pub async fn handle_message(
        &self, 
        raw_payload: String,
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>
    ) -> Result<(), std::io::Error> {
        let message: MPayload = serde_json::from_str(&raw_payload)?;
        let pushing = self.shared_queue.handle_add_message(message).await;

        let response: CBCATcpPayload = CBCATcpPayload::spawn(
            if let Ok(_) = pushing { 
                CBCATcpPayloadType::Data
            } else { 
                CBCATcpPayloadType::Error
            }, 
            if let Ok(_) = pushing { 
                "true".to_string()
            } else { 
                "false".to_string()
            }
        );

        let lock_for_res: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = Arc::clone(&stream);
        response.send(lock_for_res).await?;

        Ok(())
    }

    pub async fn handle_offer(
        &self, 
        raw_payload: String,
        stream: Arc<tokio::sync::Mutex<tokio::net::TcpStream>>
    ) -> Result<(), std::io::Error> {
        let offer: OPayload = serde_json::from_str(&raw_payload)?;
        let pushing: Result<(), std::io::Error> = self.shared_queue.handle_add_offer(offer).await;

        let response: CBCATcpPayload = CBCATcpPayload::spawn(
            if let Ok(_) = pushing { 
                CBCATcpPayloadType::Data
            } else { 
                CBCATcpPayloadType::Error
            }, 
            if let Ok(_) = pushing { 
                "true".to_string()
            } else { 
                "false".to_string()
            }
        );

        let lock_for_res: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = Arc::clone(&stream);
        response.send(lock_for_res).await?;
        Ok(())
    }

    pub async fn routine_instance(
        &self
    ) -> Result<(), std::io::Error> {
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.addr_instance.get_full_addr()).await?;
        println!("[INSTANCE] on {}.", self.addr_instance.get_full_addr());

        loop {
            let (socket, _) = listener.accept().await?;
            let shared_stream_original = Arc::new(
                tokio::sync::Mutex::new(socket)
            );

            let shared_stream_response: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = 
                Arc::clone(&shared_stream_original);
            let shared_stream_listener: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> =  
                Arc::clone(&shared_stream_original);

            let req: Result<String, communication::CBCATcpError> = 
                CBCATcpPayload::read(shared_stream_listener, CBCATcpPayloadType::Reqwest).await;

            if req.is_ok() {
                self.handle_instance(req.unwrap().to_string(), shared_stream_response).await?;
                println!("instance ok.");
            } else {
                println!("instance error.");
            }
        }
    }

    pub async fn routine_offer(
        &self
    ) -> Result<(), std::io::Error> {
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.addr_offer.get_full_addr()).await?;
        println!("[OFFER] on {}.", self.addr_offer.get_full_addr());

        loop {
            let (socket, _) = listener.accept().await?;
            let shared_stream_original = Arc::new(
                tokio::sync::Mutex::new(socket)
            );

            let shared_stream_response: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = 
                Arc::clone(&shared_stream_original);
            let shared_stream_listener: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = 
                Arc::clone(&shared_stream_original);

            let req: Result<String, communication::CBCATcpError> = 
                CBCATcpPayload::read(shared_stream_listener, CBCATcpPayloadType::Reqwest).await;

            if req.is_ok() {
                self.handle_offer(req.unwrap(), shared_stream_response).await?;
                println!("offer ok.");
            } else {
                println!("offer error.");
            }
        }
    }

    pub async fn routine_message(&self) -> Result<(), std::io::Error> {
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.addr_message.get_full_addr()).await?;
        println!("[MESSAGE] on {}.", self.addr_message.get_full_addr());

        loop {
            let (socket, _) = listener.accept().await?;
            let shared_stream_original = Arc::new(
                tokio::sync::Mutex::new(socket)
            );

            let shared_stream_response: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = 
                Arc::clone(&shared_stream_original);
            let shared_stream_listener: Arc<tokio::sync::Mutex<tokio::net::TcpStream>> = 
                Arc::clone(&shared_stream_original);

            let req: Result<String, communication::CBCATcpError> = 
                CBCATcpPayload::read(shared_stream_listener, CBCATcpPayloadType::Reqwest).await;

            if req.is_ok() {
                self.handle_message(req.unwrap(), shared_stream_response).await?;
                println!("message ok.");
            } else {
                println!("message error.");
            }
        }
    }
}