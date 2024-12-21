// Returns the local storage key for a file
export function getKey(boundary: string, filename: string): string {
  return `npw/${boundary}/${filename}`;
}

export function getLastOpenedFileKey(boundary: string): string {
  return `npw/${boundary}/last-opened-file`;
}

// Returns [filename, summary]
export function listFilesInBoundary(boundary: string): [string, string][] {
  let prefix = `npw/${boundary}/`;

  let list: [string, string][] = [];
  for (let i = 0; i < window.localStorage.length; i++) {
    let key = window.localStorage.key(i)!;
    if (key.startsWith(prefix)) {
      let filename = key.slice(prefix.length);
      if (filename == "last-opened-file") {
        continue;
      }
      try {
        let state = JSON.parse(window.localStorage.getItem(key)!);
        list.push([filename, describe(state)]);
      } catch (err) {}
    }
  }
  list.sort();
  return list;
}

// Returns boundary => list of [filename, summary]
export function listAllFiles(): Map<string, [string, string][]> {
  let map = new Map();
  for (let i = 0; i < window.localStorage.length; i++) {
    let key = window.localStorage.key(i)!;
    if (key.startsWith("npw/") && !key.endsWith("/last-opened-file")) {
      try {
        let state = JSON.parse(window.localStorage.getItem(key) || "");
        let description = describe(state);

        let [_, boundary, filename] = key.split("/");

        if (!map.has(boundary)) {
          map.set(boundary, []);
        }
        map.get(boundary).push([filename, description]);
      } catch (_) {}
    }
  }

  for (let list of map.values()) {
    list.sort();
  }
  return map;
}

function describe(state: any): string {
  let numRoutes = Object.keys(state.routes).length;
  return `${numRoutes} routes`;
}
