#!/bin/bash
#
# This generates a graph file per LAD. You'll need some dependencies:
#
# - wget, python3
# - osmium (https://osmcode.org/osmium-tool)
# - cargo (https://www.rust-lang.org/tools/install)
# - optionally pueue to parallelize (https://github.com/Nukesor/pueue)
#   - you can also modify the code to import each boundary sequentially

set -e

function split_osm {
        # Download Scotland OSM data
        if [ ! -f scotland-latest.osm.pbf ]; then
          wget https://download.geofabrik.de/europe/united-kingdom/scotland-latest.osm.pbf
        fi

        # Generate config for osmium
        mkdir osm
        cd osm
        mkdir out
        python3 ../../geojson_to_osmium_extracts.py ../boundaries.geojson --output_dir=out/ --batch_size=10

        # Create an osm.pbf file per boundary
        for batch in osmium_cfg_*; do
          time osmium extract -v -c $batch ../scotland-latest.osm.pbf
        done

        cd ..
}

function build_graph_files {
        # Build the CLI
        cd ../cli
        cargo build --release
        cd ../data_prep
        bin=../cli/target/release/cli

        mkdir -p baseline_stats
        IFS=$'\n'
        for osm in osm/out/*; do
          geojson=$(basename $osm .osm.pbf).geojson
          out=$(basename $osm .osm.pbf).bin
          stats=$(basename $osm .osm.pbf).json
          task=$(pueue add --print-task-id --escape $bin --input "$osm" --boundary "osm/$geojson" --output "../web/public/areas/$out" --stats-output "baseline_stats/$stats")
          pueue add --after $task --escape gzip -f "../web/public/areas/$out"
        done

        # Manually wait for pueue to finish
}

#split_osm
build_graph_files

# There are some tricks to serving .gz files through cloudflare with the right
# header. Do not use rclone.
#
# Upload the files:
#
# aws s3 --profile cloudflare --endpoint-url https://FOO.r2.cloudflarestorage.com sync local_dir/ s3://bucket/remote_dir
#
# Then change their metadata:
#
# aws s3 --profile cloudflare --endpoint-url https://FOO.r2.cloudflarestorage.com cp s3://bucket/remote_dir/ s3://bucket/remote_dir/ --exclude '*' --include '*.bin.gz' --no-guess-mime-type --content-type="application/octet-stream" --content-encoding=gzip --metadata-directive="REPLACE" --recursive
