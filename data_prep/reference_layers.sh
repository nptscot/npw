#!/bin/bash

set -e
set -x
mkdir -p tmp

function core_net {
  # From https://github.com/nptscot/outputdata/releases
  wget https://github.com/nptscot/outputdata/releases/download/v2024-10-01/core_network.gpkg -O tmp/core_network.gpkg
}

function rnet {
  # From https://github.com/nptscot/outputdata/releases

  # For visualization
  wget https://github.com/nptscot/outputdata/releases/download/v2024-10-01/rnet_2024-10-01.pmtiles -O ../web/public/route_network.pmtiles

  # For analysis
  wget https://github.com/nptscot/outputdata/releases/download/v2024-10-01/combined_network.gpkg -O tmp/combined_network.gpkg
}

function schools {
  # From https://www.data.gov.uk/dataset/9a6f9d86-9698-4a5d-a2c8-89f3b212c52c/scottish-school-roll-and-locations
  wget https://maps.gov.scot/ATOM/shapefiles/SG_SchoolRoll_2023.zip
  unzip SG_SchoolRoll_2023.zip
  ogr2ogr tmp/schools.geojson \
          -t_srs EPSG:4326 \
          SG_SchoolRoll_2023/SG_SchoolRoll_2023.shp \
          -sql 'SELECT SchoolType AS type, SchoolName AS name, PupilRoll AS pupils FROM SG_SchoolRoll_2023'
  rm -rf SG_SchoolRoll_2023 SG_SchoolRoll_2023.zip
}

function town_centres {
  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/town_centres-is
  ogr2ogr tmp/town_centres.geojson \
          -t_srs EPSG:4326 \
          $1 \
          -nlt PROMOTE_TO_MULTI \
          -sql 'SELECT site_name AS name FROM "Town_Centres_-_Scotland"'

  # The bboxes or something else included are breaking parsing, so clean these up
  jq '{ type: "FeatureCollection", features: [.features[] | { type: "Feature", geometry: .geometry, properties: { name: .properties.name } }] }' tmp/town_centres.geojson > tmp.gj
  mv -f tmp.gj tmp/town_centres.geojson
}

function gp_and_hospitals {
  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/gp_practices-is
  ogr2ogr tmp/gp_practices.geojson \
          -t_srs EPSG:4326 \
          $1 \
          -sql 'SELECT address AS name FROM "GP_Practices_-_Scotland"'

  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/gp_practices-i://data.spatialhub.scot/dataset/nhs_hospitals-is
  ogr2ogr tmp/hospitals.geojson \
          -t_srs EPSG:4326 \
          $2 \
          -sql 'SELECT sitename AS name FROM "NHS_Hospitals_-_Scotland"'

  # The bboxes or something else included are breaking parsing, so clean these up
  jq '{ type: "FeatureCollection", features: [.features[] | { type: "Feature", geometry: .geometry, properties: { name: .properties.name } }] }' tmp/gp_practices.geojson > tmp.gj
  mv -f tmp.gj tmp/gp_practices.geojson

  jq '{ type: "FeatureCollection", features: [.features[] | { type: "Feature", geometry: .geometry, properties: { name: .properties.name } }] }' tmp/hospitals.geojson > tmp.gj
  mv -f tmp.gj tmp/hospitals.geojson

  # TODO Consider combining
}

function urban_rural {
  # From https://www.data.gov.uk/dataset/f00387c5-7858-4d75-977b-bfdb35300e7f/urban-rural-classification-scotland
  wget https://maps.gov.scot/ATOM/shapefiles/SG_UrbanRural_2020.zip
  unzip SG_UrbanRural_2020.zip
  # Only keep urban areas, and assume anything else is rural -- it's mostly rural
  ogr2ogr urban_areas.geojson -t_srs EPSG:4326 SG_UrbanRural_2020/SG_UrbanRural_2020.shp -sql 'SELECT UR2Class FROM SG_UrbanRural_2020 WHERE UR2Class = 1' -explodecollections
  tippecanoe --drop-densest-as-needed --generate-ids -zg urban_areas.geojson -o ../web/public/urban_areas.pmtiles
  rm -rf SG_UrbanRural_2020 SG_UrbanRural_2020.zip urban_areas.geojson
}

function od_and_zones {
  # Manually download https://github.com/nptscot/inputdata/releases/download/v1/desire_lines_scotland.csv from internal GH repo
  xsv select geo_code1,geo_code2,all $1 > tmp/od.csv

  # From https://spatialdata.gov.scot/geonetwork/srv/api/records/389787c0-697d-4824-9ca9-9ce8cb79d6f5
  wget https://maps.gov.scot/ATOM/shapefiles/SG_IntermediateZoneBdry_2011.zip
  unzip SG_IntermediateZoneBdry_2011.zip
  ogr2ogr tmp/zones.geojson -t_srs EPSG:4326 SG_IntermediateZone_Bdry_2011.shp -sql 'SELECT InterZone FROM SG_IntermediateZone_Bdry_2011'
  rm -f SG_IntermediateZone_Bdry_2011*
}

function traffic {
  # Manually download https://github.com/nptscot/scottraffic/releases/download/v6/final_estimates_Scotland.gpkg from internal GH repo
  cp $1 tmp/traffic.gpkg
}

function simd {
  # From https://www.data.gov.uk/dataset/1102bf85-ed49-440a-b211-da87e8d752eb/scottish-index-of-multiple-deprivation-simd-2020
  wget https://maps.gov.scot/ATOM/shapefiles/SG_SIMD_2020.zip
  unzip SG_SIMD_2020.zip
  ogr2ogr tmp/simd.geojson \
          -t_srs EPSG:4326 \
          SG_SIMD_2020.shp \
          -nlt PROMOTE_TO_MULTI \
          -sql 'SELECT DataZone, Rankv2 as rank, Percentv2 as percentile, SAPE2017 as population FROM SG_SIMD_2020 WHERE Quintilev2 = 1'
  rm -f SG_SIMD_2020* SIMD2020v2*xlsx
}

core_net
rnet
schools
town_centres ~/Downloads/Town_Centres_-_Scotland.json
gp_and_hospitals ~/Downloads/GP_Practices_-_Scotland.json ~/Downloads/NHS_Hospitals_-_Scotland.json
urban_rural
od_and_zones ~/Downloads/desire_lines_scotland.csv
traffic ~/Downloads/final_estimates_Scotland.gpkg
simd