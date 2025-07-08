#!/bin/bash

set -x

git clone -b results git@github.com:mockersf/twitcher.git results

for stats in `find ./results -type f -name 'stats.json'`
do
    echo $stats

    if cat $stats | grep 'native.size'
    then
        tmp=`mktemp`
        jq '.metrics.["native-unix-x86_64.size"] = .metrics.["native.size"]' < $stats > $tmp
        mv $tmp $stats
        tmp=`mktemp`
        jq 'del(.metrics.["native.size"])' < $stats > $tmp
        mv $tmp $stats
    fi
done

cd results
git add .
git commit -m "Rename native.size field"
git push
cd ..
rm -rf results
