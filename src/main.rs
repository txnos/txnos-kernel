use actix_web::{ post, web, App, HttpServer, Result };
use serde::{ Serialize, Deserialize };


#[derive(Serialize, Deserialize, Debug)]
struct St {
    obj: String,
    chash: String,
    nvalue: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Txn {
    from: String,
    st: St,
}


#[derive(Serialize, Deserialize, Debug)]
struct TxRequest {
    ver: u64,
    txn: Txn,
    sig: String,
}

async fn index(txrequest: web::Json<TxRequest>) -> Result<String> {
    println!("Index hit...");
    Ok(format!("Your jawn: {}", txrequest.ver))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("TxnOS welcomes you!");
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
