#!/bin/bash
echo "//registry.npmjs.org/:_authToken=$NPM_AUTH_TOKEN" > .npmrc

VERSION=$(node -pe 'JSON.parse(process.argv[1]).version.indexOf("beta")' "$(cat lerna.json)")

if [ "$VERSION" = "-1" ]
then
  npx lerna publish from-package --yes
else
  echo "version is beta, using --dist-tag next"
  npx lerna publish from-package --dist-tag next --yes
fi

rm .npmrc