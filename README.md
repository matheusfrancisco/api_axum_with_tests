# Another Web Api for learn RUST

A post api for learn rust, using the axum framework.



## Tech

Those are the main dependencies used in this project:

- axum = "0.7.9"
- dotenvy = "0.15.7"
- eyre = "0.6.9"
- serde = { version = "1.0.193", features = ["derive"] }
- sqlx = { version = "0.7.3", features = ["postgres", "runtime-tokio-rustls"] }
- tokio = { version = "1.34.0", features = ["net", "rt-multi-thread", "macros"] }
- tower-http = { version = "0.5.0", features = ["cors", "trace"] }
- tracing = "0.1.40"
- tracing-subscriber = "0.3.18"

## Setup
1. Create the dotenv file by copying .env_example to .env

## Database

A docker compose file is included to spin up a postgres database for development. To start the database run:

```bash
docker-compose up -d
```
## Connecting into the Database
We can connect to the database using the following command:

```bash
docker compose exec database psql -U postgres
```
## Models

Posts
