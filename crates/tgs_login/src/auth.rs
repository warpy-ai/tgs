use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use tokio;
use webbrowser; // Ensure you are using a version compatible with your Actix Web

async fn token_receiver(
    query: web::Query<HashMap<String, String>>,
    token_state: web::Data<Arc<Mutex<Option<String>>>>,
    shutdown_sender: web::Data<mpsc::Sender<()>>, // Specify the type here
) -> impl Responder {
    if let Some(token) = query.get("token") {
        {
            let mut token_store = token_state.lock().unwrap();
            *token_store = Some(token.clone());
        }
        let _ = shutdown_sender.send(()); // Send shutdown signal
        HttpResponse::Ok().body("Token received, you may close this window.")
    } else {
        HttpResponse::BadRequest().body("Token not provided in the query.")
    }
}

pub async fn authenticate(server_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let token_state = Arc::new(Mutex::new(None::<String>));
    let (shutdown_sender, shutdown_receiver) = mpsc::channel::<()>();

    let token_state_for_server = Arc::clone(&token_state);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(token_state_for_server.clone()))
            .app_data(web::Data::new(shutdown_sender.clone()))
            .route("/loader", web::get().to(token_receiver))
    })
    .bind("localhost:3400")?
    .run();

    let server_handle = tokio::spawn(server);

    webbrowser::open(&format!("{}/v1/auth/github", server_url))?;

    // Wait for the shutdown signal from the token_receiver
    shutdown_receiver.recv().unwrap();

    // Instead of stopping the Actix system, gracefully stop the server
    server_handle.abort();

    // Await the server handle if necessary
    // Note: this might not be needed depending on your application's structure
    let _ = server_handle.await;

    let token = token_state
        .lock()
        .unwrap()
        .clone()
        .ok_or("Authentication failed: Token not received")?;

    Ok(token)
}
