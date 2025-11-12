use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Transaction {
    CreateItem { name: String, price: f64, quantity: i32 },
    UpdateItem { id: String, name: Option<String>, price: Option<f64>, quantity: Option<i32> },
    DeleteItem { id: String },
    CreateSale { items: Vec<String>, total: f64 },
}

pub struct SyncQueue {
    queue: VecDeque<Transaction>,
    file_path: String,
}

impl SyncQueue {
    pub fn new(file_path: &str) -> Self {
        let queue = if Path::new(file_path).exists() {
            let data = fs::read_to_string(file_path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            VecDeque::new()
        };
        Self {
            queue,
            file_path: file_path.to_string(),
        }
    }

    pub fn enqueue(&mut self, transaction: Transaction) {
        self.queue.push_back(transaction);
        self.save();
    }

    pub fn dequeue(&mut self) -> Option<Transaction> {
        let tx = self.queue.pop_front();
        if tx.is_some() {
            self.save();
        }
        tx
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn save(&self) {
        let data = serde_json::to_string(&self.queue).unwrap();
        fs::write(&self.file_path, data).unwrap();
    }
}
