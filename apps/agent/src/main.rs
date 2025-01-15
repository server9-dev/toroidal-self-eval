use actix_web::{get, rt, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello world"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rt::spawn(async {
        loop {
            // Agent logic will go here, API endpoints will be for querying the agent
            println!("Heartbeat check...");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
