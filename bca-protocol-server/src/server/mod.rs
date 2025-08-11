use std::{sync::Arc};
use std::io::Error;
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
        stream: &mut tokio::net::TcpStream
    ) -> Result<(), std::io::Error> {
        println!("ok");
        let instance: IPayload = serde_json::from_str(&raw_payload)?;
        let identifier: String = self.shared_queue.handle_add_instance(instance).await?;
        stream.writable().await?;
        println!("{:?}", stream.write_all(identifier.as_bytes()).await);
        Ok(())
    }

    pub async fn handle_message(
        &self, 
        raw_payload: String
    ) -> Result<(), std::io::Error> {
        let message: MPayload = serde_json::from_str(&raw_payload)?;
        self.shared_queue.handle_add_message(message).await?;
        Ok(())
    }

    pub async fn handle_offer(
        &self, 
        raw_payload: String
    ) -> Result<(), std::io::Error> {
        let offer: OPayload = serde_json::from_str(&raw_payload)?;
        self.shared_queue.handle_add_offer(offer).await?;
        Ok(())
    }

    pub async fn routine_instance(
        &self
    ) -> Result<(), std::io::Error> {
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.addr_instance.get_full_addr()).await?;
        println!("[INSTANCE] on {}.", self.addr_instance.get_full_addr());

        loop {
            let (mut socket, _) = listener.accept().await?;
            
            let mut payload_raw: String = String::new();
            socket.readable().await?;
            socket.read_to_string(&mut payload_raw).await?;

            println!("[INSTANCE] {}", payload_raw);
            self.handle_instance(payload_raw, &mut socket).await?;

            tokio::task::yield_now().await;
        }
    }

    pub async fn routine_offer(
        &self
    ) -> Result<(), std::io::Error> {
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.addr_offer.get_full_addr()).await?;
        println!("[OFFER] on {}.", self.addr_offer.get_full_addr());

        loop {
            let (mut socket, _) = listener.accept().await?;
            
            let mut payload_raw: String = String::new();
            socket.read_to_string(&mut payload_raw).await?;
            
            println!("[OFFER] {}", payload_raw);
            self.handle_offer(payload_raw).await?;

            tokio::task::yield_now().await;
        }
    }

    pub async fn routine_message(&self) -> Result<(), std::io::Error> {
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.addr_message.get_full_addr()).await?;
        println!("[MESSAGE] on {}.", self.addr_message.get_full_addr());

        loop {
            let (mut socket, _) = listener.accept().await?;
            
            let mut payload_raw: String = String::new();
            socket.read_to_string(&mut payload_raw).await?;
            
            println!("[MESSAGE] {}", payload_raw);
            self.handle_message(payload_raw).await?;

            tokio::task::yield_now().await;
        }
    }
}