#!/bin/bash

set -x

git pull

cargo build --release --bin collect

git clone -b queue git@github.com:bevyengine/twitcher.git queue
gitref=`find ./queue -type f  | grep -v .git  | head -n 1`
if [ ! "$gitref" ]
then
    rm -rf queue
    echo "no queued gitref found"
    exit 1
fi
gitref=`echo ${gitref#./queue/}`

git clone git@github.com:bevyengine/bevy.git
cd bevy
git reset --hard $gitref
../target/release/collect all
cd ..

git clone -b results git@github.com:bevyengine/twitcher.git results
cp -r bevy/results/* results
cd results
git add .
git commit -m "Add results for $gitref"
git push
cd ..

cd queue
git pull
rm $gitref
git add .
git commit -m "Done for $gitref"
git push
cd ..

rm -rf queue
rm -rf results
rm -rf bevy
