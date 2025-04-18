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
  SlowStats,
  StaticRoad,
  Stats,
  Tier,
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
    this.start();
    let result = await this.inner.getBounds();
    this.stop();
    return result;
  }

  async getInvertedBoundaryForStudyArea(): Promise<Feature<Polygon>> {
    this.start();
    let result = await this.inner.getInvertedBoundaryForStudyArea();
    this.stop();
    return result;
  }

  async getInvertedBoundaryInsideSettlements(): Promise<Feature<Polygon>> {
    this.start();
    let result = await this.inner.getInvertedBoundaryInsideSettlements();
    this.stop();
    return result;
  }

  async getInvertedBoundaryOutsideSettlements(): Promise<
    Feature<MultiPolygon>
  > {
    this.start();
    let result = await this.inner.getInvertedBoundaryOutsideSettlements();
    this.stop();
    return result;
  }

  async getStudyAreaBoundary(): Promise<Feature<Polygon>> {
    this.start();
    let result = await this.inner.getStudyAreaBoundary();
    this.stop();
    return result;
  }

  async renderStaticRoads(): Promise<
    FeatureCollection<LineString, StaticRoad>
  > {
    this.start();
    let result = await this.inner.renderStaticRoads();
    this.stop();
    return result;
  }

  async renderDynamicRoads(): Promise<DynamicRoad[]> {
    this.start();
    let result = await this.inner.renderDynamicRoads();
    this.stop();
    return result;
  }

  async getAllRoutes(): Promise<
    FeatureCollection<LineString, RouteProps> & {
      id_counter: number;
    }
  > {
    this.start();
    let result = await this.inner.getAllRoutes();
    this.stop();
    return result;
  }

  async getRoute(id: number): Promise<Feature<LineString, RouteProps>> {
    this.start();
    let result = await this.inner.getRoute(id);
    this.stop();
    return result;
  }

  async setRoute(id: number | null, input: SetRouteInput) {
    this.start();
    await this.inner.setRoute(id, input);
    this.stop();
  }

  async deleteRoutes(ids: number[]) {
    this.start();
    await this.inner.deleteRoutes(ids);
    this.stop();
  }

  async changeTier(routeIds: number[], tier: string) {
    this.start();
    await this.inner.changeTier(routeIds, tier);
    this.stop();
  }

  async changeInfraType(routeIds: number[], infraType: string) {
    this.start();
    await this.inner.changeInfraType(routeIds, infraType);
    this.stop();
  }

  async clearAllRoutes() {
    this.start();
    await this.inner.clearAllRoutes();
    this.stop();
  }

  async autosplitRoute(
    editingRouteId: number | null,
    waypoints: Waypoint[],
    overrideInfraType: string | null,
    defaultTier: Tier,
    majorSnapThreshold: number | null,
  ): Promise<AutosplitRoute> {
    this.start();
    let result = await this.inner.autosplitRoute(
      editingRouteId,
      waypoints,
      overrideInfraType,
      defaultTier,
      majorSnapThreshold,
    );
    this.stop();
    return result;
  }

  async evaluateRoute(req: {
    // TODO LngLatLike doesn't work?
    start: { lng: number; lat: number };
    end: Position;
    breakdown: "" | "los" | "infra_type" | "gradient";
  }): Promise<RouteGJ> {
    this.start();
    let result = await this.inner.evaluateRoute(req);
    this.stop();
    return result;
  }

  // Needs loading screen
  async evaluateOD(fastSample: boolean): Promise<EvaluateODOut> {
    this.start();
    let result = await this.inner.evaluateOD(fastSample);
    this.stop();
    return result;
  }

  // Fast
  async recalculateStats(): Promise<Stats> {
    this.start();
    let result = await this.inner.recalculateStats();
    this.stop();
    return result;
  }

  async recalculateSlowStats(): Promise<SlowStats> {
    this.start();
    let result = await this.inner.recalculateSlowStats();
    this.stop();
    return result;
  }

  async getBaselineStats(): Promise<Stats> {
    this.start();
    let result = await this.inner.getBaselineStats();
    this.stop();
    return result;
  }

  // Needs loading screen
  async recalculateODStats(): Promise<ODStats> {
    this.start();
    let result = await this.inner.recalculateODStats();
    this.stop();
    return result;
  }

  async getGridMeshDensity(
    resolution: number,
    xOffset: number,
    yOffset: number,
  ): Promise<GridMeshDensity> {
    this.start();
    let result = await this.inner.getGridMeshDensity(
      resolution,
      xOffset,
      yOffset,
    );
    this.stop();
    return result;
  }

  // Needs loading screen
  async importExistingRoutes(kind: "infra-type" | "los"): Promise<number> {
    this.start();
    let result = await this.inner.importExistingRoutes(kind);
    this.stop();
    return result;
  }

  // Needs loading screen
  async importCoreNetwork(): Promise<number> {
    this.start();
    let result = await this.inner.importCoreNetwork();
    this.stop();
    return result;
  }

  async loadSavefile(contents: string) {
    this.start();
    await this.inner.loadSavefile(contents);
    this.stop();
  }

  async getPOIs(): Promise<POIs> {
    this.start();
    let result = await this.inner.getPOIs();
    this.stop();
    return result;
  }

  async getTownCentres(): Promise<TownCentres> {
    this.start();
    let result = await this.inner.getTownCentres();
    this.stop();
    return result;
  }

  async getSettlements(): Promise<Settlements> {
    this.start();
    let result = await this.inner.getSettlements();
    this.stop();
    return result;
  }

  async getGreenspaces(): Promise<Greenspaces> {
    this.start();
    let result = await this.inner.getGreenspaces();
    this.stop();
    return result;
  }

  async getDataZones(): Promise<DataZones> {
    this.start();
    let result = await this.inner.getDataZones();
    this.stop();
    return result;
  }

  async debugReachablePath(
    kind: string,
    idx: number,
  ): Promise<FeatureCollection> {
    this.start();
    let result = await this.inner.debugReachablePath(kind, idx);
    this.stop();
    return result;
  }

  // TODO Unused
  async debugUnreachablePath(
    kind: string,
    idx: number,
  ): Promise<FeatureCollection> {
    this.start();
    let result = await this.inner.debugUnreachablePath(kind, idx);
    this.stop();
    return result;
  }

  async fixUnreachablePOI(
    kind: string,
    idx: number,
  ): Promise<Feature<LineString, SetRouteInput>> {
    this.start();
    let result = await this.inner.fixUnreachablePOI(kind, idx);
    this.stop();
    return result;
  }

  async getConnectedComponents(): Promise<ConnectedComponents> {
    this.start();
    let result = await this.inner.getConnectedComponents();
    this.stop();
    return result;
  }

  async snapPoint(
    pt: number[],
    majorSnapThreshold: number | null,
  ): Promise<[number, number]> {
    this.start();
    let result = await this.inner.snapPoint(pt, majorSnapThreshold);
    this.stop();
    return result;
  }

  async getExtraNodes(
    waypt1: Waypoint,
    waypt2: Waypoint,
    majorSnapThreshold: number | null,
  ): Promise<[number, number, boolean][]> {
    this.start();
    let result = await this.inner.getExtraNodes(
      waypt1,
      waypt2,
      majorSnapThreshold,
    );
    this.stop();
    return result;
  }

  async getMajorJunctions(): Promise<FeatureCollection> {
    this.start();
    let result = await this.inner.getMajorJunctions();
    this.stop();
    return result;
  }

  private start() {
    loadingSpinners.update((x) => {
      return x + 1;
    });
  }

  private stop() {
    loadingSpinners.update((x) => {
      return x - 1;
    });
  }
}
