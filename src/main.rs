use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use sqlite::State;

fn returndbdata(creature: &str) -> String {
    let connection = sqlite::open("./data/sqlite.db").unwrap();
    let mut query = "SELECT * FROM cosdata WHERE creature = ";

    let creature = format!("'{}'", creature);

    let binding = (query.to_string() + &creature);
    query = &binding;

    let mut statement = connection.prepare(query).unwrap();

    let mut response = String::new();
    while let Ok(State::Row) = statement.next() {
        let creature = statement.read::<String, _>("creature").unwrap();
        let price = statement.read::<String, _>("price").unwrap();
        response = creature + ", " + &price.to_string();
        println!("{}", response);
    }
    return response.to_string().to_lowercase();
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Endpoints:\n    Adharcaiin\n    Aereis\n    Aesho")
}

#[get("/Adharcaiin")]
async fn Adharcaiin() -> impl Responder {
    let response = returndbdata("Adharcaiin");
    HttpResponse::Ok().body(response)
}

#[get("/Aereis")]
async fn Aereis() -> impl Responder {
    let response = returndbdata("Aereis");
    HttpResponse::Ok().body(response)
}

#[get("/Aesho")]
async fn Aesho() -> impl Responder {
    let response = returndbdata("Aesho");
    HttpResponse::Ok().body(response)
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