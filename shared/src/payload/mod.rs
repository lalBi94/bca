use serde::{Serialize, Deserialize};
use std::any::{Any, TypeId};

use crate::fchain::CBCAConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MPayload {
    pub content: String,
    pub author: String,
    pub instance_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OPayload {
    pub amount: f32,
    pub author: String,
    pub instance_id: String,
    pub message: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IPayload {
    pub instance_id: String,
    pub config: CBCAConfig
}

pub trait Payload {
    fn get_payload(&self) -> Self;
    fn get_instance_id(&self) -> &str;
    fn is_instance_of<T: 'static + Payload + ?Sized>(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl Payload for MPayload {
    fn get_payload(&self) -> Self {
        self.clone()
    }
    
    fn get_instance_id(&self) -> &str {
        &self.instance_id
    }

    fn is_instance_of<T: 'static + Payload + ?Sized>(&self) -> bool
    where
        Self: 'static + Any,
    {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl MPayload {
    pub fn extract(payload: &MPayload) -> (String, String, String) {
        (payload.content.to_string(), payload.author.to_string(), payload.instance_id.to_string())
    }
}

impl Payload for OPayload {
    fn get_payload(&self) -> Self {
        self.clone()
    }

    fn get_instance_id(&self) -> &str {
        &self.instance_id
    }

    fn is_instance_of<T: 'static + Payload + ?Sized>(&self) -> bool
    where
        Self: 'static + Any,
    {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl Payload for IPayload {
    fn get_payload(&self) -> Self {
        self.clone()
    }
    
    fn get_instance_id(&self) -> &str {
        &self.instance_id
    }

    fn is_instance_of<T: 'static + Payload + ?Sized>(&self) -> bool
    where
        Self: 'static + Any,
    {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl IPayload {
    pub fn extract_config(&self) -> &CBCAConfig {
        &self.config
    }
}