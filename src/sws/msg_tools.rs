use super::ws_config::{WsChat, SendMsg};


// Define an enum to represent the three possible states
pub enum MessageResult {
    Success(Vec<u8>,bool),
    UnknownType,
}

// 消息接受，类型处理
pub fn message_amin(text:String)-> MessageResult{
    match serde_json::from_str::<WsChat>(&text.clone()) {
        Ok(ws_chat) => {
            // 处理消息类型
            match ws_chat.r#type.as_str() {
                "chat" => 
                {
                    if let Ok(send_msg) = serde_json::from_value::<SendMsg>(ws_chat.data.clone()) {
                       let json_string: String = serde_json::to_string(&send_msg).unwrap(); //消息格式化f转发
                        MessageResult::Success(rebyte(json_string),true)
                    } else {
                        MessageResult::UnknownType
                    }
                    
                }
                "pl_join" => MessageResult::Success(rebyte("".to_owned()),false),
                "pl_update" => MessageResult::Success(rebyte("".to_owned()),true),
                _ => MessageResult::UnknownType,
            }
        },
        Err(err) => MessageResult::UnknownType,
    } 
}

fn rebyte(str:String)->Vec<u8>{
    str.as_bytes().to_vec()
}