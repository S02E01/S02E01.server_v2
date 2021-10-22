#[cfg(test)]
use actix_web::{http, test, web, App, HttpResponse};
use crate::server;



#[actix_rt::test]
async fn test_norm() {
  //   let mut app = test::init_service(
    //     App::new()        
    //     .service(
    //         web::scope("/api-v2/user/5658681")     
    //         .service(user::get_user)      
    //     )
            
    // ).await;
    let req = test::TestRequest::default().to_http_request();
    // let resp = server::user::get_user(req);
    // let resp = user::get_user(req).await;
    // let resp = test::call_service(&mut app, req).await;
    // assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().route("/", web::get().to(|| HttpResponse::Ok().body("/")))).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}