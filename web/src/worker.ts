import init, { MapModel } from "backend";
import * as Comlink from "comlink";

async function main() {
  console.log("web worker waking up, calling init");
  await init();
  console.log("Now expose");
  Comlink.expose(MapModel);
  console.log("done?");
}

main();
