set -e
set -x

cd web
mkdir deployme
for branch in main demo; do
  echo "Building $branch"
  git checkout $branch

  npm ci
  npm run wasm-release

  if [ "$branch" == "main" ]; then
    npm run build -- --base=/npw/
  else
    VITE_OD2NET_DIR="demo_npw" npm run build -- --base=/npw/$branch
  fi

  # TODO Vite suddenly broke and I have no idea why. Hack around it.
  # new URL('backend_bg.wasm', import.meta.url) doesn't exist at build time, it will remain unchanged to be resolved at runtime
  ln -s assets/backend_bg.wasm dist/

  if [ "$branch" == "main" ]; then
    mv dist/* deployme
  else
    mv dist deployme/$branch
  fi
done

# Just for the benefit of running this locally. On GH actions, it doesn't matter
git checkout main
