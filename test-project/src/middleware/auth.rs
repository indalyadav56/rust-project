// use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, web, Error};


// pub async fn auth_middleware(
//     req: ServiceRequest,
//     next: Next<impl MessageBody>,
// ) -> Result<ServiceResponse<impl MessageBody>, Error> {
//     // pre-processing
//     println!("auth middleware, pre-processing");
//     let resp = next.call(req).await;
//     // post-processing
//     println!("auth middleware, post-processing");
//     resp
// }
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    App, Error,
};

pub async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    println!("auth middleware, pre-processing");
    next.call(req).await
    // post-processing
}
