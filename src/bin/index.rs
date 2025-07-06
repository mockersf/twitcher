use std::{fs::File, io::BufReader, path::Path};

use tera::Tera;
use twitcher::stats::{Stats, find_stats_files};

fn main() {
    let tera = Tera::new("templates/*").unwrap();
    // Prepare the context with some data
    let mut context = tera::Context::new();

    let stats: Stats = find_stats_files(&Path::new("results"))
        .first()
        .map(|path| {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        })
        .unwrap();
    let mut metrics = stats.metrics.keys().cloned().collect::<Vec<_>>();
    metrics.sort();
    context.insert("metrics", &metrics);

    // Render the template with the given context
    let rendered = tera.render("index.html", &context).unwrap();
    std::fs::write("./index.html", &rendered).unwrap();
}
