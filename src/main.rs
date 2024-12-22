use web_api_rust_learn::App;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT")
        .expect("Missing env PORT")
        .parse::<u16>()
        .expect("PORT must be a number");
    println!("PORT: {}", port);
    let database_uri = std::env::var("DATABASE_URL").expect("Missing env DATABASE_URL");

    let app = App::new(port, &database_uri)
        .await
        .expect("Failed to create app");

    app.run().await.expect("Failed to run app");
}
