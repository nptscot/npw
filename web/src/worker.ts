import init, { MapModel } from "backend";
import type { Position, Feature, Polygon, FeatureCollection } from "geojson";
import type { RouteGJ, EvaluateODOut, Stats, Schools } from "./stores";

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

  renderLevelOfService(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.renderLevelOfService());
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
  // TODO Be consistent about undefined vs null
  setRoute(id: number | null, input: any): number {
    this.checkReady();
    return this.inner!.setRoute(id == null ? undefined : id, input);
  }

  deleteRoute(id: number) {
    this.checkReady();
    this.inner!.deleteRoute(id);
  }

  clearAllRoutes() {
    this.checkReady();
    this.inner!.clearAllRoutes();
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

  evaluateOD(): EvaluateODOut {
    this.checkReady();
    return JSON.parse(this.inner!.evaluateOD());
  }

  recalculateStats(): Stats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateStats());
  }

  meshDensity(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.meshDensity());
  }

  classifyExistingNetwork(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.classifyExistingNetwork());
  }

  importExistingRoutes(): number {
    this.checkReady();
    return this.inner!.importExistingRoutes();
  }

  toSavefile(): string {
    this.checkReady();
    return this.inner!.toSavefile();
  }

  loadSavefile(contents: string) {
    this.checkReady();
    this.inner!.loadSavefile(contents);
  }

  getSchools(): Schools {
    this.checkReady();
    return JSON.parse(this.inner!.getSchools());
  }

  getNetworkBuffer(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.getNetworkBuffer());
  }

  renderReachableNetwork(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.renderReachableNetwork());
  }

  private checkReady() {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }
  }
}
