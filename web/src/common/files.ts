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
        let value = JSON.parse(window.localStorage.getItem(key)!);
        let numRoutes = Object.keys(value.routes).length;
        list.push([filename, `${numRoutes} routes`]);
      } catch (err) {}
    }
  }
  list.sort();
  return list;
}
