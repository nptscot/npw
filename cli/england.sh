#!/bin/bash

AREA=$1
set -ex

cargo run --release -- \
  --input "../data_prep/england/osm/out/$AREA.osm.pbf" \
  --boundary "../data_prep/england/osm/$AREA.geojson" \
  --output "../web/public/england/areas/$AREA.bin" \
  --stats-output "../data_prep/england/baseline_stats/$AREA.json"

gzip -f "../web/public/england/areas/$AREA.bin"
