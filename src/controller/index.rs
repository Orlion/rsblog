use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct IndexController {
    service: crate::service::article::ArticleService,
}

impl IndexController {
    pub fn new(service: crate::service::article::ArticleService) -> IndexController {
        IndexController { service }
    }

    pub async fn index(&self, req: HttpRequest) -> impl Responder {
        let page: u32 = req.match_info().get("page").unwrap().parse().unwrap();
        let articles = self.service.get_articles((page - 1) * 20, 20).unwrap();
        HttpResponse::Ok().json(articles)
    }
}
