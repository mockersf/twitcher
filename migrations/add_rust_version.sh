#!/bin/bash

set -x

git clone -b results git@github.com:bevyengine/twitcher.git results

stable=`rustc --version`
nightly=`rustc +nightly --version`

for stats in `find ./results -type f -name 'stats.json'`
do
    echo $stats
    tmp=`mktemp`
    jq ".rust.stable = \"$stable\"" < $stats > $tmp
    mv $tmp $stats
    tmp=`mktemp`
    jq ".rust.nightly = \"$nightly\"" < $stats > $tmp
    mv $tmp $stats
done

cd results
git add .
git commit -m "Add commit timestamps"
git push
cd ..
rm -rf results
