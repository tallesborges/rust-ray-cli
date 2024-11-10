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
    println!("Server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let storage = Arc::clone(&event_storage);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| handle_request(req, Arc::clone(&storage)));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Error serving connection: {:?}", err);
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
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from(format!(
                            "Failed to read body: {}",
                            e
                        ))))
                        .unwrap())
                }
            };

            let body_str = match String::from_utf8(body_bytes.to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from("Invalid UTF-8 sequence")))
                        .unwrap())
                }
            };

            let payload: Value = match serde_json::from_str(&body_str) {
                Ok(v) => v,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Full::new(Bytes::from("Invalid JSON")))
                        .unwrap())
                }
            };

            if let Some(payloads_array) = payload.get("payloads").and_then(Value::as_array) {
                for p in payloads_array {
                    process_event(p, &event_storage);
                }
                Ok(Response::new(Full::new(Bytes::from("OK"))))
            } else {
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Invalid payload structure")))
                    .unwrap())
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Full::new(Bytes::from("Method Not Allowed")))
            .unwrap()),
    }
}
