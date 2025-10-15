use actix_web::{ post, web, App, HttpServer, Result };
use serde::{ Serialize, Deserialize };

#[derive(Deserialize)]
struct TxRequest {
    jawn: String,
}

async fn index(txrequest: web::Json<TxRequest>) -> Result<String> {
    println!("Index hit...");
    Ok(format!("Your jawn: {}", txrequest.jawn))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("TxnOS welcomes you!");
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
