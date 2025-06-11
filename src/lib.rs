//! Cross-platform library to handle multiple carets and selections.
//! Provides an asynchronous API to manage carets and a global event listener
//! inspired by VS Code multi-caret behavior.

mod event_listener;
pub use event_listener::start as start_listener;
pub mod platform;

use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;

/// Identifier for a caret.
pub type CaretId = u64;

/// Represents a caret stored by the library.
#[derive(Clone, Debug, PartialEq)]
pub struct Caret {
    pub id: CaretId,
    pub x: u32,
    pub y: u32,
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
static CARETS: Lazy<Mutex<Vec<Caret>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Add a caret at the given coordinates.
pub async fn add_caret(x: u32, y: u32) -> CaretId {
    let mut carets = CARETS.lock().await;
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    carets.push(Caret { id, x, y });
    id
}

/// Remove a caret by id.
pub async fn remove_caret(id: CaretId) {
    let mut carets = CARETS.lock().await;
    carets.retain(|c| c.id != id);
}

/// List current carets.
pub async fn list_carets() -> Vec<Caret> {
    CARETS.lock().await.clone()
}

/// Clear all carets.
pub async fn clear_all_carets() {
    CARETS.lock().await.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn basic_ops() {
        clear_all_carets().await;
        let id = add_caret(10, 10).await;
        assert_eq!(list_carets().await.len(), 1);
        remove_caret(id).await;
        assert!(list_carets().await.is_empty());
    }
}
