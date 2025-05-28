use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod db;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello "
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    
    // Establish database connection
    let pool = db::establish_connection().await.expect("Failed to connect to database");
    println!("✅ Connection to the database is successful!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone())) // Add database pool to app data
            .wrap(cors)
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}