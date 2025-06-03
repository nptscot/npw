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
        # Download England OSM data
        if [ ! -f england-latest.osm.pbf ]; then
          wget https://download.geofabrik.de/europe/united-kingdom/england-latest.osm.pbf
        fi

        # Generate config for osmium
        mkdir osm
        cd osm
        mkdir out
        python3 ../../geojson_to_osmium_extracts.py ../boundaries.geojson --output_dir=out/ --batch_size=10

        # Create an osm.pbf file per boundary
        for batch in osmium_cfg_*; do
          time osmium extract -v -c $batch ../england-latest.osm.pbf
        done

        cd ..
}

split_osm
