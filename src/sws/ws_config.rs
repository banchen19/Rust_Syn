use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct WsChat {
    #[serde(rename = "type")] // 将字段重命名为"type"作为JSON键
    pub(crate)r#type:String,
    pub(crate)data:serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMsg
{
    pub(crate)name:String,
    pub(crate)prefix:String,
    pub(crate)msg:String,
}