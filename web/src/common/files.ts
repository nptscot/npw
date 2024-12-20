// Returns the local storage key for a file
export function getKey(boundary: string, filename: string): string {
  return `npw/${boundary}/${filename}`;
}
