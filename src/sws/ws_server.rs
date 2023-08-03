use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use rocket::yansi::Paint;
use serde_json;
use ws::{Handler, Handshake, Message, Result};

use crate::sws::ws_key::{decrypt, encrypt};

pub(crate) struct ServerHandler {
    pub(crate) out: ws::Sender,
    pub(crate) connections: Arc<Mutex<Vec<(ws::Sender, SystemTime)>>>,
}
impl Handler for ServerHandler {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        // 获取客户端地址
        let client_addr = shake.peer_addr;
        println!("{}:  {}",Paint::yellow("通信端接受新连接,来自"), Paint::green(client_addr.unwrap()));
        let mut connections = self.connections.lock().unwrap();
        let current_time = SystemTime::now();
        connections.push((self.out.clone(), current_time));
        println!("{}:  {}",Paint::yellow("当前已接受连接数量："),Paint::green(connections.len()));
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => {
                let plain = text.into_bytes();
                msgdecrypt(plain, self)
            }
            Message::Binary(bytes) => {
                let plain = bytes;
                msgdecrypt(plain, self)
            }
        }

        // 在收到客户端的消息时，更新连接的时间戳
        let mut connections = self.connections.lock().unwrap();
        let current_time = SystemTime::now();
        // 查找并更新对应的连接时间戳
        for (sender, time) in connections.iter_mut() {
            if sender == &self.out {
                *time = current_time;
                break;
            }
        }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        let mut connections: std::sync::MutexGuard<'_, Vec<(ws::Sender, SystemTime)>> = self.connections.lock().unwrap();
        connections.retain(|sender: &(ws::Sender, SystemTime)| {
            sender.0.connection_id() != self.out.connection_id()
        });
        println!("{}: {}",Paint::yellow("检测到通信客户端连接断开,更新连接数"), Paint::green(connections.len()));
    }
}

fn msgdecrypt(plain: Vec<u8>, _server_handler: &mut ServerHandler) {
    let separator = "*".repeat(40);

    println!("明文：{:?}", plain);
    // Convert plaintext to a String
    let plaintext_string = String::from_utf8(plain.to_vec()).unwrap();
    println!("明文字符串：{}", plaintext_string);

    let (ct, iv) = encrypt(&plain);
    println!(
        "{}\n密文：{:?}\n初始化向量：{:?}\n{}",
        separator, ct, iv, separator
    );

    to_send_chat_bds(_server_handler, ct.clone());
    let pt = decrypt(&ct, iv);
    println!("解密结果：{:?}", pt);

    // Convert decrypted result to a String
    let decrypted_string = String::from_utf8(pt.to_vec()).unwrap();
    println!("解密结果字符串：{}", decrypted_string);
}



// 发送消息：为二进制
fn to_send_chat_bds(_server_handler: &mut ServerHandler, data: Vec<u8>) {
    let binding = _server_handler.connections.clone();
    let connections_arr: std::sync::MutexGuard<'_, Vec<(ws::Sender, SystemTime)>> =
        binding.lock().unwrap();

    for sender in connections_arr.iter() {
        sender
            .0
            .send(Message::Binary(data.clone()))
            .expect("Failed to send data");
        // if _server_handler.out != sender.0 {
        //     println!("{:?}",json_string.clone());
        //     if let Err(err) = sender.0.send(json_string.clone()) {
        //         println!("玩家发送消息发送消息失败: {:?}", err);
        //         let _ = sender.0.close(ws::CloseCode::Other(404));
        //     }
        // };
        // 获取当前时间
        let now = SystemTime::now();
        // 检查是否超过10分钟
        if now
            .duration_since(sender.1)
            .unwrap_or(Duration::new(0, 0))
            .as_secs()
            >= 600
        {
            println!("超过10分钟，断开连接");
            let _ = sender.0.close(ws::CloseCode::Other(404));
        }
    }
}
