use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct WsChat {
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