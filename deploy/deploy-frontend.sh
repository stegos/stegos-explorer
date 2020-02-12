#!/usr/bin/env sh

# Stop publish on error
set -e
source ~/.env.prod

yarn build

cd dist
echo "explorer.stegos.com" > CNAME

git init
git add -A
git commit -m 'deploy'

git push -f git@github.com:stegos/explorer.git master
