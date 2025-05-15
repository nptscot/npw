import booleanIntersects from "@turf/boolean-intersects";
import type {
  FeatureCollection,
  LineString,
  MultiPolygon,
  Polygon,
} from "geojson";
import { get, writable, type Writable } from "svelte/store";
import boundariesUrl from "../assets/boundaries.geojson?url";

type Boundaries = FeatureCollection<Polygon | MultiPolygon, { name: string }>;

let boundaries: Writable<Boundaries | null> = writable(null);

export async function backfillSavefile(
  contents: string,
  assumeBoundaryName: string | null,
): Promise<string> {
  let gj = JSON.parse(contents);

  await backfillV1(gj, assumeBoundaryName);

  return JSON.stringify(gj);
}

// Backfill v1->v2 by guessing study_area_name
async function backfillV1(gj: any, assumeBoundaryName: string | null) {
  if (gj.version != 1) {
    return;
  }

  console.log(`Backfilling a v1 savefile`);

  if (assumeBoundaryName) {
    console.log(`Assuming the boundary is ${assumeBoundaryName}`);
    gj.study_area_name = assumeBoundaryName;
    gj.version = 2;
    return;
  }

  if (get(boundaries) == null) {
    let resp = await fetch(boundariesUrl);
    boundaries.set(await resp.json());
  }

  let study_area_name = findBoundaryContainingMostLineStrings(
    gj,
    get(boundaries)!,
  );
  if (study_area_name) {
    gj.study_area_name = study_area_name;
    gj.version = 2;
  } else {
    throw new Error(
      `Couldn't find any study area containing routes in your savefile`,
    );
  }
}

function findBoundaryContainingMostLineStrings(
  savefile: FeatureCollection<LineString>,
  boundaries: Boundaries,
): string | null {
  console.log(
    `Matching ${savefile.features.length} features against ${boundaries.features.length} boundaries`,
  );
  let candidates: { [name: string]: number } = {};

  // Only try the first 10, for speed
  for (let f of savefile.features.slice(0, 10)) {
    for (let boundary of boundaries.features) {
      if (booleanIntersects(boundary, f)) {
        let name = `LAD_${boundary.properties.name}`;
        if (!candidates[name]) {
          candidates[name] = 1;
        } else {
          candidates[name]++;
        }
      }
    }
  }

  let pairs: [string, number][] = Object.entries(candidates);
  pairs.sort((a, b) => b[1] - a[1]);
  console.log(`Candidate matches: ${JSON.stringify(pairs)}`);
  return pairs.length > 0 ? pairs[0][0] : null;
}
