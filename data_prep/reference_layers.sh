#!/bin/bash

set -e
set -x

function core_net {
  # From https://github.com/nptscot/npt/releases
  wget https://github.com/nptscot/npt/releases/download/CN_class/city_of_edinburgh_2024-10-01_4_coherent_network.geojson -O ../web/public/core_network.geojson
}

function schools {
  # From https://www.data.gov.uk/dataset/9a6f9d86-9698-4a5d-a2c8-89f3b212c52c/scottish-school-roll-and-locations
  wget https://maps.gov.scot/ATOM/shapefiles/SG_SchoolRoll_2023.zip
  unzip SG_SchoolRoll_2023.zip
  ogr2ogr ../web/public/schools.geojson \
          -t_srs EPSG:4326 \
          SG_SchoolRoll_2023/SG_SchoolRoll_2023.shp \
          -sql 'SELECT SchoolType as type, SchoolName as name, PupilRoll as pupils FROM SG_SchoolRoll_2023'
  rm -rf SG_SchoolRoll_2023 SG_SchoolRoll_2023.zip
}

core_net
schools
