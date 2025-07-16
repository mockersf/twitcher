#!/bin/bash

set -x

git clone -b results git@github.com:bevyengine/twitcher.git results

hostname=`hostname`
os_version=`uname -r`

for stats in `find ./results -type f -name 'stats.json'`
do
    echo $stats
    tmp=`mktemp`
    jq ".host.hostname = \"$hostname\"" < $stats > $tmp
    mv $tmp $stats
    tmp=`mktemp`
    jq ".host.os_version = \"$os_version\"" < $stats > $tmp
    mv $tmp $stats
done

cd results
git add .
git commit -m "Add host infos"
git push
cd ..
rm -rf results
