use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;
use tokio::sync::mpsc::unbounded_channel;

use crate::add_caret;

static PRESSED_KEYS: Lazy<Mutex<HashSet<rdev::Key>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static LAST_POS: Lazy<Mutex<(u32, u32)>> = Lazy::new(|| Mutex::new((0, 0)));

/// Starts listening to global mouse and keyboard events.
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = unbounded_channel();
    std::thread::spawn(move || {
        if let Err(e) = rdev::listen(move |event| {
            let _ = tx.send(event);
        }) {
            eprintln!("listen error: {:?}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        handle_event(event).await;
    }
    Ok(())
}

async fn handle_event(event: rdev::Event) {
    use rdev::{Button, EventType, Key};
    match event.event_type {
        EventType::KeyPress(key) => {
            PRESSED_KEYS.lock().unwrap().insert(key);
        }
        EventType::KeyRelease(key) => {
            PRESSED_KEYS.lock().unwrap().remove(&key);
        }
        EventType::MouseMove { x, y } => {
            *LAST_POS.lock().unwrap() = (x as u32, y as u32);
        }
        EventType::ButtonPress(Button::Left) => {
            let keys = PRESSED_KEYS.lock().unwrap();
            let pos = *LAST_POS.lock().unwrap();
            if keys.contains(&Key::Alt) {
                let _ = add_caret(pos.0, pos.1).await;
            } else if keys.contains(&Key::ControlLeft) || keys.contains(&Key::ControlRight) {
                let _ = add_caret(pos.0, pos.1).await;
            }
        }
        _ => {}
    }
}
