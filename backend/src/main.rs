use std::fs::File;
use std::io::BufWriter;

use anyhow::Result;
use graph::Timer;

use backend::MapModel;

fn main() -> Result<()> {
    // TODO simple logger (but split crates, probably)
    let args: Vec<String> = std::env::args().collect();
    let mut timer = Timer::new("build model", None);
    let osm_bytes = std::fs::read(&args[1])?;
    let model = MapModel::create(&osm_bytes, &mut timer)?;

    timer.step("Writing");
    let writer = BufWriter::new(File::create("../web/public/model.bin")?);
    bincode::serialize_into(writer, &model)?;

    timer.done();
    Ok(())
}
