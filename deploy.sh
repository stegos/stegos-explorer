#!/usr/bin/env sh

# Stop publish on error
set -e
rm dist -rf
mkdir dist
cp index.html dist/
cd dist

echo "explorer.stegos.com" > CNAME
git init
git add -A
git commit -m 'deploy'

git push -f git@github.com:stegos/explorer.git master

