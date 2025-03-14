// Sets a local storage item. If quota is exceeded, then redirect to a page
// where the user can clean things up.
export function setLocalStorage(key: string, value: string) {
  try {
    window.localStorage.setItem(key, value);
  } catch (err) {
    console.log(`Couldn't set local storage for ${key}: ${err}`);
    window.alert(
      "Your changes couldn't be saved because you've run out of local storage. Please fix this problem on the next page and try again.",
    );
    window.location.href = "local_storage.html";
  }
}

// Returns the local storage key for a file
export function getKey(boundary: string, filename: string): string {
  return `npw/${boundary}/${filename}`;
}

// Returns [filename, summary]
export function listFilesInBoundary(boundary: string): [string, string][] {
  let prefix = `npw/${boundary}/`;

  let list: [string, string][] = [];
  for (let i = 0; i < window.localStorage.length; i++) {
    let key = window.localStorage.key(i)!;
    if (key.startsWith(prefix)) {
      let filename = key.slice(prefix.length);
      // TODO Legacy, remove for v1
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
    // TODO Legacy, remove for v1
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

// Returns all local storage items, sorted by size (in bytes) descending.
export function measureLocalStorageSizes(): [string, number][] {
  let list: [string, number][] = [];
  for (let i = 0; i < window.localStorage.length; i++) {
    let key = window.localStorage.key(i)!;
    list.push([key, window.localStorage.getItem(key)!.length]);
  }
  // Sort by size descending
  list.sort((a, b) => b[1] - a[1]);
  return list;
}
