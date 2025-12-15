use actix_web::{web, App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server startet auf http://localhost:8080");
    println!("📂 Serving files from current directory");
    
    HttpServer::new(|| {
        App::new()
            // Serve index.html at root
            .route("/", web::get().to(|| async {
                fs::NamedFile::open_async("./index.html").await
            }))
            // Serve static files
            .service(fs::Files::new("/", ".").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
