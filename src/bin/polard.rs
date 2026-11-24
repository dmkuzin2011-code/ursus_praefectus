//! polard — main daemon: tick loop + HTTP + safety.

use std::sync::Arc;
use tokio::time::{interval, Duration};
use ursus_praefectus as up;

struct DaemonState {
    limits: up::safety::SafetyLimits,
    tick_count: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Lua sandbox
    let lua = mlua::Lua::new();
    up::lua::setup_sandbox(&lua)?;

    // 2. Shared state
    let state = Arc::new(DaemonState {
        limits: up::safety::SafetyLimits::default(),
        tick_count: 0,
    });

    // 3. HTTP server (in background) — placeholder handler
    let http_state = state.clone();
    tokio::spawn(async move {
        struct Placeholder(Arc<DaemonState>);
        impl up::http::Handler for Placeholder {
            fn handle(&self, req: up::http::Request)
                -> std::pin::Pin<Box<dyn std::future::Future<Output = up::http::Response> + Send>>
            {
                let s = self.0.clone();
                Box::pin(async move {
                    match (req.method.as_str(), req.path.as_str()) {
                        ("GET", "/status") => {
                            up::http::Response::json(&serde_json::json!({
                                "ok": true,
                                "ticks": s.tick_count,
                                "max_speed": s.limits.max_speed_mps,
                            }))
                        }
                        _ => up::http::Response::not_found(),
                    }
                })
            }
        }
        let _ = up::http::serve("0.0.0.0:8080", Arc::new(Placeholder(http_state))).await;
    });

    // 4. Main tick loop
    let mut ticker = interval(Duration::from_millis(100));
    loop {
        ticker.tick().await;

        // collect_ctx: placeholder sensor data
        let ctx = serde_json::json!({
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_