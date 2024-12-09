import init, { MapModel } from "backend";
import type { Position, Feature, Polygon, FeatureCollection } from "geojson";
import type {
  RouteGJ,
  EvaluateODOut,
  Stats,
  Schools,
  GPHospitals,
  TownCentres,
  DataZones,
  PrecalculatedFlows,
} from "./stores";

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

  renderCoreNetwork(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.renderCoreNetwork());
  }

  renderPrecalculatedFlows(): PrecalculatedFlows {
    this.checkReady();
    return JSON.parse(this.inner!.renderPrecalculatedFlows());
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
    breakdown: "" | "los" | "infra_type";
  }): RouteGJ {
    this.checkReady();
    return JSON.parse(
      this.inner!.evaluateRoute({
        x1: req.start.lng,
        y1: req.start.lat,
        x2: req.end[0],
        y2: req.end[1],
        breakdown: req.breakdown,
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

  importCoreNetwork(): number {
    this.checkReady();
    return this.inner!.importCoreNetwork();
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

  getGpHospitals(): GPHospitals {
    this.checkReady();
    return JSON.parse(this.inner!.getGPHospitals());
  }

  getTownCentres(): TownCentres {
    this.checkReady();
    return JSON.parse(this.inner!.getTownCentres());
  }

  getDataZones(): DataZones {
    this.checkReady();
    return JSON.parse(this.inner!.getDataZones());
  }

  renderReachableNetwork(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.renderReachableNetwork());
  }

  debugReachablePath(kind: string, idx: number): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.debugReachablePath(kind, idx));
  }

  debugUnreachablePath(kind: string, idx: number): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.debugUnreachablePath(kind, idx));
  }

  getMajorJunctions(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.getMajorJunctions());
  }

  private checkReady() {
    if (!this.inner) {
      throw new Error("Backend used without a file loaded");
    }
  }
}
