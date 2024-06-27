use handlebars::{Handlebars, DirectorySourceOptions};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    fs::File,
    io::BufReader,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex, PoisonError},
};
use tokio::net::TcpListener;
use viz::{
    handlers::serve, serve, types::State, Error, IntoResponse, Request, Response, ResponseExt, RequestExt, Result, Router, StatusCode,
    middleware::limits,
    header::HeaderValue,
};


use viz::Body;
use http_body_util::Full;
use bytes::Bytes;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Post {
    id: usize,
    title: String,
    content: String,
}

#[derive(Debug, Clone)]
struct AppState {
    posts: Vec<Post>,
}

type SharedState = Arc<Mutex<AppState>>;

static TPLS: Lazy<Handlebars> = Lazy::new(|| {
    let mut handlebars = Handlebars::new();
    
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut templates_path = PathBuf::from(dir);
    templates_path.push("templates");

    let options = DirectorySourceOptions {
        tpl_extension: ".html".to_string(),
        hidden: false,
        temporary: false,
    };

    handlebars
        .register_templates_directory(templates_path, options)
        .unwrap();

    handlebars
});

fn into_error<T>(e: PoisonError<T>) -> Error {
    e.to_string().into_error()
}

async fn index(req: Request) -> Result<Response> {
    let state = req.state::<SharedState>().unwrap();
    let posts = state.lock().map_err(into_error)?.posts.clone();
    let body = TPLS
        .render(
            "index",
            &json!({
                "posts": posts
            }),
        )
        .map_err(Error::boxed)?;
    Ok(Response::html(body))
}


async fn show_post(req: Request) -> Result<Response> {
    let state = req.state::<SharedState>().unwrap();
    let posts = state.lock().map_err(into_error)?.posts.clone();
    let post_id: usize = req.param::<usize>("id")?;

    if let Some(post) = posts.into_iter().find(|p| p.id == post_id) {

        let body = TPLS
            .render(
                "post",
                &json!({
                    "post": post
                }),
            )
            .map_err(Error::boxed)?;
        Ok(Response::html(body))
    } else {
        println!("Post not found"); // Debug statement
        let not_found_body = Full::new(Bytes::from("Post not found"));
        let not_found_response = Response::builder()
            .body(Body::from(not_found_body))
            .map_err(Error::boxed)?;
        Ok(not_found_response)
    }
}

async fn create(mut req: Request) -> Result<Response> {
    let post = req.form::<Post>().await?;
    let db = req.state::<SharedState>().unwrap();

    let mut posts_data = db.lock().map_err(into_error)?;
    posts_data.posts.push(post);

    let mut resp = StatusCode::CREATED.into_response();
    resp.headers_mut()
        .insert("HX-Trigger", HeaderValue::from_static("newPost"));
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on http://{addr}");

    let file = File::open("data/posts.json").expect("Failed to open posts.json");
    let reader = BufReader::new(file);
    let posts: Vec<Post> = serde_json::from_reader(reader).expect("Failed to parse posts.json");
    println!("Loaded posts: {:?}", posts); // Add this debug statement

    let state = Arc::new(Mutex::new(AppState { posts }));

    let dir = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();

    let app = Router::new()
        .get("/", index)
        .get("/posts/:id", show_post)
        .post("/posts", create)
        .get("/static/styles.css", serve::File::new(dir.join("static/styles.css")))
        .with(State::new(state))
        .with(limits::Config::default());

    if let Err(e) = serve(listener, app).await {
        println!("{e}");
    }

    Ok(())
}
