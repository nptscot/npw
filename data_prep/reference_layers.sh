#!/bin/bash

set -e
set -x

# From https://github.com/nptscot/npt/releases
wget https://github.com/nptscot/npt/releases/download/CN_class/city_of_edinburgh_2024-10-01_4_coherent_network.geojson -O ../web/public/core_network.geojson
