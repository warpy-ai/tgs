use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use webbrowser;

pub async fn authenticate(
    server_url: &str,
    redirect_uri: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Open the user's browser and navigate to your server's /auth/github endpoint
    webbrowser::open(&format!("{}/v1/auth/github", server_url))?;

    // Create a shared state to store the token
    let token = Arc::new(Mutex::new(None));

    // Clone the Arc for the route handler
    let token_for_handler = Arc::clone(&token);

    // Start a local server to receive the token
    let server: Server = HttpServer::new(move || {
        let token_for_route = Arc::clone(&token_for_handler);

        println!("Starting server...");

        App::new().data(token_for_route).route(
            "/loader",
            web::get().to(
                |req: web::Query<HashMap<String, String>>,
                 token: web::Data<Arc<Mutex<Option<String>>>>| async move {
                    let token_value = req.get("token").unwrap().to_string();
                    println!("Received token: {}", token_value);

                    // Store the token in the shared state
                    *token.lock().unwrap() = Some(token_value);

                    HttpResponse::Ok().body("You can close this page now.")
                },
            ),
        )
    })
    .bind(redirect_uri)?
    .run();

    let _ = server.await?;

    // Get the token from the shared state
    let token = token.lock().unwrap().clone().ok_or("Token not found")?;

    Ok(token)
}
