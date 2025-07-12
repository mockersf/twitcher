#!/bin/bash

set -x

git clone git@github.com:bevyengine/bevy

git clone -b results git@github.com:bevyengine/twitcher.git results

cd bevy
for stats in `find ../results -type f -name 'stats.json'`
do
    echo $stats
    commit=`jq --raw-output ".commit"< $stats`
    ts=`git show --no-patch --format=%ct $commit`
    ts=$((ts * 1000))

    tmp=`mktemp`
    jq ".commit_timestamp = $ts" < $stats > $tmp
    mv $tmp $stats
done
cd ..

rm -rf bevy

cd results
git add .
git commit -m "Add commit timestamps"
git push
cd ..
rm -rf results
