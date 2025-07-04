use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use xshell::{Shell, cmd};

use crate::Metrics;

pub struct CompileTime {
    pub example_name: String,
}

impl CompileTime {
    pub fn on(example_name: String) -> Self {
        Self {
            example_name: if example_name == "" {
                "breakout".to_string()
            } else {
                example_name
            },
        }
    }
}

impl Metrics for CompileTime {
    fn prepare(&self) {
        let command = format!("cargo build --release --example {}", self.example_name);
        let sh = Shell::new().unwrap();
        cmd!(
            sh,
            "hyperfine --export-json build.json --prepare 'cargo clean; sleep 2' {command}"
        )
        .run()
        .unwrap();
    }

    fn artifacts(&self) -> HashMap<String, PathBuf> {
        HashMap::from([(
            "compile-time.stats".to_string(),
            Path::new("build.json").to_path_buf(),
        )])
    }

    fn collect(&self) -> HashMap<String, u64> {
        let results: Hyperfine =
            serde_json::from_reader(std::fs::File::open("build.json").unwrap()).unwrap();
        HashMap::from([
            (
                "compile-time.mean".to_string(),
                (results.results[0].mean * 1000.0) as u64,
            ),
            (
                "compile-time.stddev".to_string(),
                (results.results[0].stddev.unwrap_or_default() * 1000.0) as u64,
            ),
            (
                "compile-time.median".to_string(),
                (results.results[0].median * 1000.0) as u64,
            ),
            (
                "compile-time.user".to_string(),
                (results.results[0].user * 1000.0) as u64,
            ),
            (
                "compile-time.system".to_string(),
                (results.results[0].system * 1000.0) as u64,
            ),
        ])
    }
}

#[derive(Deserialize)]
struct Hyperfine {
    results: Vec<HyperfineResults>,
}
#[derive(Deserialize)]
struct HyperfineResults {
    mean: f32,
    stddev: Option<f32>,
    median: f32,
    user: f32,
    system: f32,
}
