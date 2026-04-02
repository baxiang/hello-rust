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






## 第七步：房间管理

### src/handlers/room.rs

```rust
use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

/// 创建房间请求
#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
}

/// 房间信息响应
#[derive(Debug, Serialize)]
pub struct RoomInfo {
    pub id: String,
    pub name: String,
    pub user_count: usize,
}

/// 获取房间列表
pub async fn list_rooms(
    State(state): State<AppState>,
) -> Json<Vec<RoomInfo>> {
    let rooms = state.get_room_list();
    
    Json(rooms
        .into_iter()
        .map(|(id, name, count)| RoomInfo {
            id,
            name,
            user_count: count,
        })
        .collect())
}

/// 创建房间
pub async fn create_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomRequest>,
) -> Json<RoomInfo> {
    let room_id = payload.name.to_lowercase().replace(" ", "-");
    let room = state.get_or_create_room(&room_id);
    
    Json(RoomInfo {
        id: room.id,
        name: room.name,
        user_count: 0,
    })
}
```






## 第八步：主入口

### src/main.rs

```rust
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;

mod models;
mod handlers;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 创建应用状态
    let state = AppState::new();
    
    // 构建路由
    let app = Router::new()
        // WebSocket 路由
        .route("/ws/:room_id", get(handlers::ws::ws_handler))
        
        // 房间管理
        .route("/api/rooms", get(handlers::room::list_rooms))
        .route("/api/rooms", post(handlers::room::create_room))
        
        // 静态文件
        .nest_service("/", ServeDir::new("static"))
        
        // CORS
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        
        // 状态
        .with_state(state);
    
    // 启动服务器
    let addr = "127.0.0.1:3000";
    tracing::info!("服务器启动: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```






## 第九步：前端页面

### static/index.html

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>聊天室</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: Arial, sans-serif; background: #f0f0f0; }
        
        .container { max-width: 800px; margin: 20px auto; }
        
        .header { background: #333; color: white; padding: 15px; }
        
        .sidebar { float: left; width: 200px; background: white; padding: 10px; }
        .sidebar h3 { margin-bottom: 10px; }
        .room-item { padding: 8px; cursor: pointer; border-bottom: 1px solid #eee; }
        .room-item:hover { background: #f5f5f5; }
        .room-item.active { background: #e0e0e0; }
        
        .chat-area { float: right; width: 580px; background: white; }
        
        .messages { height: 400px; overflow-y: auto; padding: 10px; border-bottom: 1px solid #eee; }
        .message { margin-bottom: 10px; }
        .message .user { color: #666; font-size: 12px; }
        .message .content { margin-top: 5px; }
        .message.system { color: #999; text-align: center; }
        
        .input-area { padding: 10px; }
        .input-area input { width: 450px; padding: 10px; border: 1px solid #ddd; }
        .input-area button { padding: 10px 20px; background: #333; color: white; border: none; cursor: pointer; }
        
        .users { padding: 10px; border-top: 1px solid #eee; }
        .user-badge { display: inline-block; padding: 3px 8px; margin: 2px; background: #e0e0e0; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Rust 聊天室</h1>
        </div>
        
        <div class="sidebar">
            <h3>房间列表</h3>
            <div id="rooms"></div>
            
            <h3 style="margin-top: 20px;">在线用户</h3>
            <div id="users"></div>
        </div>
        
        <div class="chat-area">
            <div class="messages" id="messages"></div>
            <div class="input-area">
                <input type="text" id="message-input" placeholder="输入消息...">
                <button onclick="sendMessage()">发送</button>
            </div>
        </div>
    </div>
    
    <script>
        let ws = null;
        let currentRoom = 'general';
        let username = 'User_' + Math.random().toString(36).substr(2, 8);
        
        function connect(roomId) {
            ws = new WebSocket(`ws://${location.host}/ws/${roomId}`);
            
            ws.onmessage = (event) => {
                const msg = JSON.parse(event.data);
                handleMessage(msg);
            };
            
            ws.onclose = () => {
                addMessage('system', '连接已断开');
            };
        }
        
        function handleMessage(msg) {
            switch (msg.type) {
                case 'Join':
                    addMessage('system', `${msg.username} 加入房间`);
                    break;
                case 'Leave':
                    addMessage('system', `${msg.username} 离开房间`);
                    break;
                case 'Chat':
                    addMessage(msg.username, msg.content);
                    break;
                case 'System':
                    addMessage('system', msg.content);
                    break;
                case 'UserList':
                    updateUserList(msg.users);
                    break;
            }
        }
        
        function addMessage(user, content) {
            const div = document.createElement('div');
            div.className = 'message' + (user === 'system' ? ' system' : '');
            div.innerHTML = `<div class="user">${user}</div><div class="content">${content}</div>`;
            document.getElementById('messages').appendChild(div);
            document.getElementById('messages').scrollTop = document.getElementById('messages').scrollHeight;
        }
        
        function updateUserList(users) {
            const div = document.getElementById('users');
            div.innerHTML = users.map(u => `<span class="user-badge">${u.username}</span>`).join('');
        }
        
        function sendMessage() {
            const input = document.getElementById('message-input');
            const content = input.value.trim();
            
            if (content && ws) {
                ws.send(JSON.stringify({
                    type: 'Chat',
                    content: content
                }));
                input.value = '';
            }
        }
        
        function loadRooms() {
            fetch('/api/rooms')
                .then(r => r.json())
                .then(rooms => {
                    const div = document.getElementById('rooms');
                    div.innerHTML = rooms.map(r => 
                        `<div class="room-item ${r.id === currentRoom ? 'active' : ''}" onclick="joinRoom('${r.id}')">
                            ${r.name} (${r.user_count})
                        </div>`
                    ).join('');
                });
        }
        
        function joinRoom(roomId) {
            if (ws) ws.close();
            currentRoom = roomId;
            connect(roomId);
            loadRooms();
            document.getElementById('messages').innerHTML = '';
        }
        
        // 初始化
        connect(currentRoom);
        loadRooms();
        
        document.getElementById('message-input').addEventListener('keypress', (e) => {
            if (e.key === 'Enter') sendMessage();
        });
    </script>
</body>
</html>
```







