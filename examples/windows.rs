use recaret::platform::windows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    windows::start().await
}
