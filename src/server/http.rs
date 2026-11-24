//! HTTP/1.1 server on pure tokio.

use serde_json::Value;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub struct Response {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

impl Response {
    pub fn json(v: &Value) -> Self {
        Self {
            status: 200,
            content_type: "application/json",
            body: serde_json::to_vec(v).unwrap_or_default(),
        }
    }
    pub fn text(s: &str) -> Self {
        Self {
            status: 200,
            content_type: "text/plain; charset=utf-8",
            body: s.as_bytes().to_vec(),
        }
    }
    pub fn not_found() -> Self {
        Self { status: 404, content_type: "text/plain", body: b"not found".to_vec() }
    }
}

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, req: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>;
}

pub async fn serve<H: Handler>(addr: &str, h: Arc<H>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (sock, _) = listener.accept().await?;
        let h = h.clone();
        tokio::spawn(async move {
            let _ = handle_conn(sock, h).await;
        });
    }
}

async fn handle_conn<H: Handler>(sock: TcpStream, h: Arc<H>) -> std::io::Result<()> {
    let (r, mut w) = sock.into_split();
    let mut reader = BufReader::new(r);

    let mut line = String::new();
    reader.read_line(&mut line).await?;
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();

    let mut headers = Vec::new();
    let mut content_length = 0usize;
    loop {
        let mut l = String::new();
        reader.read_line(&mut l).await?;
        if l == "\r\n" || l == "\n" || l.is_empty() { break; }
        if let Some((k, v)) = l.split_once(':') {
            let (k, v) = (k.trim().to_string(), v.trim().to_string());
            if k.eq_ignore_ascii_case("content-length") {
                content_length = v.parse().unwrap_or(0);
            }
            headers.push((k, v));
        }
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body).await?;
    }

    let req = Request { method, path, headers, body };
    let resp = h.handle(req).await;

    let reason = match resp.status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "OK",
    };
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        resp.status, reason, resp.content_type, resp.body.len()
    );
    w.write_all(head.as_bytes()).await?;
    w.write_all(&resp.body).await?;
    w.shutdown().await?;
    Ok(())
}