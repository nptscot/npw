#!/bin/bash

set -e
set -x
mkdir -p tmp

function core_net {
  # From https://github.com/nptscot/outputdata/releases
  wget https://github.com/nptscot/outputdata/releases/download/v2025-03-01/combined_CN_4_2025-03-01_OS.gpkg -O tmp/core_network.gpkg
  # While we're still comparing map-matched results, include this too
  wget https://github.com/nptscot/outputdata/releases/download/v2025-03-01/combined_CN_4_2025-03-01_OS.pmtiles -O ../web/public/core_network.pmtiles
}

function rnet {
  # From https://github.com/nptscot/outputdata/releases

  # For visualization
  wget https://github.com/nptscot/outputdata/releases/download/v2025-03-01/rnet_2025-03-01.pmtiles -O ../web/public/route_network.pmtiles

  # For analysis
  wget https://github.com/nptscot/outputdata/releases/download/v2025-03-01/combined_network.gpkg -O tmp/combined_network.gpkg
}

function streetspace {
  # For visualization
  wget https://github.com/nptscot/outputdata/releases/download/v2025-03-01/os_networks_categorized_street_space_with_widths.pmtiles -O ../web/public/streetspace.pmtiles

  # For analysis
  wget https://github.com/nptscot/outputdata/releases/download/v2025-03-01/os_networks_categorized_street_space_with_widths.gpkg -O tmp/streetspace.gpkg
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
          -explodecollections \
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

  # Manually register and download GeoJSON from https://data.spatialhub.scot/dataset/nhs_hospitals-is
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
  # Manually download https://github.com/nptscot/scottraffic/releases/download/v7/final_estimates_Scotland_20241202_crs4326.gpkg from internal GH repo
  cp $1 tmp/traffic.gpkg

  # While we're still comparing map-matched results, include this too
  wget https://nptscot.blob.core.windows.net/pmtiles/cbd_layer_2024-12-01.pmtiles -O ../web/public/cbd.pmtiles
}

function population {
  # From https://www.data.gov.uk/dataset/1102bf85-ed49-440a-b211-da87e8d752eb/scottish-index-of-multiple-deprivation-simd-2020
  wget https://maps.gov.scot/ATOM/shapefiles/SG_SIMD_2020.zip
  unzip SG_SIMD_2020.zip
  ogr2ogr tmp/population.geojson \
          -t_srs EPSG:4326 \
          SG_SIMD_2020.shp \
          -nlt PROMOTE_TO_MULTI \
          -sql 'SELECT DataZone, Rankv2 as rank, Percentv2 as percentile, SAPE2017 as population, OGR_GEOM_AREA as area FROM SG_SIMD_2020'
  rm -f SG_SIMD_2020* SIMD2020v2*xlsx
}

function elevation {
  # TODO Switch to https://osdatahub.os.uk/downloads/open/Terrain50 directly
  wget https://play.abstreet.org/dev/data/input/shared/elevation/UK-dem-50m-4326.tif.gz
  gunzip UK-dem-50m-4326.tif.gz
  mv UK-dem-50m-4326.tif tmp
}

function settlements {
  # From https://www.nrscotland.gov.uk/publications/population-estimates-for-settlements-and-localities-in-scotland-mid-2020/
  wget https://www.nrscotland.gov.uk/media/2hsoadnx/shapefiles.zip
  unzip shapefiles.zip
  ogr2ogr settlements.geojson \
          -t_srs EPSG:4326 \
          Settlements_2020_MHW.shp \
          -nlt PROMOTE_TO_MULTI
  # There's some non-utf8 encodings; fix
  iconv -f latin1 -t UTF-8 settlements.geojson > tmp/settlements.geojson
  rm -f Localities* Settlements* shapefiles.zip settlements.geojson
}

function greenspace {
  # Manually download all of GB as GeoPackage from https://osdatahub.os.uk/downloads/open/OpenGreenspace and pass in the .zip here
  unzip $1 -j Data/opgrsp_gb.gpkg
  mv Data/opgrsp_gb.gpkg .
  rmdir Data
  ogr2ogr tmp/greenspace.gpkg \
          -t_srs EPSG:4326 \
          opgrsp_gb.gpkg \
          -sql 'SELECT id, distinctive_name_1 as name, geometry FROM greenspace_site WHERE function IN ("Playing Field", "Public Park Or Garden")'
  ogr2ogr tmp/all_greenspace.gpkg \
          -t_srs EPSG:4326 \
          opgrsp_gb.gpkg \
          -nlt PROMOTE_TO_MULTI \
          -sql 'SELECT geometry FROM greenspace_site'
  ogr2ogr tmp/greenspace_access_points.gpkg \
          -t_srs EPSG:4326 \
          opgrsp_gb.gpkg \
          -sql 'SELECT ref_to_greenspace_site as site_id, geometry FROM access_point'
  rm -f opgrsp_gb.gpkg
}

#core_net
#rnet
#streetspace
#schools
#town_centres ~/Downloads/Town_Centres_-_Scotland.json
#gp_and_hospitals ~/Downloads/GP_Practices_-_Scotland.json ~/Downloads/NHS_Hospitals_-_Scotland.json
#od_and_zones ~/Downloads/desire_lines_scotland.csv
#traffic ~/Downloads/final_estimates_Scotland.gpkg
#population
#elevation
#settlements
#greenspace ~/Downloads/opgrsp_gpkg_gb.zip

# Keep track of input files
md5sum tmp/* ../web/public/*pmtiles > manifest
