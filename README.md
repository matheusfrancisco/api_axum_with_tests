# Another Web Api for learn RUST

A post api for learn rust, using the axum framework.



## Tech

- axum = "0.7.9"
- dotenvy = "0.15.7"
- eyre = "0.6.9"
- serde = { version = "1.0.193", features = ["derive"] }
- sqlx = { version = "0.7.3", features = ["postgres", "runtime-tokio-rustls"] }
- tokio = { version = "1.34.0", features = ["net", "rt-multi-thread", "macros"] }
- tower-http = { version = "0.5.0", features = ["cors", "trace"] }
- tracing = "0.1.40"
- tracing-subscriber = "0.3.18"
