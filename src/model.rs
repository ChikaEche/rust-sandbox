use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{Error, Result};
use chrono::Utc;

#[derive(Clone, Serialize, Debug)]
pub struct StringEntry {
    pub id: u64,
    pub value: String,
    pub timestamp: u64,
}

#[derive(Deserialize, Clone)]
pub struct RequestData {
    pub value: String,
}

#[derive(Clone)]
pub struct StringEntryController {
    string_entry_store: Arc<Mutex<Vec<Option<StringEntry>>>>,
}

impl StringEntryController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            string_entry_store: Arc::default(),
        })
    }

    pub async fn create_entry(&self, data: RequestData) -> Result<StringEntry> {
        let mut store = self.string_entry_store.lock().unwrap();

        let id = store.len() as u64;
        let timestamp = Utc::now().timestamp_millis() as u64;
        let string_entry = StringEntry {
            id,
            timestamp,
            value: data.value,
        };

        store.push(Some(string_entry.clone()));

        Ok(string_entry)
    }

    pub async fn list_entry(&self) -> Result<Vec<StringEntry>> {
        let store = self.string_entry_store.lock().unwrap();
        let string_entries = store.iter().filter_map(|t| t.clone()).collect();
        Ok(string_entries)
    }

    pub async fn delete_entry(&self, id: u64) -> Result<StringEntry> {
        let mut store = self.string_entry_store.lock().unwrap();

        let string_entry = store.get_mut(id as usize).and_then(|t| t.take());

        string_entry.ok_or(Error::EntryNotFound { id })
    }
}
