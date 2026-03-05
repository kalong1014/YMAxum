// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! Configuration file watcher

use std::path::Path;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

/// Configuration watcher
pub struct ConfigWatcher {
    /// File path
    path: String,
    /// Callback function
    callback: Arc<dyn Fn() + Send + Sync>,
    /// Thread handle
    thread: Option<thread::JoinHandle<()>>,
    /// Stop channel
    stop_tx: Option<mpsc::Sender<()>>,
}

impl ConfigWatcher {
    /// Create new configuration watcher
    pub fn new(path: String, callback: Box<dyn Fn() + Send + Sync>) -> Self {
        let (stop_tx, stop_rx) = mpsc::channel();
        
        let path_clone = path.clone();
        let callback_arc = Arc::new(callback);
        let callback_clone = Arc::clone(&callback_arc);
        
        let thread = thread::spawn(move || {
            Self::watch_file(&path_clone, callback_clone, stop_rx);
        });
        
        Self {
            path,
            callback: callback_arc,
            thread: Some(thread),
            stop_tx: Some(stop_tx),
        }
    }

    /// Watch file for changes
    fn watch_file(path: &str, callback: Arc<dyn Fn() + Send + Sync>, stop_rx: mpsc::Receiver<()>) {
        let path = Path::new(path);
        let mut last_modified = match path.metadata() {
            Ok(meta) => meta.modified().unwrap(),
            Err(_) => return,
        };
        
        loop {
            // Check if we should stop
            if stop_rx.try_recv().is_ok() {
                break;
            }
            
            // Check if file has been modified
            if let Ok(meta) = path.metadata() {
                if let Ok(modified) = meta.modified() {
                    if modified > last_modified {
                        last_modified = modified;
                        (callback)();
                    }
                }
            }
            
            // Sleep for a short time
            thread::sleep(Duration::from_secs(1));
        }
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        // Send stop signal
        if let Some(stop_tx) = self.stop_tx.take() {
            let _ = stop_tx.send(());
        }
        
        // Join the thread
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl std::fmt::Debug for ConfigWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigWatcher")
            .field("path", &self.path)
            .finish()
    }
}

impl Clone for ConfigWatcher {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            callback: Arc::clone(&self.callback),
            thread: None, // Thread handles cannot be cloned
            stop_tx: None, // Channels cannot be cloned
        }
    }
}
