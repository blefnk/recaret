#[cfg(target_os = "linux")]
use recaret::platform::linux;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    linux::start().await
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("This example only runs on Linux.");
}
