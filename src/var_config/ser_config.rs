use serde::{Serialize, Deserialize};

use crate::var_config::Config_A::Player;
//放置结构体


#[derive(Debug, Serialize, Deserialize)]
pub struct WsData {
    pub(crate)key:String,
    pub(crate)typestr:String,
    pub(crate)data:serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SerToData {
    pub(crate)player_name: String,
    pub(crate)perm_level: u8,
    pub(crate)data: String,
    pub(crate)serverver_name: String,
    
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SerToChatData {
    pub(crate)typestr: String,
    pub(crate)serverver_name: String,
    pub(crate)data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct DataResponse {
    pub(crate)code: String,
    pub(crate)msg: String,
}

#[derive(Debug, Serialize)]
pub struct JsonResponse {
    pub(crate)data: DataResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionMg {
    typestr:String,
    perm_name:String
}

#[derive(Debug, Deserialize)]
pub struct RootData {
    pub(crate)key: String,
    pub(crate)t: String,
    pub(crate)token: String,
}


