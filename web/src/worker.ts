import init, { MapModel } from "backend";
import type { Position, Feature, Polygon, FeatureCollection } from "geojson";
import type { RouteGJ } from "./stores";

export class Backend {
  inner: MapModel | null;

  constructor() {
    this.inner = null;
  }

  // Intended to be called immediately after constructing
  async loadFile(graphBytes: Uint8Array) {
    // TODO Do we need to do this only once?
    await init();

    this.inner = new MapModel(graphBytes);
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

  toRouteSnapper(): Uint8Array {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return this.inner.toRouteSnapper();
  }

  renderRoutes(): FeatureCollection {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return JSON.parse(this.inner.renderRoutes());
  }

  // TODO types
  newRoute(input: any): number {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return this.inner.newRoute(input);
  }

  // TODO types
  editRoute(id: number, input: any) {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    this.inner.editRoute(id, input);
  }

  deleteRoute(id: number) {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    this.inner.deleteRoute(id);
  }

  evaluateRoute(req: {
    // TODO LngLatLike doesn't work?
    start: { lng: number; lat: number };
    end: Position;
  }): RouteGJ {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }

    return JSON.parse(
      this.inner.evaluateRoute({
        x1: req.start.lng,
        y1: req.start.lat,
        x2: req.end[0],
        y2: req.end[1],
      }),
    );
  }
}
