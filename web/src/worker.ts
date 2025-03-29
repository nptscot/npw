import init, { MapModel } from "backend";
import * as Comlink from "comlink";
import type {
  Feature,
  FeatureCollection,
  LineString,
  Polygon,
  Position,
} from "geojson";
import type {
  AutosplitRoute,
  ConnectedComponents,
  DataZones,
  DynamicRoad,
  EvaluateODOut,
  Greenspaces,
  GridMeshDensity,
  ODStats,
  POIs,
  RouteGJ,
  RouteProps,
  SetRouteInput,
  Settlements,
  StaticRoad,
  Stats,
  Tier,
  TownCentres,
  Waypoint,
} from "./types";

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

  getInvertedBoundaryForStudyArea(): Feature<Polygon> {
    this.checkReady();
    return JSON.parse(this.inner!.getInvertedBoundaryForStudyArea());
  }

  getInvertedBoundaryInsideSettlements(): Feature<Polygon> {
    this.checkReady();
    return JSON.parse(this.inner!.getInvertedBoundaryInsideSettlements());
  }

  getInvertedBoundaryOutsideSettlements(): Feature<Polygon> {
    this.checkReady();
    return JSON.parse(this.inner!.getInvertedBoundaryOutsideSettlements());
  }

  renderStaticRoads(): FeatureCollection<LineString, StaticRoad> {
    this.checkReady();
    return JSON.parse(this.inner!.renderStaticRoads());
  }

  renderDynamicRoads(): DynamicRoad[] {
    this.checkReady();
    return JSON.parse(this.inner!.renderDynamicRoads());
  }

  getAllRoutes(): FeatureCollection<LineString, RouteProps> {
    this.checkReady();
    return JSON.parse(this.inner!.getAllRoutes());
  }

  getRoute(id: number): Feature<LineString, RouteProps> {
    this.checkReady();
    return JSON.parse(this.inner!.getRoute(id));
  }

  // TODO Be consistent about undefined vs null
  setRoute(id: number | null, input: SetRouteInput) {
    this.checkReady();
    this.inner!.setRoute(id == null ? undefined : id, input);
  }

  deleteRoutes(ids: number[]) {
    this.checkReady();
    this.inner!.deleteRoutes(new Uint32Array(ids));
  }

  changeTier(routeIds: number[], tier: string) {
    this.checkReady();
    // Wrap in quotes for JSON parsing
    this.inner!.changeTier(new Uint32Array(routeIds), `"${tier}"`);
  }

  changeInfraType(routeIds: number[], infraType: string) {
    this.checkReady();
    // Wrap in quotes for JSON parsing
    this.inner!.changeInfraType(new Uint32Array(routeIds), `"${infraType}"`);
  }

  clearAllRoutes() {
    this.checkReady();
    this.inner!.clearAllRoutes();
  }

  autosplitRoute(
    editingRouteId: number | null,
    waypoints: Waypoint[],
    overrideInfraType: string | null,
    defaultTier: Tier,
    preferMajor: boolean,
  ): AutosplitRoute {
    this.checkReady();
    return JSON.parse(
      this.inner!.autosplitRoute(
        editingRouteId == null ? undefined : editingRouteId,
        waypoints,
        overrideInfraType,
        `"${defaultTier}"`,
        preferMajor,
      ),
    );
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

  // Needs loading screen
  evaluateOD(fastSample: boolean): EvaluateODOut {
    this.checkReady();
    return JSON.parse(this.inner!.evaluateOD(fastSample));
  }

  // Fast
  recalculateStats(): Stats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateStats());
  }

  getBaselineStats(): Stats {
    this.checkReady();
    return JSON.parse(this.inner!.getBaselineStats());
  }

  // Needs loading screen
  recalculateODStats(): ODStats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateODStats());
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

  // Needs loading screen
  importExistingRoutes(kind: "infra-type" | "los"): number {
    this.checkReady();
    return this.inner!.importExistingRoutes(kind == "infra-type");
  }

  // Needs loading screen
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

  getPOIs(): POIs {
    this.checkReady();
    return JSON.parse(this.inner!.getPOIs());
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

  debugReachablePath(kind: string, idx: number): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.debugReachablePath(kind, idx));
  }

  // TODO Unused
  debugUnreachablePath(kind: string, idx: number): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.debugUnreachablePath(kind, idx));
  }

  fixUnreachablePOI(kind: string, idx: number): SetRouteInput {
    this.checkReady();
    let route = JSON.parse(this.inner!.fixUnreachablePOI(kind, idx));
    // TODO Hack around this necessary duplication
    route.roads = route.feature.properties.roads;
    return route;
  }

  getConnectedComponents(): ConnectedComponents {
    this.checkReady();
    return JSON.parse(this.inner!.getConnectedComponents());
  }

  snapRoute(
    waypoints: Waypoint[],
    preferMajor: boolean,
  ): Feature<LineString, RouteProps> {
    this.checkReady();
    return JSON.parse(this.inner!.snapRoute(waypoints, preferMajor));
  }

  snapPoint(pt: number[], preferMajor: boolean): [number, number] {
    this.checkReady();
    let snapped = this.inner!.snapPoint(pt[0], pt[1], preferMajor);
    return [snapped[0], snapped[1]];
  }

  getExtraNodes(
    waypt1: Waypoint,
    waypt2: Waypoint,
    preferMajor: boolean,
  ): [number, number, boolean][] {
    this.checkReady();
    return JSON.parse(this.inner!.getExtraNodes(waypt1, waypt2, preferMajor));
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

Comlink.expose(Backend);
