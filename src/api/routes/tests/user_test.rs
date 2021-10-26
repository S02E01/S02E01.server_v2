#[cfg(test)]
mod tests {
    use actix_session::CookieSession;
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::json;

    use crate::api;
    use crate::sql::store::user_repository::user::UserData;

    #[actix_rt::test]
    async fn test_user() {
        crate::sql::tests::init();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/api-v2")
                    .wrap(CookieSession::signed(&[0; 32]).secure(false))
                    .configure(api::routes::public)
                    .configure(api::routes::private),
            ),
        )
        .await;

        /*
         * РЕГЕСТРИРУЮ НОВОГО ТЕСТОВОГО ПОЛЬЗОВАТЕЛЯ
         */

        let request_body = json!({
            "chat_id": 11,
            "role": 2,
        });

        let resp = test::TestRequest::post()
            .uri("/api-v2/registration")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status().as_u16(), StatusCode::CREATED);

        /*
         * РЕГЕСТРИРУЮ ТЕСТОВОГО АДМИНА
         */

        let request_body = json!({
            "chat_id": 10,
            "role": 2,
        });

        let resp = test::TestRequest::post()
            .uri("/api-v2/registration")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status().as_u16(), StatusCode::CREATED);

        // Разбираю json на структуру
        let user: UserData = test::read_body_json(resp).await;

        /*
         * ПЫТАЮСЬ АВТОРИЗОВАТЬСЯ АДМИНОМ
         */

        let resp = test::TestRequest::post()
            .uri(&format!("/api-v2/auth/{}", user.chat_id))
            .set_json(&request_body)
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status().as_u16(), StatusCode::OK);

        // Тестирую получение информации пользователя о себе
        let resp_get_user = test::TestRequest::get()
            .cookie(resp.response().cookies().next().unwrap())
            .uri(&format!("/api-v2/user/{}", user.chat_id))
            .send_request(&mut app)
            .await;

        assert_eq!(resp_get_user.status().as_u16(), StatusCode::OK);

        /*
         * ТЕСТИРОВАНИЕ ОБНОВЛЕНИЯ РОЛИ ДЛЯ ВТОРОГО ТЕСТОВОГО ПОЛЬЗОВАТЕЛЯ
         */

        let request_body = json!({
            "chat_id": 11,
        });

        let resp_get_user = test::TestRequest::put()
            .cookie(resp.response().cookies().next().unwrap())
            .set_json(&request_body)
            .uri(&format!("/api-v2/user/{}", user.chat_id))
            .send_request(&mut app)
            .await;

        assert_eq!(resp_get_user.status().as_u16(), StatusCode::OK);
        // let updated_user: UserData = test::read_body_json(resp).await;
    }
}
