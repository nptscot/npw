import * as Comlink from "comlink";
import type {
  Feature,
  FeatureCollection,
  LineString,
  MultiPolygon,
  Polygon,
  Position,
} from "geojson";
import { loadingSpinners } from "./stores";
import type {
  AutosplitRoute,
  BaselineStats,
  ConnectedComponents,
  DataZones,
  DynamicRoad,
  EvaluateODOut,
  Greenspaces,
  GridMeshDensity,
  InfraType,
  NetworkLengths,
  ODStats,
  POIs,
  RouteGJ,
  RouteProps,
  RouteSection,
  SetRouteInput,
  Settlements,
  SlowStats,
  StaticRoad,
  Stats,
  Tier,
  TownCentreRoutes,
  TownCentres,
  Waypoint,
} from "./types";
import type { InnerBackend } from "./worker";

// This is a wrapper around the Comlink wrapper. It runs on the browser/UI
// thread and does common logging / loading stuff. Ideally we'd put the Comlink
// wrapper around just the WASM API directly and move all the logic here, so
// that boilerplate isn't repeated.
export class Backend {
  inner: Comlink.Remote<InnerBackend>;

  constructor(inner: Comlink.Remote<InnerBackend>) {
    this.inner = inner;
  }

  async getBounds(): Promise<[number, number, number, number]> {
    let t = this.start("getBounds");
    let result = await this.inner.getBounds();
    this.stop(t);
    return result;
  }

  async getInvertedBoundaryForStudyArea(): Promise<Feature<Polygon>> {
    let t = this.start("getInvertedBoundaryForStudyArea");
    let result = await this.inner.getInvertedBoundaryForStudyArea();
    this.stop(t);
    return result;
  }

  async getInvertedBoundaryInsideSettlements(): Promise<Feature<Polygon>> {
    let t = this.start("getInvertedBoundaryInsideSettlements");
    let result = await this.inner.getInvertedBoundaryInsideSettlements();
    this.stop(t);
    return result;
  }

  async getInvertedBoundaryOutsideSettlements(): Promise<
    Feature<MultiPolygon>
  > {
    let t = this.start("getInvertedBoundaryOutsideSettlements");
    let result = await this.inner.getInvertedBoundaryOutsideSettlements();
    this.stop(t);
    return result;
  }

  async getStudyAreaBoundary(): Promise<Feature<Polygon>> {
    let t = this.start("getStudyAreaBoundary");
    let result = await this.inner.getStudyAreaBoundary();
    this.stop(t);
    return result;
  }

  async renderStaticRoads(): Promise<
    FeatureCollection<LineString, StaticRoad>
  > {
    let t = this.start("renderStaticRoads");
    let result = await this.inner.renderStaticRoads();
    this.stop(t);
    return result;
  }

  async renderDynamicRoads(): Promise<DynamicRoad[]> {
    let t = this.start("renderDynamicRoads");
    let result = await this.inner.renderDynamicRoads();
    this.stop(t);
    return result;
  }

  async getAllRoutes(): Promise<
    FeatureCollection<LineString, RouteProps> & {
      id_counter: number;
    }
  > {
    let t = this.start("getAllRoutes");
    let result = await this.inner.getAllRoutes();
    this.stop(t);
    return result;
  }

  async getRoute(id: number): Promise<Feature<LineString, RouteProps>> {
    let t = this.start("getRoute");
    let result = await this.inner.getRoute(id);
    this.stop(t);
    return result;
  }

  async getRouteSections(ids: Array<number>): Promise<RouteSection[]> {
    let t = this.start("getRouteSections");
    let result = await this.inner.getRouteSections(ids);
    this.stop(t);
    return result;
  }

  async setRoute(id: number | null, input: SetRouteInput): Promise<number[]> {
    let t = this.start("setRoute");
    let result = await this.inner.setRoute(id, input);
    this.stop(t);
    return result;
  }

  async deleteRoutes(ids: number[]) {
    let t = this.start("deleteRoutes");
    await this.inner.deleteRoutes(ids);
    this.stop(t);
  }

  async changeTier(routeIds: number[], tier: Tier) {
    let t = this.start("changeTier");
    await this.inner.changeTier(routeIds, tier);
    this.stop(t);
  }

  async changeInfraType(routeIds: number[], infraType: InfraType) {
    let t = this.start("changeInfraType");
    await this.inner.changeInfraType(routeIds, infraType);
    this.stop(t);
  }

  async clearAllRoutes() {
    let t = this.start("clearAllRoutes");
    await this.inner.clearAllRoutes();
    this.stop(t);
  }

  async autosplitRoute(
    editingRouteId: number | null,
    waypoints: Waypoint[],
    overrideInfraType: InfraType | null,
    defaultTier: Tier,
    majorSnapThreshold: number | null,
  ): Promise<AutosplitRoute> {
    let t = this.start("autosplitRoute");
    let result = await this.inner.autosplitRoute(
      editingRouteId,
      waypoints,
      overrideInfraType,
      defaultTier,
      majorSnapThreshold,
    );
    this.stop(t);
    return result;
  }

  async previewRoute(
    waypoints: Waypoint[],
    majorSnapThreshold: number | null,
  ): Promise<Feature<LineString>> {
    let t = this.start("previewRoute");
    let result = await this.inner.previewRoute(waypoints, majorSnapThreshold);
    this.stop(t);
    return result;
  }

  async evaluateRoute(req: {
    // TODO LngLatLike doesn't work?
    start: { lng: number; lat: number };
    end: Position;
    breakdown: "" | "los" | "infra_type" | "gradient";
  }): Promise<RouteGJ> {
    let t = this.start("evaluateRoute");
    let result = await this.inner.evaluateRoute(req);
    this.stop(t);
    return result;
  }

  async evaluateOD(fastSample: boolean): Promise<EvaluateODOut> {
    let t = this.start("evaluateOD");
    let result = await this.inner.evaluateOD(fastSample);
    this.stop(t);
    return result;
  }

  async recalculateStats(): Promise<Stats> {
    let t = this.start("recalculateStats");
    let result = await this.inner.recalculateStats();
    this.stop(t);
    return result;
  }

  async recalculateSlowStats(): Promise<SlowStats> {
    let t = this.start("recalculateSlowStats");
    let result = await this.inner.recalculateSlowStats();
    this.stop(t);
    return result;
  }

  async getBaselineStats(): Promise<BaselineStats> {
    let t = this.start("getBaselineStats");
    let result = await this.inner.getBaselineStats();
    this.stop(t);
    return result;
  }

  async recalculateODStats(): Promise<ODStats> {
    let t = this.start("recalculateODStats");
    let result = await this.inner.recalculateODStats();
    this.stop(t);
    return result;
  }

  async getGridMeshDensity(
    resolution: number,
    xOffset: number,
    yOffset: number,
  ): Promise<GridMeshDensity> {
    let t = this.start("getGridMeshDensity");
    let result = await this.inner.getGridMeshDensity(
      resolution,
      xOffset,
      yOffset,
    );
    this.stop(t);
    return result;
  }

  async importExistingRoutes(kind: "infra-type" | "los") {
    let t = this.start("importExistingRoutes");
    await this.inner.importExistingRoutes(kind);
    this.stop(t);
  }

  async importArterialRoads() {
    let t = this.start("importArterialRoads");
    await this.inner.importArterialRoads();
    this.stop(t);
  }

  async loadSavefile(contents: string) {
    let t = this.start("loadSavefile");
    try {
      await this.inner.loadSavefile(contents);
    } finally {
      this.stop(t);
    }
  }

  async getPOIs(): Promise<POIs> {
    let t = this.start("getPOIs");
    let result = await this.inner.getPOIs();
    this.stop(t);
    return result;
  }

  async getTownCentres(): Promise<TownCentres> {
    let t = this.start("getTownCentres");
    let result = await this.inner.getTownCentres();
    this.stop(t);
    return result;
  }

  async getSettlements(): Promise<Settlements> {
    let t = this.start("getSettlements");
    let result = await this.inner.getSettlements();
    this.stop(t);
    return result;
  }

  async getSettlementLocations(): Promise<
    [string, [number, number, number, number]][]
  > {
    let t = this.start("getSettlementLocations");
    let result = await this.inner.getSettlementLocations();
    this.stop(t);
    return result;
  }

  async getGreenspaces(): Promise<Greenspaces> {
    let t = this.start("getGreenspaces");
    let result = await this.inner.getGreenspaces();
    this.stop(t);
    return result;
  }

  async getDataZones(): Promise<DataZones> {
    let t = this.start("getDataZones");
    let result = await this.inner.getDataZones();
    this.stop(t);
    return result;
  }

  async debugReachablePath(
    kind: string,
    idx: number,
  ): Promise<FeatureCollection & { length_meters: number }> {
    let t = this.start("debugReachablePath");
    try {
      return await this.inner.debugReachablePath(kind, idx);
    } finally {
      this.stop(t);
    }
  }

  // TODO Unused
  async debugUnreachablePath(
    kind: string,
    idx: number,
  ): Promise<FeatureCollection> {
    let t = this.start("debugUnreachablePath");
    let result = await this.inner.debugUnreachablePath(kind, idx);
    this.stop(t);
    return result;
  }

  async fixUnreachablePOI(
    kind: string,
    idx: number,
  ): Promise<Feature<LineString, SetRouteInput & { length_meters: number }>> {
    let t = this.start("fixUnreachablePOI");
    try {
      return await this.inner.fixUnreachablePOI(kind, idx);
    } finally {
      this.stop(t);
    }
  }

  async getConnectedComponents(): Promise<ConnectedComponents> {
    let t = this.start("getConnectedComponents");
    let result = await this.inner.getConnectedComponents();
    this.stop(t);
    return result;
  }

  async snapPoint(
    pt: number[],
    majorSnapThreshold: number | null,
  ): Promise<[number, number]> {
    let t = this.start("snapPoint");
    let result = await this.inner.snapPoint(pt, majorSnapThreshold);
    this.stop(t);
    return result;
  }

  async getExtraNodes(
    waypt1: Waypoint,
    waypt2: Waypoint,
    majorSnapThreshold: number | null,
  ): Promise<[number, number, boolean][]> {
    let t = this.start("getExtraNodes");
    let result = await this.inner.getExtraNodes(
      waypt1,
      waypt2,
      majorSnapThreshold,
    );
    this.stop(t);
    return result;
  }

  async getMajorJunctions(): Promise<FeatureCollection> {
    let t = this.start("getMajorJunctions");
    let result = await this.inner.getMajorJunctions();
    this.stop(t);
    return result;
  }

  async getTownCentreRoutes(): Promise<TownCentreRoutes> {
    let t = this.start("getTownCentreRoutes");
    let result = await this.inner.getTownCentreRoutes();
    this.stop(t);
    return result;
  }

  async getTownCentrePoints(): Promise<FeatureCollection> {
    let t = this.start("getTownCentrePoints");
    let result = await this.inner.getTownCentrePoints();
    this.stop(t);
    return result;
  }

  async getNetworkLengths(): Promise<NetworkLengths> {
    let t = this.start("getNetworkLengths");
    let result = await this.inner.getNetworkLengths();
    this.stop(t);
    return result;
  }

  private start(method: string): string {
    loadingSpinners.update((x) => {
      return x + 1;
    });
    return method;
  }

  private stop(timer: string) {
    loadingSpinners.update((x) => {
      return x - 1;
    });
  }
}
