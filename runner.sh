#!/bin/bash

set -x

cargo build --release

git clone -b queue git@github.com:mockersf/twitcher.git queue
gitref=`find ./queue -type f  | grep -v .git  | head -n 1`
if [ ! "$gitref" ]
then
    echo "no queued gitref found"
    exit 1
fi
gitref=`echo ${gitref#./queue/}`

git clone git@github.com:bevyengine/bevy.git
cd bevy
git reset --hard $gitref
../target/release/twitcher all
cd ..

git clone -b results git@github.com:mockersf/twitcher.git results
cp -r bevy/results/* results
cd results
git add .
git commit -m "Add results for $gitref"
git push
cd ..

cd queue
rm $gitref
git add .
git commit -m "Done for $gitref"
git push
cd ..

rm -rf queue
rm -rf results
rm -rf bevy
