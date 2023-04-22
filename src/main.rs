use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpServer};
use std::path::PathBuf;

#[get("/")]
async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path = ["static", "index.html"].into_iter().collect::<PathBuf>();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
