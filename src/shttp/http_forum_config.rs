


use serde::{Deserialize, Serialize};

use crate::var_config::def_Config::DefPlayer;

// 添加帖子
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Forum
{
    pub(crate) sender:String,
    pub(crate) title:String,
    pub(crate) text:String
}

// 标准返回帖子json结构
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Forumjson
{
    pub(crate) sender:String,
    pub(crate) title:String,
    pub(crate) text:String,
    pub(crate) time:String
}

// 所有帖子
#[derive(Debug, Serialize, Deserialize)]
pub struct Forums {
    pub(crate) forums: Vec<Forumjson>,
}
