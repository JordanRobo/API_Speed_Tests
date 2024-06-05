pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::RunQueryDsl;
use dotenv::dotenv;
use actix_web::{ get, App, HttpServer, HttpResponse };
use actix_web::web::Data;
use models::Test;
use crate::schema::test::dsl::test;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub async fn get_data(pool: &DbPool) -> Vec<Test> {
    let mut conn = pool.get().unwrap();
    test.load::<Test>(&mut conn).expect("Error loading posts")
}

#[get("/data")]
pub async fn data_api(pool: Data<DbPool>) -> HttpResponse {
    let data = get_data(&pool).await;
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = establish_connection();
    let app_data = Data::new(db);

    HttpServer::new(move || 
        App::new()
            .app_data(app_data.clone())
            .service(data_api)
    )
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}