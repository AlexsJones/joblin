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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_add_message_request_serde() {
        let req = AddMessageRequest { job: "echo hello".to_string() };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{\"job\":\"echo hello\"}");
        let de: AddMessageRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(de.job, "echo hello");
    }

    #[test]
    fn test_add_message_request_from_value() {
        let value = serde_json::json!({"job": "ls -l"});
        let req = AddMessageRequest::from_value(value);
        assert_eq!(req.job, "ls -l");
    }
}