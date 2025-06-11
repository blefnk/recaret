use recaret::platform::macos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    macos::start().await
}
