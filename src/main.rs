use std::{
    collections::HashMap,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use clap::{Parser, Subcommand};
use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};
use xshell::{Shell, cmd};

mod binary_size;
mod compile_time;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Commit to run on. If ommitted, run on the already checked out commit
    #[arg(short, long)]
    commit: Option<String>,

    /// Target folder for results
    #[arg(short, long, default_value = "results")]
    out: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, EnumIter)]
enum Commands {
    BinarySize {
        #[arg(short, long, default_value = "breakout")]
        example: String,
    },
    CompileTime {
        #[arg(short, long, default_value = "breakout")]
        example: String,
    },
    All,
}

impl Commands {
    fn to_metrics(self, recur: bool) -> Vec<Box<dyn Metrics>> {
        match self {
            Commands::BinarySize { example } => {
                vec![Box::new(binary_size::BinarySize::on(example))]
            }
            Commands::CompileTime { example } => {
                vec![Box::new(compile_time::CompileTime::on(example))]
            }
            Commands::All => {
                if recur {
                    Commands::iter()
                        .map(|command| command.to_metrics(false))
                        .flatten()
                        .collect()
                } else {
                    vec![]
                }
            }
        }
    }
}

trait Metrics {
    fn prepare(&self);
    fn artifacts(&self) -> HashMap<String, PathBuf> {
        HashMap::new()
    }
    fn collect(&self) -> HashMap<String, u64>;
}

fn main() {
    let cli = Cli::parse();

    let commit = if let Some(commit) = cli.commit {
        let sh = Shell::new().unwrap();
        cmd!(sh, "git checkout {commit}").run().unwrap();
        commit
    } else {
        let sh = Shell::new().unwrap();
        let out = cmd!(sh, "git rev-parse HEAD").output().unwrap();
        let mut output = out.stdout;
        output.pop();
        String::from_utf8(output).unwrap()
    };
    let commit_timestamp = {
        let sh = Shell::new().unwrap();
        let out = cmd!(sh, "git show --no-patch --format=%ct HEAD")
            .output()
            .unwrap();
        let mut output = out.stdout;
        output.pop();
        String::from_utf8(output).unwrap().parse::<u128>().unwrap() * 1000
    };

    let metrics_to_run = cli.command.to_metrics(true);

    let output_prefix = Path::new(&cli.out)
        .join(commit.chars().nth(0).unwrap().to_string())
        .join(commit.chars().nth(1).unwrap().to_string())
        .join(&commit);

    let metrics: HashMap<String, u64> = metrics_to_run
        .iter()
        .flat_map(|m| {
            m.prepare();
            for (save_as, file_name) in m.artifacts() {
                let target_folder = output_prefix.join(save_as);
                std::fs::create_dir_all(&target_folder).unwrap();
                std::fs::copy(file_name.clone(), target_folder.join(file_name)).unwrap();
            }
            m.collect()
        })
        .collect();

    let file = File::create(output_prefix.join("stats.json")).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(
        &mut writer,
        &Stats {
            metrics,
            commit,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            commit_timestamp,
        },
    )
    .unwrap();
}

#[derive(Serialize)]
struct Stats {
    metrics: HashMap<String, u64>,
    commit: String,
    timestamp: u128,
    commit_timestamp: u128,
}
