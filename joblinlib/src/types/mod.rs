use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct AddMessageRequest {
    pub job: String,
}
impl AddMessageRequest {
    pub fn from_value(value: serde_json::Value) -> AddMessageRequest {
        serde_json::from_value(value).unwrap()
    }
}
#[derive(Serialize, Deserialize)]
pub struct ListMessageRequest {

}
#[derive(Serialize, Deserialize)]
pub struct ListMessageResponse {
    job: Vec<String>
}