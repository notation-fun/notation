#!/usr/bin/env bash

cd `dirname $(readlink -f "$0")`

cargo make release-native
cd ../../
rm -rf ../../target/release/osx/notation_viewer.app
cp -vr target/release/bundle/osx/notation_viewer.app/ release/osx/
cd release/osx/notation_viewer.app/Content/MacOS
ln -s ../Resources/assets .
cd ../../

