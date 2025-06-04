use std::io::BufWriter;

use anyhow::{bail, Result};
use clap::Parser;
use fs_err::File;
use graph::Timer;

mod common;
mod disconnected;
mod england;
mod scotland;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    country: String,

    /// Path to a .osm.pbf or .xml file to convert
    #[arg(long)]
    input: String,

    /// Path to GeoJSON file with the boundary to clip the input to
    #[arg(long)]
    boundary: String,

    /// Map model output file to write
    #[arg(long)]
    output: String,

    /// Baseline stats output file to write
    #[arg(long)]
    stats_output: String,
}

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = Args::parse();

    let mut timer = Timer::new("build model", None);
    let osm_bytes = fs_err::read(&args.input)?;
    let boundary_gj = fs_err::read_to_string(&args.boundary)?;
    let study_area_name = args
        .output
        .split("/")
        .last()
        .unwrap()
        .strip_suffix(".bin")
        .unwrap()
        .to_string();
    let model = match args.country.as_ref() {
        "scotland" => scotland::create(study_area_name, &osm_bytes, &boundary_gj, &mut timer)?,
        "england" => england::create(study_area_name, &osm_bytes, &boundary_gj, &mut timer)?,
        x => bail!("Unknown country {x}"),
    };

    timer.step("writing");
    let writer = BufWriter::new(File::create(&args.output)?);
    bincode::serialize_into(writer, &model)?;

    fs_err::write(
        &args.stats_output,
        serde_json::to_string(&model.get_baseline_stats())?,
    )?;

    timer.done();
    Ok(())
}
