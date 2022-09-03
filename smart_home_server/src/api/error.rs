use actix_web::error::ResponseError;
use smart_home::SmartHomeError;
use thiserror::Error;

#[derive(Error, std::fmt::Debug)]
#[error("{0}")]
pub struct HttpSmartHomeError(#[from] SmartHomeError);

impl ResponseError for HttpSmartHomeError {}
