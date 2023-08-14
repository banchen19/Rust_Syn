use serde::{Deserialize, Serialize};

use crate::var_config::def_Config::DefPlayer;

//添加玩家白名单
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AddPlayer
{
    pub(crate) name:String,
    pub(crate) pw:String,
    pub(crate) player:serde_json::Value,
}
// 所有玩家
#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub players: Vec<DefPlayer>,
}

//添加帖子
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AddForum
{
    pub(crate) name:String,
    pub(crate) pw:String,
    pub(crate) forum:serde_json::Value,
}

