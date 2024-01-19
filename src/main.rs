use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use sqlite::State;

fn returncreaturedata(creature: &str) -> String {
    let connection = sqlite::open("./data/sqlite.db").unwrap();
    let mut query = "SELECT * FROM costdata WHERE creature = ";

    let creature = format!("'{}'", creature);

    let binding = query.to_string() + &creature;
    query = &binding;

    let mut statement = connection.prepare(query).unwrap();

    let mut response = String::new();
    while let Ok(State::Row) = statement.next() {
        let creature = statement.read::<String, _>("creature").unwrap();
        let price = statement.read::<String, _>("price").unwrap();
        response = creature + ", " + &price.to_string();
        println!("{}", response);
    }
    response.to_string().to_lowercase()
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Endpoints:\n    adharcaiin\n    aereis\n    aesho")
}

#[get("/{creature}")]
async fn displayout(arg: web::Path<(String)>) -> impl Responder {
    let arg = arg.into_inner();
    let creature = format!("{}", arg.to_string().to_ascii_lowercase());
    let response = returncreaturedata(&creature);
    HttpResponse::Ok().body(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(displayout)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
