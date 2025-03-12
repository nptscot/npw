set -e
set -x

cd web
mkdir deployme
for branch in main demo; do
  echo "Building $branch"
  git checkout $branch
  if [ "$branch" == "main" ]; then
    base=/npw/
  else
    base=/npw/$branch
  fi

  npm ci
  npm run wasm-release
  npm run build -- --base=$base
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
