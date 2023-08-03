use serde::{Deserialize, Serialize};

use crate::var_config::def_Config::DefPlayer;

//添加玩家白名单
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AddPlayer
{
    pub(crate) name:String,
    pub(crate) pw:String,
    pub(crate) player:DefPlayer,
}


// 玩家默认数据
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Player {
    pub player: DefPlayer,
    pub money: i32,
}

// 所有玩家
#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub players: Vec<Player>,
}