use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use webbrowser;

async fn token_receiver(
    query: web::Query<HashMap<String, String>>,
    token_state: web::Data<Arc<Mutex<Option<String>>>>,
) -> impl Responder {
    if let Some(token) = query.get("token") {
        let mut token_store = token_state.lock().unwrap();
        *token_store = Some(token.clone());
        println!("Token received: {}", token);
        HttpResponse::Ok().body("Token received, you may close this window.")
    } else {
        HttpResponse::BadRequest().body("Token not provided in the query.")
    }
}

pub async fn authenticate(server_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // The shared state to store the token
    let token_state = Arc::new(Mutex::new(None::<String>));

    // Clone the Arc to use inside the HttpServer closure
    let token_state_for_server = Arc::clone(&token_state);

    // Start the local HTTP server to listen for the GitHub callback redirect
    let server = HttpServer::new(move || {
        let server_data = web::Data::new(token_state_for_server.clone());
        App::new()
            .app_data(server_data) // Use the cloned Arc here
            .route("/loader", web::get().to(token_receiver))
    })
    .bind("localhost:3400")?
    .run();

    // Open the user's browser to initiate GitHub OAuth
    webbrowser::open(&format!("{}/v1/auth/github", server_url))?;

    // Run the server in the background
    let server_future = server.await;

    let mut token = None;
    while token.is_none() {
        let token_state_guard = token_state.lock().map_err(|e| e.to_string())?;
        token = token_state_guard.clone();
        if token.is_none() {
            drop(token_state_guard); // Drop the lock before sleeping
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    // Stop the system only after we've received the token
    actix_web::rt::System::current().stop();

    server_future.map_err(|e| e.to_string())?;

    // Unwrap the token safely and return it
    token.ok_or_else(|| "Authentication failed: Token not received".into())
}
