#!/usr/bin/env bash

cd `dirname $(readlink -f "$0")`

cargo build --release --features native
cargo bundle --release --features native

cd ../../
rm -rf release/osx/notation_viewer.app
cp -vr target/release/bundle/osx/notation_viewer.app/ release/osx/
cd release/osx/notation_viewer.app/Contents/MacOS
ln -s ../Resources/assets .
cd ../../../

rm -rf ~/Applications/notation_viewer.app
cp -vr notation_viewer.app ~/Applications/
