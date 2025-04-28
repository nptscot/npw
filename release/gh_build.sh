set -e
set -x

cd web
mkdir deployme
for branch in main demo nptless_demo; do
  echo "Building $branch"
  git checkout $branch

  npm ci
  npm run wasm-release

  if [ "$branch" == "main" ]; then
    npm run build -- --base=/npw/
  else
    npm run build -- --base=/npw/$branch
  fi

  # TODO Vite suddenly broke and I have no idea why. Hack around it.
  # new URL('backend_bg.wasm', import.meta.url) doesn't exist at build time, it will remain unchanged to be resolved at runtime
  ln -s assets/backend_bg.wasm dist/

  # Writes to dist/. If the script is missing in some old branches, don't fail.
  npm run generate-manual || echo

  if [ "$branch" == "main" ]; then
    mv dist/* deployme
  else
    mv dist deployme/$branch
  fi
done

# Just for the benefit of running this locally. On GH actions, it doesn't matter
git checkout main
