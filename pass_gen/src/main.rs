use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789\
!@#$%^&*()-_=+[]{}|;:,.<>?/~`";

#[derive(Deserialize)]
struct PasswordParams {
    length: Option<usize>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct PasswordResponse {
    password: String,
}

/// Generate a random password of the given length.
fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// The REST endpoint that generates the password.
/// It reads an optional query parameter `length`, ensures it is not more than 64,
/// and then returns a JSON response with the generated password.
#[get("/password")]
async fn password_handler(params: web::Query<PasswordParams>) -> impl Responder {
    // Default length is 32 if not specified.
    let len = params.length.unwrap_or(32);
    if len > 64 {
        return HttpResponse::BadRequest().body("Password length must be 64 characters or less.");
    }
    let password = generate_password(len);
    HttpResponse::Ok().json(PasswordResponse { password })
}

/// The main function that starts the Actix Web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(password_handler))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[test]
    async fn test_generate_password_length() {
        let len = 16;
        let password = generate_password(len);
        assert_eq!(password.len(), len, "Generated password should have length {}", len);
    }

    #[actix_web::test]
    async fn test_password_handler_default_length() {
        let app = test::init_service(App::new().service(password_handler)).await;
        let req = test::TestRequest::get().uri("/password").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Deserialize response
        let result: PasswordResponse = test::read_body_json(resp).await;
        // Default password length should be 32
        assert_eq!(result.password.len(), 32, "Default password length should be 32");
    }

    #[actix_web::test]
    async fn test_password_handler_custom_length() {
        let custom_length = 16;
        let app = test::init_service(App::new().service(password_handler)).await;
        let req = test::TestRequest::get()
            .uri(&format!("/password?length={}", custom_length))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let result: PasswordResponse = test::read_body_json(resp).await;
        assert_eq!(result.password.len(), custom_length, "Password length should be {}", custom_length);
    }

    #[actix_web::test]
    async fn test_password_handler_invalid_length() {
        // Set an invalid length (> 64)
        let invalid_length = 65;
        let app = test::init_service(App::new().service(password_handler)).await;
        let req = test::TestRequest::get()
            .uri(&format!("/password?length={}", invalid_length))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "Invalid length should return 400 Bad Request");
    }
}
