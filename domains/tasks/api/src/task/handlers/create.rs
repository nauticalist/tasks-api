use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Json;
use core::errors::{ServiceError};
use service::task::actions::create::TaskCreatorService;
use service::task::models::NewTask;

pub async fn create<T: TaskCreatorService>(body: Json<NewTask>) -> Result<HttpResponse, ServiceError> {
    let task = T::create_task(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(task))
}