mod controller;
mod dao;
mod domain;
mod handler;
mod model;
mod service;

use actix_web::{web, App, HttpServer};
use controller::index::IndexController;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    bootstrap();

    HttpServer::new(|| App::new().route("/index/{page}", web::get().to(handler::index_handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

static mut CONTROLLERS: Option<&Controllers> = None;

struct Controllers {
    index_controller: IndexController,
}

fn bootstrap() {
    // 初始化数据库连接池
    let pool = mysql::Pool::new("mysql://root:123456@localhost:3306/blog")
        .expect("数据库连接池初始化失败");
    let article_dao = dao::article::ArticleDao::new(pool);
    let article_service = service::article::ArticleService::new(article_dao);
    let index_controller = IndexController::new(article_service);

    let controllers = Box::new(Controllers { index_controller });

    unsafe { CONTROLLERS = Some(Box::leak(controllers)) };
}
