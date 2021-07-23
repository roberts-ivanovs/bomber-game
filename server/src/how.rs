// use std::io;

// use uuid::Uuid;
// use warp::{Reply, hyper::StatusCode};

// use crate::api::ApiResult;

// /// Defines the base error enum with all of the possible variants of errors
// /// If a handler returns an error of this enum, then it will be gracefully
// /// transformed into a JSON response.
// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("Failed websocket transaction!")]
//     InvalidWebsocketUserId(Uuid),
// }

// impl Reply for Error {

//     fn into_response(self) -> warp::reply::Response {
//         let body = ApiResult::<()>::new()
//             .with_msg(format!("{}", &self))
//             .code(StatusCode::OK.as_u16());

//         warp::reply::json(body);
//         HttpResponse::Ok()
//             .content_type("application/json")
//             .status(StatusCode::from_u16(body.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
//             .json(body)
//     }
// }
