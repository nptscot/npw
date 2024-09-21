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
    this.checkReady();
    return Array.from(this.inner!.getBounds()) as [
      number,
      number,
      number,
      number,
    ];
  }

  getInvertedBoundary(): Feature<Polygon> {
    this.checkReady();
    return JSON.parse(this.inner!.getInvertedBoundary());
  }

  renderDebug(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.renderDebug());
  }

  toRouteSnapper(): Uint8Array {
    this.checkReady();
    return this.inner!.toRouteSnapper();
  }

  // TODO More specific type
  renderRoutes(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.renderRoutes());
  }

  // TODO types
  newRoute(input: any): number {
    this.checkReady();
    return this.inner!.newRoute(input);
  }

  // TODO types
  editRoute(id: number, input: any) {
    this.checkReady();
    this.inner!.editRoute(id, input);
  }

  deleteRoute(id: number) {
    this.checkReady();
    this.inner!.deleteRoute(id);
  }

  evaluateRoute(req: {
    // TODO LngLatLike doesn't work?
    start: { lng: number; lat: number };
    end: Position;
  }): RouteGJ {
    this.checkReady();
    return JSON.parse(
      this.inner!.evaluateRoute({
        x1: req.start.lng,
        y1: req.start.lat,
        x2: req.end[0],
        y2: req.end[1],
      }),
    );
  }

  toSavefile(): string {
    this.checkReady();
    return this.inner!.toSavefile();
  }

  loadSavefile(contents: string) {
    this.checkReady();
    this.inner!.loadSavefile(contents);
  }

  private checkReady() {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }
  }
}
