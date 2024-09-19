import * as Comlink from "comlink";
import init, { MapModel } from "backend";
import type {
  Feature,
  Polygon,
  FeatureCollection,
} from "geojson";

export class Backend {
  inner: MapModel | null;

  constructor() {
    this.inner = null;
  }

  // TODO If we can call init() somewhere, we could synchronously construct this and avoid all the isLoaded mess
  async loadFile(graphBytes: Uint8Array) {
    // TODO Do we need to do this only once?
    await init();

    this.inner = new MapModel(graphBytes);
  }

  isLoaded(): boolean {
    return this.inner != null;
  }

  getBounds(): [number, number, number, number] {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return Array.from(this.inner.getBounds()) as [
      number,
      number,
      number,
      number,
    ];
  }

  getInvertedBoundary(): Feature<Polygon> {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return JSON.parse(this.inner.getInvertedBoundary());
  }

  renderDebug(): FeatureCollection {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return JSON.parse(this.inner.renderDebug());
  }
}

Comlink.expose(Backend);
