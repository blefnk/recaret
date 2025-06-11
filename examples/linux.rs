use recaret::platform::linux;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    linux::start().await
}
