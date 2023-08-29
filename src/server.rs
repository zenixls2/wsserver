use actix::prelude::*;
use actix_web_actors::ws;
use log::info;
use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct Wsserver {}

impl Wsserver {
    pub fn new() -> Self {
        Self {}
    }
}
type WsResult = Result<ws::Message, ws::ProtocolError>;
impl StreamHandler<WsResult> for Wsserver {
    fn handle(&mut self, item: WsResult, ctx: &mut ws::WebsocketContext<Self>) {
        if item.is_err() {
            ctx.stop();
            return;
        }
        match item.unwrap() {
            ws::Message::Text(text) => {
                info!("recv {}", text);
                ctx.text(text);
            }
            ws::Message::Ping(t) => {
                info!("ping {:?}", t);
            }
            ws::Message::Pong(_) => {
                info!("pong")
            }
            _ => {}
        }
    }
    fn finished(&mut self, _ctx: &mut Self::Context) {
        info!("finished");
    }
}

impl Actor for Wsserver {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let (tx, rx) = unbounded_channel();
        ctx.add_stream(UnboundedReceiverStream::new(rx));
        ctx.run_interval(Duration::from_secs(1), move |_act, _ctx| {
            tx.send(Ok(ws::Message::Text("124".into()))).unwrap()
        });
    }
}
