use colored::Colorize;
use mysql::Pool;
use rocket::yansi::Paint;

use crate::{
    shttp::http_player_config::{Player, Players},
    var_config::{
        def_Config::{Config, DatabaseError, DefPlayer},
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
            Err(_) => match getplayer_pw_name_sqlite3(&config.clone().def_money_name) {
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
        insert_player_sqlite3(config,player);
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
            },
        }
    }
    Ok(pw)
}

// 获取玩家数据-默认经济体
pub fn getplayer_information(config: Config, name: String) -> Result<Player, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_name_sqlite3(&name,&config.def_money_name) {
            Ok(player) => Ok(player),
            Err(err) => Err(DatabaseError::SQLite(err)), // 封装为DatabaseError::SQLite
        }
    }
}

// 获取玩家数据-指定经济体
pub fn getplayer_information_money(config: Config, name: String,def_money_name:String) -> Result<Player, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_name_sqlite3(&name,&def_money_name) {
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

// 获取所有玩家数据
pub fn get_player_all(config: Config,economy_name:String) -> Result<Option<Players>, DatabaseError> {
    if config.sqlmode == "mysql" {
        unimplemented!()
    } else {
        match getplayer_information_all_sqlite3(economy_name) {
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
        match delete_money_name_sqlite3(moneysName,key) {
            Ok(()) => Ok(()),
            Err(err) => Err(DatabaseError::SQLite(err)),
        }
    }
}
