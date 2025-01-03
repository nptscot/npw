import init, { MapModel } from "backend";
import type {
  Feature,
  FeatureCollection,
  LineString,
  Polygon,
  Position,
} from "geojson";
import type {
  AreaMeshDensity,
  DataZones,
  EvaluateODOut,
  GPHospitals,
  Greenspaces,
  GridMeshDensity,
  PrecalculatedFlows,
  RouteGJ,
  RouteNode,
  RouteProps,
  Schools,
  SetRouteInput,
  Settlements,
  Stats,
  TownCentres,
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

  renderRoutes(): FeatureCollection<LineString, RouteProps> {
    this.checkReady();
    return JSON.parse(this.inner!.renderRoutes());
  }

  // TODO Be consistent about undefined vs null
  setRoute(id: number | null, input: SetRouteInput) {
    this.checkReady();
    this.inner!.setRoute(id == null ? undefined : id, input);
  }

  deleteRoute(id: number) {
    this.checkReady();
    this.inner!.deleteRoute(id);
  }

  clearAllRoutes() {
    this.checkReady();
    this.inner!.clearAllRoutes();
  }

  autosplitRoute(full_path: RouteNode[]): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.autosplitRoute(full_path));
  }

  evaluateRoute(req: {
    // TODO LngLatLike doesn't work?
    start: { lng: number; lat: number };
    end: Position;
    breakdown: "" | "los" | "infra_type" | "gradient";
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

  evaluateOD(fastSample: boolean): EvaluateODOut {
    this.checkReady();
    return JSON.parse(this.inner!.evaluateOD(fastSample));
  }

  recalculateStats(): Stats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateStats());
  }

  getAreaMeshDensity(): AreaMeshDensity {
    this.checkReady();
    return JSON.parse(this.inner!.getAreaMeshDensity());
  }

  getGridMeshDensity(
    resolution: number,
    xOffset: number,
    yOffset: number,
  ): GridMeshDensity {
    this.checkReady();
    return JSON.parse(
      this.inner!.getGridMeshDensity(resolution, xOffset, yOffset),
    );
  }

  classifyExistingNetwork(): FeatureCollection<
    LineString,
    { infra_type: string; way: string }
  > {
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

  getSettlements(): Settlements {
    this.checkReady();
    return JSON.parse(this.inner!.getSettlements());
  }

  getGreenspaces(): Greenspaces {
    this.checkReady();
    return JSON.parse(this.inner!.getGreenspaces());
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
