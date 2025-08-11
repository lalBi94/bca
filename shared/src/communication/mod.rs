#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CBCAResponse {
    error: Option<String>,
    data: Option<String>
}

impl CBCAResponse {
    pub fn spawn(
        error: Option<String>,
        data: Option<String>
    ) -> Self {
        Self { error, data }
    }
    
    pub fn serialized_response(    
        &self
    ) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn deserializer(
        raw_response: String
    ) -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_str(&raw_response)?)
    }
}