use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use plotters::prelude::*;
use twitcher::stats::{Stats, find_stats_files};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("graphs").unwrap();
    let stats: Vec<Stats> = find_stats_files(&Path::new("results"))
        .iter()
        .map(|path| {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        })
        .collect();

    let zero = stats.first().unwrap();
    for metric in zero.metrics.keys() {
        println!("Metric: {}", metric);
        let mut data = stats
            .iter()
            .map(|stat| {
                (
                    stat.commit_timestamp,
                    stat.commit.clone(),
                    stat.metrics[metric].clone(),
                )
            })
            .collect::<Vec<_>>();

        data.sort_by_key(|d| d.0);
        let min = data.iter().min_by_key(|d| d.2).unwrap().2;
        let max = data.iter().max_by_key(|d| d.2).unwrap().2;

        let out = format!("graphs/{}.svg", metric);

        let root = SVGBackend::new(&out, (1536, 512)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(100)
            .caption(metric, ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(data.first().unwrap().0..data.last().unwrap().0, min..max)?;

        chart.configure_mesh().light_line_style(WHITE).draw()?;

        chart
            .draw_series(LineSeries::new(data.iter().map(|x| (x.0, x.2)), BLUE))
            .unwrap();

        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", out);
    }

    Ok(())
}
