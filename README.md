# Network Planning Workspace

## Developer instructions

You'll need
[npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) and
[wasm-pack](https://github.com/rustwasm/wasm-pack)

Clone the repo. `cd web`, and then:

- `npm ci` to install dependencies (`ci` to make sure the versions in
  `package-lock.json` are used)
- `npm run wasm` to rebuild the Rust backend
  - vite doesn't automatically rebuild when you edit things
- `npm run dev` to run locally
  - Changes to the Svelte/CSS usually auto-reload in your browser
- `npm run fmt` to auto-format code
- `npm run check` to see TypeScript errors

This will fetch all data files from the live site by default, which is slow.
You can set up `web/public`, but I haven't described how to do this yet.
