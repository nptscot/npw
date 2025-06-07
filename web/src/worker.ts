import init, { MapModel } from "backend";
import * as Comlink from "comlink";
import type {
  Feature,
  FeatureCollection,
  LineString,
  MultiPolygon,
  Polygon,
  Position,
} from "geojson";
import type {
  BaselineStats,
  ConnectedComponents,
  InfraType,
  NetworkLengths,
  ODStats,
  RouteGJ,
  SetRouteInput,
  SlowStats,
  Stats,
  Tier,
  TownCentreRoutes,
  Waypoint,
} from "./types";

export class InnerBackend {
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

  getInvertedBoundaryOutsideSettlements(): Feature<MultiPolygon> {
    this.checkReady();
    return JSON.parse(this.inner!.getInvertedBoundaryOutsideSettlements());
  }

  getStudyAreaBoundary(): Feature<Polygon> {
    this.checkReady();
    return JSON.parse(this.inner!.getStudyAreaBoundary());
  }

  renderStaticRoads(): Uint8Array {
    return asBytes(this.checkReady().renderStaticRoads());
  }

  renderDynamicRoads(): Uint8Array {
    return asBytes(this.checkReady().renderDynamicRoads());
  }

  getAllRoutes(): Uint8Array {
    return asBytes(this.checkReady().getAllRoutes());
  }

  getRoute(id: number): Uint8Array {
    return asBytes(this.checkReady().getRoute(id));
  }

  getRouteSections(ids: Array<number>): Uint8Array {
    return asBytes(this.checkReady().getRouteSections(new Uint32Array(ids)));
  }

  // TODO Be consistent about undefined vs null
  setRoute(id: number | null, input: SetRouteInput): number[] {
    this.checkReady();
    return Array.from(this.inner!.setRoute(id == null ? undefined : id, input));
  }

  deleteRoutes(ids: number[]) {
    this.checkReady();
    this.inner!.deleteRoutes(new Uint32Array(ids));
  }

  changeTier(routeIds: number[], tier: Tier) {
    this.checkReady();
    // Wrap in quotes for JSON parsing
    this.inner!.changeTier(new Uint32Array(routeIds), `"${tier}"`);
  }

  changeInfraType(routeIds: number[], infraType: InfraType) {
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
    overrideInfraType: InfraType | null,
    defaultTier: Tier,
    majorSnapThreshold: number | null,
  ): Uint8Array {
    return asBytes(
      this.checkReady().autosplitRoute(
        editingRouteId == null ? undefined : editingRouteId,
        waypoints,
        overrideInfraType,
        `"${defaultTier}"`,
        majorSnapThreshold,
      ),
    );
  }

  previewRoute(
    waypoints: Waypoint[],
    majorSnapThreshold: number | null,
  ): Uint8Array {
    return asBytes(
      this.checkReady().previewRoute(waypoints, majorSnapThreshold),
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

  evaluateOD(fastSample: boolean): Uint8Array {
    return asBytes(this.checkReady().evaluateOD(fastSample));
  }

  recalculateStats(): Stats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateStats());
  }

  recalculateSlowStats(): SlowStats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateSlowStats());
  }

  getBaselineStats(): BaselineStats {
    this.checkReady();
    return JSON.parse(this.inner!.getBaselineStats());
  }

  recalculateODStats(): ODStats {
    this.checkReady();
    return JSON.parse(this.inner!.recalculateODStats());
  }

  getGridMeshDensity(
    resolution: number,
    xOffset: number,
    yOffset: number,
  ): Uint8Array {
    return asBytes(
      this.checkReady().getGridMeshDensity(resolution, xOffset, yOffset),
    );
  }

  importExistingRoutes(kind: "infra-type" | "los") {
    this.checkReady();
    this.inner!.importExistingRoutes(kind == "infra-type");
  }

  importArterialRoads() {
    this.checkReady();
    this.inner!.importArterialRoads();
  }

  loadSavefile(contents: string) {
    this.checkReady();
    this.inner!.loadSavefile(contents);
  }

  getPOIs(): Uint8Array {
    return asBytes(this.checkReady().getPOIs());
  }

  getTownCentres(): Uint8Array {
    return asBytes(this.checkReady().getTownCentres());
  }

  getSettlements(): Uint8Array {
    return asBytes(this.checkReady().getSettlements());
  }

  getSettlementLocations(): [string, [number, number, number, number]][] {
    this.checkReady();
    return JSON.parse(this.inner!.getSettlementLocations());
  }

  getGreenspaces(): Uint8Array {
    return asBytes(this.checkReady().getGreenspaces());
  }

  getDataZones(): Uint8Array {
    return asBytes(this.checkReady().getDataZones());
  }

  debugReachablePath(
    kind: string,
    idx: number,
  ): FeatureCollection & { length_meters: number } {
    this.checkReady();
    return JSON.parse(this.inner!.debugReachablePath(kind, idx));
  }

  // TODO Unused
  debugUnreachablePath(kind: string, idx: number): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.debugUnreachablePath(kind, idx));
  }

  fixUnreachablePOI(
    kind: string,
    idx: number,
  ): Feature<LineString, SetRouteInput & { length_meters: number }> {
    this.checkReady();
    return JSON.parse(this.inner!.fixUnreachablePOI(kind, idx));
  }

  getConnectedComponents(): ConnectedComponents {
    this.checkReady();
    return JSON.parse(this.inner!.getConnectedComponents());
  }

  snapPoint(pt: number[], majorSnapThreshold: number | null): [number, number] {
    this.checkReady();
    let snapped = this.inner!.snapPoint(pt[0], pt[1], majorSnapThreshold);
    return [snapped[0], snapped[1]];
  }

  getExtraNodes(
    waypt1: Waypoint,
    waypt2: Waypoint,
    majorSnapThreshold: number | null,
  ): [number, number, boolean][] {
    this.checkReady();
    return JSON.parse(
      this.inner!.getExtraNodes(waypt1, waypt2, majorSnapThreshold),
    );
  }

  getMajorJunctions(): Uint8Array {
    return asBytes(this.checkReady().getMajorJunctions());
  }

  getTownCentreRoutes(): TownCentreRoutes {
    this.checkReady();
    return JSON.parse(this.inner!.getTownCentreRoutes());
  }

  getTownCentrePoints(): FeatureCollection {
    this.checkReady();
    return JSON.parse(this.inner!.getTownCentrePoints());
  }

  getNetworkLengths(): NetworkLengths {
    this.checkReady();
    return JSON.parse(this.inner!.getNetworkLengths());
  }

  private checkReady(): MapModel {
    if (!this.inner) {
      throw new Error("InnerBackend used without a file loaded");
    }
    return this.inner;
  }
}

// Avoid a structured clone when passing data from the web worker back to the main thread
function asBytes(data: Uint8Array): Uint8Array {
  return Comlink.transfer(data, [data.buffer]);
}

Comlink.expose(InnerBackend);
