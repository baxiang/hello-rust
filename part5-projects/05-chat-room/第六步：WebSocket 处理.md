## 第六步：WebSocket 处理

### src/handlers/ws.rs

```rust
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, Path,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::{message::Message as ChatMessage, user::User};

/// WebSocket 升级请求
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, room_id))
}

/// 处理 WebSocket 连接
async fn handle_socket(socket: WebSocket, state: AppState, room_id: String) {
    // 分离发送和接收
    let (mut sender, mut receiver) = socket.split();
    
    // 创建发送通道
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    
    // 获取房间
    let room = state.get_or_create_room(&room_id);
    
    // 创建用户（临时用户名）
    let username = format!("User_{}", &Uuid::new_v4().to_string()[..8]);
    let user = User::new(username.clone(), tx.clone());
    let user_id = user.id;
    
    // 添加用户到房间
    room.add_user(user.clone());
    
    // 广播加入消息
    let join_msg = ChatMessage::Join {
        user_id,
        username: username.clone(),
        room_id: room_id.clone(),
    };
    room.broadcast(&join_msg.to_json());
    
    // 发送用户列表
    let user_list_msg = ChatMessage::UserList {
        room_id: room_id.clone(),
        users: room.get_users(),
    };
    let _ = tx.send(user_list_msg.to_json());
    
    // 启动发送任务
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv() {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    
    // 启动接收任务
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // 解析消息
                if let Some(chat_msg) = ChatMessage::from_json(&text) {
                    match chat_msg {
                        ChatMessage::Chat { content, .. } => {
                            // 创建聊天消息
                            let msg = ChatMessage::chat(
                                user_id,
                                username.clone(),
                                room_id.clone(),
                                content,
                            );
                            room.broadcast(&msg.to_json());
                        }
                        _ => {}
                    }
                }
            }
        }
    });
    
    // 等待任一任务结束
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
    
    // 用户离开
    room.remove_user(user_id);
    
    // 广播离开消息
    let leave_msg = ChatMessage::Leave {
        user_id,
        username,
        room_id,
    };
    room.broadcast(&leave_msg.to_json());
    
    // 更新用户列表
    let user_list_msg = ChatMessage::UserList {
        room_id: room_id.clone(),
        users: room.get_users(),
    };
    room.broadcast(&user_list_msg.to_json());
}
```

---

---

## 下一步

[第七步：房间管理](../第七步：房间管理.md)