#!/bin/bash

AREA=$1
set -x
cargo run --release -- --input "../data_prep/osm/out/$AREA.osm.pbf" --boundary "../data_prep/osm/$AREA.geojson" --output "../web/public/areas/$AREA.bin"
