import { layers, namedFlavor } from "@protomaps/basemaps";
import type { StyleSpecification } from "maplibre-gl";
import { assetUrl } from "../stores";

export function getBasemapStyle(
  knowRemoteStorage: boolean,
  flavor: string,
): StyleSpecification {
  // Before we know if we're using remote storage, just show a blank map
  if (!knowRemoteStorage) {
    return {
      version: 8,
      sources: {},
      layers: [],
    };
  }

  return {
    version: 8,
    // TODO Host
    glyphs:
      "https://protomaps.github.io/basemaps-assets/fonts/{fontstack}/{range}.pbf",
    sprite: "https://protomaps.github.io/basemaps-assets/sprites/v4/light",
    sources: {
      protomaps: {
        type: "vector",
        url: `pmtiles://${assetUrl("uk_basemap.pmtiles")}`,
        attribution:
          '<a href="https://protomaps.com">Protomaps</a> © <a href="https://openstreetmap.org">OpenStreetMap</a>',
      },
    },
    layers: layers("protomaps", namedFlavor(flavor), { lang: "en" }),
  };
}
