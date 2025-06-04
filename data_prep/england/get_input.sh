#!/bin/bash

set -ex
mkdir -p tmp
mkdir -p inputs
cd tmp

function od {
  # From https://www.nomisweb.co.uk/sources/census_2021_od
  wget https://www.nomisweb.co.uk/output/census/2021/odwp01ew.zip
  unzip odwp01ew.zip

  # Filter for OD pairs starting and ending in England
  xan filter 'startswith(col("Output Areas code"), "E") && startswith(col("OA of workplace code"), "E")' ODWP01EW_OA.csv | \
    xan rename zone1,zone2,count -s 'Output Areas code,OA of workplace code,Count' \
    | xan select zone1,zone2,count > ../inputs/od.csv
}

function zones {
  # https://github.com/dabreegster/uk-boundaries has already preprocessed 2021 output areas
  wget https://github.com/dabreegster/uk-boundaries/raw/main/2021_output_areas.geojson.gz
  gunzip 2021_output_areas.geojson.gz
  ogr2ogr raw_zones.geojson -nlt PROMOTE_TO_MULTI -sql 'SELECT OA21CD AS name FROM "2021_output_areas"' 2021_output_areas.geojson

  # Separately grab population density data
  wget https://www.nomisweb.co.uk/output/census/2021/census2021-ts006.zip
  unzip census2021-ts006.zip census2021-ts006-oa.csv

  # Join the data
  mapshaper raw_zones.geojson -join census2021-ts006-oa.csv keys=name,"geography code" fields="Population Density: Persons per square kilometre; measures: Value" -rename-fields population="Population Density: Persons per square kilometre; measures: Value" -o zones_with_population.geojson

  # Mapshaper gets rid of MultiPolygons; coerce back
  ogr2ogr ../inputs/zones.geojson -nlt PROMOTE_TO_MULTI zones_with_population.geojson
}

function elevation {
  # TODO Switch to https://osdatahub.os.uk/downloads/open/Terrain50 directly
  wget https://play.abstreet.org/dev/data/input/shared/elevation/UK-dem-50m-4326.tif.gz
  gunzip UK-dem-50m-4326.tif.gz
  mv UK-dem-50m-4326.tif ../inputs/
}

#od
#zones
#elevation
