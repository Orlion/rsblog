use actix_web::{HttpRequest, Responder};

use crate::CONTROLLERS;

pub async fn index_handler(req: HttpRequest)  -> impl Responder {
    unsafe { 
        CONTROLLERS.unwrap().index_controller.index(req).await
    }
}