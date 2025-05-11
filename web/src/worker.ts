import { MapModel } from "backend";
import * as Comlink from "comlink";

  // Intended to be called immediately after constructing
  /*async loadFile(graphBytes: Uint8Array) {
    // TODO Do we need to do this only once?
    await init();

    this.inner = new MapModel(graphBytes);
  }*/

Comlink.expose(MapModel);
