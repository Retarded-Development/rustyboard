#[macro_use]
extern crate diesel;
use actix_web::{
    error, get, http, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use tera::Tera;

pub mod actions;
pub mod models;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[get("/{slug}/")]
async fn list(
    info: web::Path<(u32, String)>,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    ctx.insert("name", &info.1.to_owned());
    ctx.insert("text", &"Welcome!".to_owned());
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    // format!("Hello {}! id:{}", info.1, info.0)
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


#[get("/{slug}/{id}/")]
async fn detail(
    info: web::Path<(u32, String)>,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &info.1.to_owned());
    ctx.insert("text", &"Welcome!".to_owned());
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    // format!("Hello {}! id:{}", info.1, info.0)
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/{id}/new_thread")]
async fn new_thread_get(
    info: web::Path<u32>,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("board_id", &info.to_owned());
    let s = tmpl
        .render("new_thread.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    // format!("Hello {}! id:{}", info.1, info.0)
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/new_thread")]
async fn new_thread(
    pool: web::Data<DbPool>,
    form: web::Form<models::NewPostForm>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    dbg!(req.headers());
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let other_new_post = web::block(move || actions::insert_new_post(form, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let post_id = other_new_post.post_id;
    let conn = pool.get().expect("couldn't get db connection from pool");
    let board = web::block(move || actions::get_board(other_new_post.board_id, &conn)).await?;
    Ok(HttpResponse::Found()
        .header(
            http::header::LOCATION,
            format!("http://localhost:8080{}/{}?nocache=true", board, post_id),
        )
        .finish()
        .into_body())
}

#[get("/{id}/{thread_id}/{parent_id}/new_post")]
async fn new_post_get(
    info: web::Path<(i32, i32, i32)>,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("board_id", &info.0.to_owned());
    ctx.insert("thread_id", &info.1.to_owned());
    ctx.insert("parent_id", &info.2.to_owned());
    let s = tmpl
        .render("new_post.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    // format!("Hello {}! id:{}", info.1, info.0)
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/new_post")]
async fn new_post(
    pool: web::Data<DbPool>,
    form: web::Form<models::NewPostForm>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    dbg!(req.headers());
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let new_post = web::block(move || actions::insert_new_post(form, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let mut post_id = new_post.post_id;
    if let Some(parent_id) = new_post.parent_id {
        post_id = parent_id;
    }
    let conn = pool.get().expect("couldn't get db connection from pool");
    let board = web::block(move || actions::get_board(new_post.board_id, &conn)).await?;
    Ok(HttpResponse::Found()
        .header(
            http::header::LOCATION,
            format!("http://localhost:8080{}/{}/?nocache=true", board, post_id),
        )
        .finish()
        .into_body())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .data(tera)
            .wrap(middleware::Logger::default())
            .service(list)
            .service(detail)
            .service(new_post)
            .service(new_post_get)
            .service(new_thread)
            .service(new_thread_get)
    })
    .bind(&bind)?
    .run()
    .await
}
