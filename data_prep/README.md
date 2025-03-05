`boundaries.geojson` was produced by downloading <https://github.com/nptscot/inputdata/blob/main/boundaries/la_regions_scotland_bfe_simplified_2023.geojson> (needs a login).

```
mapshaper la_regions_scotland_bfe_simplified_2023.geojson -each 'name=LAD23NM, delete LAD23NM, delete LAD23CD, delete Region, kind="LAD"' -o precision=0.000001 boundaries.geojson
```
