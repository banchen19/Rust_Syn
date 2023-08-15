use colored::Colorize;
use mysql::Pool;
use rocket::yansi::Paint;
use rusqlite::{params, Connection, Error, Result};
use crate::{
    shttp::{http_player_config::{Players}, http_forum_config::{Forum, Forums}},
    var_config::{
        def_Config::{Config, DatabaseError, DefPlayer, EconomyInfo},
        yml_util::generate_random_key,
    },
};

use super::sqlite3_util::*;

//连接并创建数据库
pub async fn create_table(config: Config) {
    if config.sqlmode == "mysql" {
    } else {
        let _ = create_sqlite3();
        let def_money_name = config.clone().def_money_name;
        let mut key = generate_random_key(16);
        match add_money_name_sqlite3(config.clone().def_money_name) {
            Ok(_key) => {
                key = _key;
            }
            Err(_) => match getmoney_key_sqlite3(config.clone().def_money_name) {
                Ok(_key) => {
                    key = _key;
                }
                Err(_err) => {
                    println!("{}", Paint::red("密钥获取失败"));
                }
            },
        }
        println!("默认经济：{}", Paint::green(def_money_name));
        println!("默认经济密钥：{}", Paint::green(key));
    }
}

// 添加玩家
pub fn add_player(config: Config, player: DefPlayer) {
    if config.sqlmode == "mysql" {
    } else {
        insert_player_sqlite3(config, player);
    }
}

// 获取玩家密码
pub fn getplayer_pw(config: Config, name: String) -> Result<String, DatabaseError> {
    let mut pw: String = generate_random_key(16);
    if config.sqlmode == "mysql" {
    } else {
        match getplayer_pw_name_sqlite3(&name) {
            Ok(_pw) => {
                pw = _pw;
            }
            Err(_) => {
                println!("{}", Paint::red("密钥获取失败"));
            }
        }
    }
    Ok(pw)
}

// 获取玩家数据-默认经济体
pub fn getplayer_information(config: Config, name: String) -> Result<DefPlayer, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_name_sqlite3(&name, &config.def_money_name) {
            Ok(player) => {
                Ok(player)
            }
            Err(err) => Err(DatabaseError::SQLite(err)), // 封装为DatabaseError::SQLite
        }
    }
}

// 获取玩家数据-指定经济体
pub fn getplayer_information_money(
    config: Config,
    name: String,
    def_money_name: String,
) -> Result<DefPlayer, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_name_sqlite3(&name, &def_money_name) {
            Ok(player) => Ok(player),
            Err(err) => Err(DatabaseError::SQLite(err)), // 封装为DatabaseError::SQLite
        }
    }
}

//删除玩家
pub fn deleteplayer_me_sql(config: Config, name: String) -> Result<(), DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match delete_player_sqlite3_by_name(&name) {
            Ok(()) => Ok(()),
            Err(err) => Err(DatabaseError::SQLite(err)), // 封装为DatabaseError::SQLite
        }
    }
}

// 获取所有玩家数据——管理员版
pub fn get_player_all(
    config: Config,
    economy_name: String,
) -> Result<Option<Players>, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_all_sqlite3(economy_name) {
            Ok(players) => Ok(players),
            Err(_) => Ok(None),
        }
    }
}

// 获取所有玩家数据——乞丐版
pub fn get_player_all_pl(
    config: Config,
    economy_name: String,
) -> Result<Option<Players>, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_all_sqlite3_pl(economy_name) {
            Ok(players) => Ok(players),
            Err(_) => Ok(None),
        }
    }
}

//修改玩家权限
pub fn update_player_level(config: Config, name: String, level: i32) -> Result<(), DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match update_player_level_name_sqlite3(name, level) {
            Ok(()) => Ok(()),
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}

//删除经济体
pub fn deletemoney(config: Config, moneysName: String, key: String) -> Result<(), DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match delete_money_name_sqlite3(moneysName, key) {
            Ok(()) => Ok(()),
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}

//查询所有经济体
pub fn getmoney_name(config: Config)  -> Result<Vec<EconomyInfo>, DatabaseError>{
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getmoney_name_sqlite3() {
            Ok(moneys) => Ok(moneys),
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}

//查询所有经济体——pl
pub fn getmoney_name_pl(config: Config)  -> Result<Vec<EconomyInfo>, DatabaseError>{
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getmoney_name_sqlite3_pl() {
            Ok(moneys) => Ok(moneys),
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}

// 添加帖子
pub fn add_forum(config: Config, forum: Forum) {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        insert_forum_sqlite3(forum);
    }
}

// 添加帖子评论
pub fn add_forum_comment(config: Config, forum: Forum) {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        insert_forum_comment_sqlite3(forum);
    }
}

//查询所有帖子
pub fn getforumall(config: Config)  ->  Result<Option<Forums>, DatabaseError>{
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getforum_information_all_sqlite3() {
            Ok(forums) => {

            Ok(forums)},
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}

//查询所有帖子
pub fn getforumcommentall(config: Config,forum_title: String)  ->  Result<Option<Forums>, DatabaseError>{
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getforumcomment_information_all_sqlite3(forum_title) {
            Ok(forums) => {

            Ok(forums)},
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}