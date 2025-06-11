#[cfg(target_os = "windows")]
use recaret::platform::windows;

#[cfg(target_os = "windows")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    windows::start().await
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("This example only runs on Windows.");
}
