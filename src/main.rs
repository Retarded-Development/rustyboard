#[macro_use]
extern crate diesel;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpRequest, HttpServer, error};
use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use tera::Tera;

pub mod models;
pub mod actions;
pub mod schema;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[get("/{id}/{name}/index.html")]
async fn index(
    info: web::Path<(u32, String)>,
    tmpl: web::Data<tera::Tera>,
) ->  Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &info.1.to_owned());
    ctx.insert("text", &"Welcome!".to_owned());
    let s = tmpl.render("index.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    // format!("Hello {}! id:{}", info.1, info.0)
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
    
}

#[post("/new_post")]
async fn new_post(
    pool: web::Data<DbPool>,
    tmpl: web::Data<tera::Tera>,
    form: web::Form<models::NewPostForm>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    dbg!(req.connection_info());
    // use web::block to offload blocking Diesel code without blocking server thread
    let new_post = web::block(move || actions::insert_new_post(form, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(new_post))
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
            .service(index)
            .service(new_post)
    })
    .bind(&bind)?
    .run()
    .await
}