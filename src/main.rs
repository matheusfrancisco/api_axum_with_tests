use web_api_rust_learn::App;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT")
        .expect("Missing env PORT")
        .parse::<u16>()
        .expect("PORT must be a number");
    println!("PORT: {}", port);

    let app = App::new(port);
    app.run().await.expect("Failed to run app");
}
