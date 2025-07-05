#!/bin/bash

cargo build --release

git clone -b queue git@github.com:mockersf/twitcher.git queue
gitref=`find ./queue -type f  | grep -v .git  | head -n 1`
gitref=`echo ${gitref#./}`

git clone git@github.com:bevyengine/bevy.git
cd bevy
git reset --hard $gitref

cd bevy
../target/release/twitcher all

cd ..
git clone -b results git@github.com:mockersf/twitcher.git results
cp -r bevy/results results
