use std::{
    env, sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    }, time::Instant
};

use actix::*;
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
// `actix_web_actors` is deprecated. Use `actix_ws` instead. WebSockets for Actix Web, without actors.
use actix_web_actors::ws;

mod server;
mod session;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
/// TODO: How to do it in Rocket? There seems to have no such function in the example.
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

// the actor-based WebSocket examples REQUIRE `actix_web::main` for actor support
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info")); // e.g. [2017-11-09T02:12:24Z ERROR main] this is printed by default

    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    log::info!("starting HTTP server at http://localhost:18080");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(web::resource("/").to(index))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            // Use the following:
            // cargo build; cargo run --bin websocket-chat-server
            // , otherwise the pwd will be /actix
            .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)     // <-------------------------------- WORKERS
    .bind(("127.0.0.1", 18080))?
    .run()
    .await
}
