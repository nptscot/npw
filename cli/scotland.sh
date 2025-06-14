#!/bin/bash

AREA=$1
set -ex

cargo run --release -- \
  --country scotland \
  --input "../data_prep/scotland/osm/out/$AREA.osm.pbf" \
  --boundary "../data_prep/scotland/osm/$AREA.geojson" \
  --output "../web/public/scotland/areas/$AREA.bin" \
  --stats-output "../data_prep/scotland/baseline_stats/$AREA.json"

gzip -f "../web/public/scotland/areas/$AREA.bin"
