use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Endpoints:\nAdharcaiin\nAereis\nAesho")
}

#[get("/Adharcaiin")]
async fn Adharcaiin() -> impl Responder {
    HttpResponse::Ok().body("Creature: Adharcaiin \nPrice: $1,000 mush")
}

#[get("/Aereis")]
async fn Aereis() -> impl Responder {
    HttpResponse::Ok().body("Creature: Aereis \nPrice: $25,000-$30,000 mush")
}

#[get("/Aesho")]
async fn Aesho() -> impl Responder {
    HttpResponse::Ok().body("Creature: Aesho \nPrice: $50 mush")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(Aereis)
            .service(Adharcaiin)
            .service(Aesho)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

