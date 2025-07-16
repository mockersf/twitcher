use std::{
    collections::HashMap,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use xshell::{Shell, cmd};

use crate::Metrics;

pub struct CrateCompileTime {
    pub nb_jobs: u32,
}

impl CrateCompileTime {
    pub fn on(nb_jobs: u32) -> Self {
        Self {
            nb_jobs: if nb_jobs == 0 { 16 } else { nb_jobs },
        }
    }
}

impl Metrics for CrateCompileTime {
    fn prepare(&self) {
        let nb_jobs = format!("{}", self.nb_jobs);
        let sh = Shell::new().unwrap();
        cmd!(
            sh,
            "cargo +nightly build --jobs {nb_jobs} --release -Z unstable-options --timings=json"
        )
        .run()
        .unwrap();

        let mut timings: HashMap<String, Vec<CrateTiming>> = HashMap::new();

        for _ in 0..10 {
            let sh = Shell::new().unwrap();
            cmd!(sh, "cargo clean").run().unwrap();
            let out = cmd!(
                sh,
                "cargo +nightly build --jobs {nb_jobs} --release -Z unstable-options --timings=json"
            )
            .read()
            .unwrap();
            out.lines()
                .map(|line| serde_json::from_str::<TimingInfo>(line).unwrap())
                .filter(|info| info.package_id.starts_with("path"))
                .for_each(|info| {
                    timings
                        .entry(info.target.name)
                        .or_default()
                        .push(CrateTiming {
                            duration: info.duration,
                            rmeta_time: info.rmeta_time.unwrap_or_default(),
                        })
                });
        }

        let file = File::create(format!("crate-stats-{}.json", self.nb_jobs)).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &timings).unwrap();
    }

    fn artifacts(&self) -> HashMap<String, PathBuf> {
        HashMap::from([(
            "crate-compile-time.stats".to_string(),
            Path::new(&format!("crate-stats-{}.json", self.nb_jobs)).to_path_buf(),
        )])
    }

    fn collect(&self) -> HashMap<String, u64> {
        let key = format!(
            "crate-compile-time-{}-{}-{}",
            std::env::consts::FAMILY,
            std::env::consts::ARCH,
            self.nb_jobs
        );

        let timings: HashMap<String, Vec<CrateTiming>> = serde_json::from_reader(
            std::fs::File::open(&format!("crate-stats-{}.json", self.nb_jobs)).unwrap(),
        )
        .unwrap();
        timings
            .iter()
            .flat_map(|(crate_name, timings)| {
                let durations: Vec<f64> = timings.iter().map(|timing| timing.duration).collect();
                let rmeta_times: Vec<f64> =
                    timings.iter().map(|timing| timing.rmeta_time).collect();
                statistical::mean(&durations);
                vec![
                    (
                        format!("{}.{}.mean", key, crate_name),
                        (statistical::mean(&durations) * 1000.0) as u64,
                    ),
                    (
                        format!("{}.{}.median", key, crate_name),
                        (statistical::median(&durations) * 1000.0) as u64,
                    ),
                    (
                        format!("{}.{}.min", key, crate_name),
                        (durations.iter().map(|d| (d * 1000.0) as u64).min().unwrap()),
                    ),
                    (
                        format!("{}.{}.max", key, crate_name),
                        (durations.iter().map(|d| (d * 1000.0) as u64).max().unwrap()),
                    ),
                    (
                        format!("{}.{}.std_dev", key, crate_name),
                        (statistical::standard_deviation(&durations, None) * 1000.0) as u64,
                    ),
                    (
                        format!("{}.{}.rmeta-mean", key, crate_name),
                        (statistical::mean(&rmeta_times) * 1000.0) as u64,
                    ),
                    (
                        format!("{}.{}.rmeta-median", key, crate_name),
                        (statistical::median(&rmeta_times) * 1000.0) as u64,
                    ),
                    (
                        format!("{}.{}.rmeta-min", key, crate_name),
                        (rmeta_times
                            .iter()
                            .map(|d| (d * 1000.0) as u64)
                            .min()
                            .unwrap()),
                    ),
                    (
                        format!("{}.{}.rmeta-max", key, crate_name),
                        (rmeta_times
                            .iter()
                            .map(|d| (d * 1000.0) as u64)
                            .max()
                            .unwrap()),
                    ),
                    (
                        format!("{}.{}.rmeta-std_dev", key, crate_name),
                        (statistical::standard_deviation(&rmeta_times, None) * 1000.0) as u64,
                    ),
                ]
            })
            .collect()
    }
}

#[derive(Deserialize, Debug)]
struct TimingInfo {
    package_id: String,
    target: Target,
    duration: f64,
    rmeta_time: Option<f64>,
}
#[derive(Deserialize, Debug)]
struct Target {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CrateTiming {
    duration: f64,
    rmeta_time: f64,
}
