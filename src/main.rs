use actix_web::{get, post, web::{self, Data}, App,  HttpServer, Responder};
use bson::Bson;
// use dotenvy::dotenv;
// use futures::lock::MutexGuard;
use serde::{Deserialize, Serialize};
use std::env;
// use std::time::SystemTime;
use tokio::{self, sync::Mutex};
//use futures_util::future;
// use futures_util::future;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use actix_cors::Cors;
use std::sync::Arc;
// use std::future::Future;
use bson::doc;
mod api;

#[derive(Serialize, Deserialize, Debug)]
enum ChargeStation {
    DidNotAttempt,
    Failed,
    DockedNotBalanced,
    DockedAndBalanced,
}

#[derive(Serialize, Deserialize, Debug)]
enum Defense {
    DidNotAttempt,
    VeryBad,
    Bad,
    Mid,
    Good,
    VeryGood,
}

#[derive(Serialize, Deserialize, Debug)]
struct GamePiece {
    low: i32,
    mid: i32,
    high: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScoreInfo {
    cubes: GamePiece,
    cones: GamePiece,
}

#[derive(Serialize, Deserialize, Debug)]
struct Form {
    submitter: String,
    team: i32,
    match_number: i32,
    auto_data: ScoreInfo,
    left_com: bool,
    auto_charge_station: ChargeStation,
    teleop_data: ScoreInfo,
    cones_missed: i32,
    cubes_missed: i32,
    teleop_charge_station: ChargeStation,
    charge_station_time: i32,
    defense: Defense,
    fouls_caused: String,
    fouls_commited: i32,
    notes: String,
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

async fn write_form(form: Bson, client: tokio::sync::MutexGuard<'_, Client>) {
    println!("Writing form");
    let collection = client.database("2023").collection("KCMT");
    collection.insert_one(form.clone(), None).await.unwrap();
    println!("DOne");
}

#[post("/v1/test")]
async fn echo_post(info: String) -> String {
    return info;
}

#[post("/v1/form")]
async fn get_form(form_req: web::Json<Form>, data: Data<Arc<Mutex<Client>>>) -> String {
    println!("Recived data: {}", &form_req.submitter);
    // write_form(form_req.0, &client).await;
    write_form(bson::to_bson(&form_req).unwrap(),data.lock().await).await;
    return "OK".to_string();
}

#[post("/vdbg/form")]
async fn dbg_get_form(form_req: web::Json<ScoreInfo>, data: Data<Arc<Mutex<Client>>>) -> String {
    println!("Recived data: {}", &form_req.0.cubes.high);
    // write_form(form_req.0, &client).await;
    write_form(bson::to_bson(&form_req).unwrap(),data.lock().await).await;
    return "OK".to_string();
}
#[post("/vdbg/echo")]
async fn dbg_echo(string: String, client: Data<Arc<Mutex<Client>>>) -> String {
    println!("Recived data: {}", string);
    let _ = client.lock().await.database("2023").collection("KCMT").insert_one(doc! {"hello": "World"}, None);
    return "Ok".to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    println!("Hello, world!");
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client =
        Data::new(
            Arc::new(
                Mutex::new(
                    match Client::with_uri_str(client_uri).await {
            Ok(v) => v,
        Err(error) => panic!("Fatal: Could not establish connection to database: {}", error),
                    })));
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(client.clone())
            .service(index)
            .service(get_form)
            .service(echo_post)
            .service(dbg_get_form)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
