//! HTTP server builtin functions for Jing language.
//!
//! Provides basic HTTP server capabilities including:
//! - Starting HTTP servers on specified ports
//! - Handling basic GET/POST requests
//! - Serving static content and JSON responses

use crate::error::{JingError, JingResult};
use crate::features::BuiltinFunction;
use crate::value::Value;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use tokio::net::TcpListener;

/// Global storage for HTTP server state
static HTTP_SERVERS: OnceLock<Mutex<HashMap<u16, ServerHandle>>> = OnceLock::new();

fn get_servers() -> &'static Mutex<HashMap<u16, ServerHandle>> {
    HTTP_SERVERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Handle for a running HTTP server
#[derive(Clone)]
struct ServerHandle {
    #[allow(dead_code)]
    port: u16,
    running: Arc<Mutex<bool>>,
}

/// Start HTTP server builtin function
/// Usage: start_http_server(port, handler_name)
#[derive(Debug)]
pub struct StartHttpServerFunction;

impl BuiltinFunction for StartHttpServerFunction {
    fn name(&self) -> &'static str {
        "start_http_server"
    }

    fn arity(&self) -> usize {
        1
    }

    fn help(&self) -> &'static str {
        "start_http_server(port) - Start HTTP server on specified port (8000-9999)"
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if args.len() != 1 {
            return Err(JingError::runtime_error(
                "start_http_server() expects 1 argument (port)",
            ));
        }

        let port = match &args[0] {
            Value::Number(n) => {
                let port = *n as u16;
                if !(8000..=9999).contains(&port) {
                    return Err(JingError::runtime_error(
                        "Port must be between 8000 and 9999",
                    ));
                }
                port
            }
            _ => {
                return Err(JingError::runtime_error(
                    "start_http_server() port must be a number",
                ))
            }
        };

        // Check if server already running on this port
        let mut servers = get_servers().lock().unwrap();
        if servers.contains_key(&port) {
            return Ok(Value::String(format!(
                "Server already running on port {}",
                port
            )));
        }

        // Start the server in a separate thread
        let running = Arc::new(Mutex::new(true));
        let handle = ServerHandle {
            port,
            running: running.clone(),
        };

        servers.insert(port, handle);
        drop(servers);

        // Start server in background thread
        let running_clone = running.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Err(e) = start_server(port, running_clone).await {
                    eprintln!("HTTP server error: {}", e);
                }
            });
        });

        // Give the server a moment to start
        thread::sleep(std::time::Duration::from_millis(100));

        Ok(Value::String(format!(
            "HTTP server started on port {}",
            port
        )))
    }
}

/// Stop HTTP server builtin function
/// Usage: stop_http_server(port)
#[derive(Debug)]
pub struct StopHttpServerFunction;

impl BuiltinFunction for StopHttpServerFunction {
    fn name(&self) -> &'static str {
        "stop_http_server"
    }

    fn arity(&self) -> usize {
        1
    }

    fn help(&self) -> &'static str {
        "stop_http_server(port) - Stop HTTP server running on specified port"
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if args.len() != 1 {
            return Err(JingError::runtime_error(
                "stop_http_server() expects 1 argument (port)",
            ));
        }

        let port = match &args[0] {
            Value::Number(n) => *n as u16,
            _ => {
                return Err(JingError::runtime_error(
                    "stop_http_server() port must be a number",
                ))
            }
        };

        let mut servers = get_servers().lock().unwrap();
        if let Some(handle) = servers.remove(&port) {
            *handle.running.lock().unwrap() = false;
            Ok(Value::String(format!(
                "HTTP server on port {} stopped",
                port
            )))
        } else {
            Ok(Value::String(format!("No server running on port {}", port)))
        }
    }
}

/// HTTP request handler builtin function
/// Usage: http_response(status, content_type, body)
#[derive(Debug)]
pub struct HttpResponseFunction;

impl BuiltinFunction for HttpResponseFunction {
    fn name(&self) -> &'static str {
        "http_response"
    }

    fn arity(&self) -> usize {
        3
    }

    fn help(&self) -> &'static str {
        "http_response(status, content_type, body) - Create HTTP response (status: 200-599, content_type: string, body: string)"
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if args.len() != 3 {
            return Err(JingError::runtime_error(
                "http_response() expects 3 arguments (status, content_type, body)",
            ));
        }

        let status = match &args[0] {
            Value::Number(n) => {
                let status = *n as u16;
                if !(200..=599).contains(&status) {
                    return Err(JingError::runtime_error(
                        "HTTP status must be between 200 and 599",
                    ));
                }
                status
            }
            _ => {
                return Err(JingError::runtime_error(
                    "http_response() status must be a number",
                ))
            }
        };

        let content_type = match &args[1] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(JingError::runtime_error(
                    "http_response() content_type must be a string",
                ))
            }
        };

        let body = match &args[2] {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Nil => "null".to_string(),
            _ => {
                return Err(JingError::runtime_error(
                    "http_response() body must be a string, number, boolean, or null",
                ))
            }
        };

        // For now, we'll return a formatted string that represents the response
        // In a full implementation, this would be used by the server
        Ok(Value::String(format!(
            "HTTP/{} {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            "1.1",
            status,
            get_status_text(status),
            content_type,
            body.len(),
            body
        )))
    }
}

/// List running HTTP servers builtin function
/// Usage: list_http_servers()
#[derive(Debug)]
pub struct ListHttpServersFunction;

impl BuiltinFunction for ListHttpServersFunction {
    fn name(&self) -> &'static str {
        "list_http_servers"
    }

    fn arity(&self) -> usize {
        0
    }

    fn help(&self) -> &'static str {
        "list_http_servers() - List all running HTTP servers"
    }

    fn call(&self, args: Vec<Value>) -> JingResult<Value> {
        if !args.is_empty() {
            return Err(JingError::runtime_error(
                "list_http_servers() expects no arguments",
            ));
        }

        let servers = get_servers().lock().unwrap();
        let mut result = String::new();

        if servers.is_empty() {
            result.push_str("No HTTP servers running");
        } else {
            result.push_str("Running HTTP servers:\n");
            for (port, handle) in servers.iter() {
                let running = *handle.running.lock().unwrap();
                result.push_str(&format!(
                    "  Port {}: {}\n",
                    port,
                    if running { "running" } else { "stopped" }
                ));
            }
        }

        Ok(Value::String(result))
    }
}

/// Simple HTTP server implementation
async fn start_server(
    port: u16,
    running: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let listener = TcpListener::bind(addr).await?;

    println!("HTTP server listening on http://{}", addr);

    while *running.lock().unwrap() {
        // Set a short timeout for accepting connections
        let timeout =
            tokio::time::timeout(std::time::Duration::from_millis(100), listener.accept()).await;

        match timeout {
            Ok(Ok((stream, _))) => {
                let io = TokioIo::new(stream);
                let running_clone = running.clone();

                tokio::task::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(
                            io,
                            service_fn(move |req| {
                                let running = running_clone.clone();
                                handle_request(req, running)
                            }),
                        )
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
            Ok(Err(e)) => {
                eprintln!("Error accepting connection: {}", e);
                break;
            }
            Err(_) => {
                // Timeout - continue loop to check if we should stop
                continue;
            }
        }
    }

    Ok(())
}

/// Handle HTTP requests
async fn handle_request(
    req: Request<hyper::body::Incoming>,
    _running: Arc<Mutex<bool>>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Full::new(Bytes::from(
                "Hello from Jing HTTP Server!\n\nEndpoints:\n- GET / (this page)\n- GET /status\n- POST /echo"
            ))))
        }
        (&Method::GET, "/status") => {
            let response_body = serde_json::json!({
                "status": "ok",
                "server": "Jing HTTP Server",
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            });

            let mut response = Response::new(Full::new(Bytes::from(response_body.to_string())));
            response.headers_mut().insert(
                hyper::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            Ok(response)
        }
        (&Method::POST, "/echo") => {
            let body_bytes = req.collect().await?.to_bytes();
            let body_str = String::from_utf8_lossy(&body_bytes);

            let response_body = serde_json::json!({
                "echo": body_str,
                "method": "POST",
                "path": "/echo"
            });

            let mut response = Response::new(Full::new(Bytes::from(response_body.to_string())));
            response.headers_mut().insert(
                hyper::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            Ok(response)
        }
        _ => {
            let mut not_found = Response::new(Full::new(Bytes::from("404 Not Found")));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

/// Get HTTP status text for a status code
fn get_status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}
