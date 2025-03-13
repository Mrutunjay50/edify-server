use std::future::{Ready, ready};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
// use mongodb::bson::doc;
use std::rc::Rc;

use crate::{
    // db::db::Database,
    // models::{students::Student, teachers::Teacher},
    utils::{api_response::ApiResponse, jwt::Claims},
};

// enum User {
//     Student(Student),
//     Teacher(Teacher),
// }
pub struct JwtAuthMiddleware {
    secret: Rc<String>,
}

impl JwtAuthMiddleware {
    pub fn new(secret: String) -> Self {
        JwtAuthMiddleware {
            secret: Rc::new(secret),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddlewareTransform<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddlewareTransform {
            service,
            secret: Rc::clone(&self.secret),
        }))
    }
}

pub struct JwtAuthMiddlewareTransform<S> {
    service: S,
    secret: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddlewareTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();
        let auth_header = req.headers().get("Authorization");

        println!("{:?}", auth_header);

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Remove "Bearer " prefix
                    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                    let validation = Validation::new(Algorithm::HS256);

                    match decode::<Claims>(token, &decoding_key, &validation) {
                        Ok(token_data) => {
                            println!("{:?}", token_data.claims);
                            // let db = Database::get();
                            // let future = async move {
                            //     let result: Result<(User, String), String> = match token_data.claims.profession.to_uppercase().as_str() {
                            //         "TEACHER" => {
                            //             let filter = doc! { "_id": token_data.claims.user_id };
                            //             match db.teacher_repo.get_teacher(filter).await {
                            //                 Ok(Some(teacher)) => Ok((User::Teacher(teacher), "TEACHER".to_string())),
                            //                 Ok(None) => Err("Teacher not found".to_string()), // Handle empty result
                            //                 Err(_) => Err("Database error".to_string()),
                            //             }
                            //         }
                            //         "STUDENT" => {
                            //             let filter = doc! { "_id": token_data.claims.user_id };
                            //             match db.student_repo.get_student(filter).await {
                            //                 Ok(Some(student)) => Ok((User::Student(student), "STUDENT".to_string())),
                            //                 Ok(None) => Err("Student not found".to_string()), // Handle empty result
                            //                 Err(_) => Err("Database error".to_string()),
                            //             }
                            //         }
                            //         _ => Err("Invalid profession".to_string()),
                            //     };

                            //     // Log the result instead of returning it
                            //     match result {
                            //         Ok(user) => println!("User data: {:?}", user),
                            //         Err(e) => eprintln!("Error: {}", e),
                            //     }
                            // };

                            // // Spawn the async task so it doesn't block request handling
                            // actix_web::rt::spawn(future);

                            req.extensions_mut().insert(token_data.claims);
                        }
                        Err(_) => {
                            return Box::pin(async move {
                                let error_response = serde_json::to_string(&ApiResponse::error(
                                    401,
                                    "Invalid or expired token"
                                ))
                                .unwrap_or_else(|_| "{\"status\":401,\"message\":\"Invalid or expired token\",\"data\":{}}".to_string());

                                Err(actix_web::error::ErrorUnauthorized(error_response))
                            });
                        }
                    }
                }
            }
        } else {
            return Box::pin(async move {
                let error_response = serde_json::to_string(&ApiResponse::error(
                    401,
                    "Authorization header missing",
                ))
                .unwrap_or_else(|_| {
                    "{\"status\":401,\"message\":\"Authorization header missing\",\"data\":{}}"
                        .to_string()
                });

                Err(actix_web::error::ErrorUnauthorized(error_response))
            });
        }

        println!("{:?}", req);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
