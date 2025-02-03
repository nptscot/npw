`boundaries.geojson` was produced by downloading <https://github.com/nptscot/inputdata/blob/main/boundaries/la_regions_scotland_bfe_simplified_2023.geojson> and <https://github.com/nptscot/npt/releases/download/boundaries-2024/scottish_regions.geojson> (needs a login).

```
mapshaper scottish_regions.geojson -each 'name=Region, delete Region, delete this.properties["Number of LAs"], delete LAs, kind="REGION"' -o precision=0.000001 regions.geojson
mapshaper la_regions_scotland_bfe_simplified_2023.geojson -each 'name=LAD23NM, delete LAD23NM, delete LAD23CD, delete Region, kind="LAD"' -o precision=0.000001 lads.geojson
mapshaper -i regions.geojson lads.geojson combine-files -merge-layers -o boundaries.geojson
```
