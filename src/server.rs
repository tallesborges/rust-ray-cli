// server.rs
use crate::event_storage::{process_event, EventStorage};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn start_server(
    event_storage: Arc<EventStorage>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 23517));
    let listener = TcpListener::bind(addr).await?;

    let server_msg = format!("Server listening on {}", addr);
    
    // Store server info in event storage for TUI to display
    event_storage.set_server_info(server_msg.clone());
    // Only log this once at startup
    event_storage.info("Server", &format!("Started and listening on {}", addr));

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let storage_clone = Arc::clone(&event_storage);
        let error_storage = Arc::clone(&event_storage);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let req_storage = Arc::clone(&storage_clone);
                async move { handle_request(req, req_storage).await }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                let error_msg = format!("Error serving connection: {:?}", err);
                error_storage.error("Server", &error_msg);
            }
        });
    }
}

async fn handle_request(
    req: Request<Incoming>,
    event_storage: Arc<EventStorage>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/_availability_check") => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()),
        (&hyper::Method::POST, "/") => {
            let body_bytes = match req.collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(e) => {
                    let error_msg = format!("Failed to read body: {}", e);
                    event_storage.error("Request", &error_msg);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from(error_msg)))
                        .unwrap())
                }
            };

            let body_str = match String::from_utf8(body_bytes.to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    let error_msg = "Invalid UTF-8 sequence";
                    event_storage.error("Request", error_msg);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from(error_msg)))
                        .unwrap())
                }
            };

            let payload: Value = match serde_json::from_str(&body_str) {
                Ok(v) => v,
                Err(e) => {
                    let error_msg = format!("Invalid JSON: {}", e);
                    event_storage.error("Request", &error_msg);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from(error_msg)))
                        .unwrap())
                }
            };

            if let Some(payloads_array) = payload.get("payloads").and_then(Value::as_array) {
                for p in payloads_array {
                    process_event(p, &event_storage);
                }
                event_storage.info("Request", &format!("Processed {} payloads", payloads_array.len()));
                Ok(Response::new(Full::new(Bytes::from("OK"))))
            } else {
                let error_msg = "Invalid payload structure";
                event_storage.error("Request", error_msg);
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from(error_msg)))
                    .unwrap())
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Full::new(Bytes::from("Method Not Allowed")))
            .unwrap()),
    }
}
