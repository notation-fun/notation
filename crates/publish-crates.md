Note: there is a delay between publish and download, so can't just run this automatically, currently do it manually, the order of packages is important.

```
cd notation_core
cargo publish

cd ../notation_fretted
cargo publish

cd ../notation_guitar
cargo publish

cd ../notation_proto
cargo publish

cd ../notation_model
cargo publish

cd ../notation_dsl
cargo publish

cd ../notation_macro
cargo publish

cd ../notation_tab
cargo publish

cd ../notation_midi
cargo publish

cd ../notation_bevy_utils
cargo publish

cd ../notation_bevy
cargo publish
```
