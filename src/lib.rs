use std::{collections::HashMap, path::PathBuf};

pub mod binary_size;
pub mod compile_time;

pub trait Metrics {
    fn prepare(&self);
    fn artifacts(&self) -> HashMap<String, PathBuf> {
        HashMap::new()
    }
    fn collect(&self) -> HashMap<String, u64>;
}
