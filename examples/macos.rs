#[cfg(target_os = "macos")]
use recaret::platform::macos;

#[cfg(target_os = "macos")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    macos::start().await
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This example only runs on macOS.");
}
