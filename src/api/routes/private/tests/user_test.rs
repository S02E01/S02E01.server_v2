#[cfg(test)]
mod tests {
    use actix_web::{test::{self, TestRequest}, web, App};
    // use actix_redis::RedisSession;
    use serde_json::json;

    use crate::api;
    use crate::sql::store::user_repository::user::User;

    #[actix_rt::test]
    async fn test_registration() {
        crate::tests::init();

        let request_body = json!({
            "chat_id": 10,
            "role": 1,
        });

        let mut app = test::init_service(
            App::new().service(
                web::scope("/api-v2")
                    .configure(api::routes::public)
                    .configure(api::routes::private),
            )
        ).await;

        let resp = TestRequest::post().uri("/api-v2/registration").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create user");

    }
}