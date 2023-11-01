use std::{net::SocketAddr, ops::ControlFlow};

use axum::{
    debug_handler,
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    headers::UserAgent,
    response::IntoResponse,
    Extension, TypedHeader,
};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use hyper::{Body, Request, Uri};
use tokio::{net::TcpStream, select};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame as TCloseFrame},
    Message as TMessage,
};
use url::Url;

use crate::state::ProxyState;

#[debug_handler]
pub async fn websocket_handler(
    Extension(state): Extension<ProxyState>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    info!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_websocket(state, req.uri().clone(), socket, addr))
}

pub async fn handle_websocket(state: ProxyState, uri: Uri, mut socket: WebSocket, who: SocketAddr) {
    let url = Url::parse(&state.base).unwrap();
    let base = format!("{}/{}", url.authority(), url.path());
    let base = base.trim_end_matches("/");

    let proto = if url.scheme().starts_with("https") {
        "wss"
    } else {
        "ws"
    };

    let url = format!("{}://{}{}", proto, base, uri.path());
    let res = connect_async(Url::parse(&url).unwrap()).await;

    if let Err(err) = res {
        error!("Could not connect to proxy socket: {:?}", err);
        return;
    }

    let (psocket, _) = res.unwrap();
    let (mut psend, mut precv) = psocket.split();

    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        info!("Pinged {who}...");
    } else {
        error!("Could not send ping {who}!");
        return;
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(&mut psend, tungstenize(msg), who)
                .await
                .is_break()
            {
                return;
            }
        } else {
            warn!("Client {who} abruptly disconnected!");
            return;
        }
    }

    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = precv.next().await {
            cnt += 1;

            if process_message_axum(&mut sender, axumize(msg), who)
                .await
                .is_break()
            {
                break;
            }
        }

        cnt
    });

    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;

            if process_message(&mut psend, tungstenize(msg), who)
                .await
                .is_break()
            {
                break;
            }
        }

        cnt
    });

    select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => info!("{a} messages sent to {who}"),
                Err(a) => error!("Error sending messages {a:?}"),
            };

            recv_task.abort();
        }

        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => info!("Received {b} messages"),
                Err(b) => error!("Error receiving messages {b:?}"),
            }

            send_task.abort();
        }
    }

    info!("Websocket context {who} destroyed!");
}

pub fn tungstenize(msg: Message) -> TMessage {
    match msg {
        Message::Text(v) => TMessage::Text(v),
        Message::Binary(v) => TMessage::Binary(v),

        Message::Close(v) => TMessage::Close(v.map(|v| TCloseFrame {
            code: CloseCode::from(v.code),
            reason: v.reason,
        })),

        Message::Ping(v) => TMessage::Ping(v),
        Message::Pong(v) => TMessage::Pong(v),
    }
}

pub fn axumize(msg: TMessage) -> Message {
    match msg {
        TMessage::Text(v) => Message::Text(v),
        TMessage::Binary(v) => Message::Binary(v),

        TMessage::Close(v) => Message::Close(v.map(|v| CloseFrame {
            code: u16::from(v.code),
            reason: v.reason,
        })),

        TMessage::Ping(v) => Message::Ping(v),
        TMessage::Pong(v) => Message::Pong(v),
        TMessage::Frame(_) => unreachable!(),
    }
}

pub async fn process_message(
    socket: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TMessage>,
    msg: TMessage,
    who: SocketAddr,
) -> ControlFlow<(), ()> {
    match msg {
        TMessage::Text(txt) => {
            socket.send(TMessage::Text(txt)).await.unwrap();
        }

        TMessage::Binary(bin) => {
            socket.send(TMessage::Binary(bin)).await.unwrap();
        }

        TMessage::Close(c) => {
            if let Some(cf) = c.clone() {
                info!(
                    "{} sent close with code {} and reason `{}`.",
                    who, cf.code, cf.reason
                );
            } else {
                info!("{who} somehow sent close message without CloseFrame!");
            }

            socket.send(TMessage::Close(c)).await.unwrap();
            socket.close().await.unwrap();

            return ControlFlow::Break(());
        }

        TMessage::Pong(v) => {
            socket.send(TMessage::Pong(v)).await.unwrap();
        }

        TMessage::Ping(v) => {
            socket.send(TMessage::Ping(v)).await.unwrap();
        }

        TMessage::Frame(_) => unreachable!(),
    }

    ControlFlow::Continue(())
}

pub async fn process_message_axum(
    socket: &mut SplitSink<WebSocket, Message>,
    msg: Message,
    who: SocketAddr,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(txt) => {
            socket.send(Message::Text(txt)).await.unwrap();
        }

        Message::Binary(bin) => {
            socket.send(Message::Binary(bin)).await.unwrap();
        }

        Message::Close(c) => {
            if let Some(cf) = c.clone() {
                info!(
                    "{} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                info!("{who} somehow sent close message without CloseFrame");
            }

            socket.send(Message::Close(c)).await.unwrap();
            socket.close().await.unwrap();

            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            socket.send(Message::Pong(v)).await.unwrap();
        }

        Message::Ping(v) => {
            socket.send(Message::Ping(v)).await.unwrap();
        }
    }

    ControlFlow::Continue(())
}
