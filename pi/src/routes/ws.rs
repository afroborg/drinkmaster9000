use actix::{Actor, StreamHandler};
use actix_web::{get, web, HttpRequest, Responder, Scope};
use actix_web_actors::ws;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("")]
async fn index(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

pub fn ws_scope() -> Scope {
    web::scope("/ws").service(index)
}
