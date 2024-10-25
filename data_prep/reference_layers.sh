#!/bin/bash

set -e
set -x

function core_net {
  # From https://github.com/nptscot/npt/releases
  wget https://github.com/nptscot/outputdata/releases/download/v2024-10-01/combined_CN_4_2024-10-01_OS.pmtiles -O ../web/public/core_network.pmtiles
}

function schools {
  # From https://www.data.gov.uk/dataset/9a6f9d86-9698-4a5d-a2c8-89f3b212c52c/scottish-school-roll-and-locations
  wget https://maps.gov.scot/ATOM/shapefiles/SG_SchoolRoll_2023.zip
  unzip SG_SchoolRoll_2023.zip
  ogr2ogr ../web/public/schools.geojson \
          -t_srs EPSG:4326 \
          SG_SchoolRoll_2023/SG_SchoolRoll_2023.shp \
          -sql 'SELECT SchoolType AS type, SchoolName AS name, PupilRoll AS pupils FROM SG_SchoolRoll_2023'
  rm -rf SG_SchoolRoll_2023 SG_SchoolRoll_2023.zip
}

function town_centres {
  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/town_centres-is
  ogr2ogr town_centres.geojson \
          -t_srs EPSG:4326 \
          $1 \
          -sql 'SELECT site_name AS name FROM "Town_Centres_-_Scotland"'
  tippecanoe --drop-densest-as-needed --generate-ids -zg town_centres.geojson -o ../web/public/town_centres.pmtiles
  rm -f town_centres.geojson
}

function gp_and_hospitals {
  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/gp_practices-is
  ogr2ogr ../web/public/gp_practices.geojson \
          -t_srs EPSG:4326 \
          $1 \
          -sql 'SELECT address AS name FROM "GP_Practices_-_Scotland"'

  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/gp_practices-i://data.spatialhub.scot/dataset/nhs_hospitals-is
  ogr2ogr ../web/public/hospitals.geojson \
          -t_srs EPSG:4326 \
          $2 \
          -sql 'SELECT sitename AS name FROM "NHS_Hospitals_-_Scotland"'
  # TODO Clean up bboxes and IDs from both

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

core_net
schools
town_centres ~/Downloads/Town_Centres_-_Scotland.json
gp_and_hospitals ~/Downloads/GP_Practices_-_Scotland.json ~/Downloads/NHS_Hospitals_-_Scotland.json
urban_rural
