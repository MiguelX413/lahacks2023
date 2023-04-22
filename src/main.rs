use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

struct AppState {
    posts: Mutex<Vec<Post>>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Post {
    name: String,
    text: String,
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path = ["static", "index.html"].into_iter().collect::<PathBuf>();
    Ok(NamedFile::open(path)?)
}

#[get("/posts")]
async fn posts(data: web::Data<AppState>) -> actix_web::Result<web::Json<Vec<Post>>> {
    Ok(web::Json(data.posts.lock().unwrap().to_owned()))
}

#[post("/post")]
async fn post(req: web::Json<Post>, data: web::Data<AppState>) -> actix_web::Result<&'static str> {
    data.posts.lock().unwrap().push(req.0);
    Ok("Submitted!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        posts: Mutex::new(vec![
            Post {
                name: "Miguel".to_string(),
                text: "Hi".to_string(),
            },
            Post {
                name: "Kevin".to_string(),
                text: "Bye".to_string(),
            },
            Post {
                name: "Josh".to_string(),
                text: "Bouncy House".to_string(),
            },
        ]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(posts)
            .service(post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
