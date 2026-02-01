use crate::store::{Record, RuntimeConfig, Store};
use crate::types::ExecutionId;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
struct MemoryStoreData {
    records: HashMap<ExecutionId, Record>,
    latest_id: Option<ExecutionId>,
}

#[derive(Debug, Clone)]
pub struct MemoryStore {
    data: Arc<RwLock<MemoryStoreData>>,
    runtime_config: Arc<RwLock<Option<RuntimeConfig>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(MemoryStoreData {
                records: HashMap::new(),
                latest_id: None,
            })),
            runtime_config: Arc::new(RwLock::new(None)),
        }
    }
}

impl Store for MemoryStore {
    fn add_record(&mut self, record: Record) -> Result<()> {
        if let Ok(mut data) = self.data.write() {
            data.latest_id = Some(record.id);
            data.records.insert(record.id, record);
        }
        Ok(())
    }

    fn get_record(&self, id: ExecutionId) -> Result<Option<Record>> {
        Ok(match self.data.read() { Ok(data) => {
            data.records.get(&id).cloned()
        } _ => {
            None
        }})
    }

    fn get_latest_id(&self) -> Result<Option<ExecutionId>> {
        Ok(match self.data.read() { Ok(data) => {
            data.latest_id
        } _ => {
            None
        }})
    }

    fn get_records(&self) -> Result<Vec<Record>> {
        Ok(match self.data.read() { Ok(data) => {
            data.records.values().cloned().collect()
        } _ => {
            vec![]
        }})
    }

    fn get_runtime_config(&self) -> Result<Option<RuntimeConfig>> {
        Ok(match self.runtime_config.read() { Ok(runtime_config) => {
            runtime_config.clone()
        } _ => {
            None
        }})
    }

    fn set_runtime_config(&mut self, config: RuntimeConfig) -> Result<()> {
        if let Ok(mut runtime_config) = self.runtime_config.write() {
            *runtime_config = Some(config);
        }
        Ok(())
    }
}
