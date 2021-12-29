#!/usr/bin/env bash

cd `dirname $(readlink -f "$0")`

# cargo make --profile release release-native

cargo build --release --features native
cargo bundle --release --features native

cd ../../
rm -rf release/osx/notation_viewer_cn.app
cp -vr target/release/bundle/osx/notation_viewer_cn.app/ release/osx/
cd release/osx/notation_viewer_cn.app/Contents/MacOS
ln -s ../Resources/assets .
cd ../../../

rm -rf ~/Applications/notation_viewer_cn.app
cp -vr notation_viewer_cn.app ~/Applications/
