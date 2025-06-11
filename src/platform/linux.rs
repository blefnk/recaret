use crate::event_listener;

/// Start global event listener on Linux.
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    event_listener::start().await
}
