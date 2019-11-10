use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello")]
fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new()
        .route("/", web::get().to(index))
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:4000").unwrap()
    };

    server.run().unwrap();

    // HttpServer::new(|| {
    //     App::new()
    //         .service(hello)
    //         .route("/", web::get().to(index))
    // })
    // .bind("127.0.0.1:4000")
    // .unwrap()
    // .run()
    // .unwrap();
}
