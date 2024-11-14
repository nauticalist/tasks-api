use actix_web::HttpResponse;
use service::task::actions::get::TaskFetcherService;

use core::errors::{ServiceError};

pub async fn get_all<T: TaskFetcherService>() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(T::get_tasks().await?))
}
