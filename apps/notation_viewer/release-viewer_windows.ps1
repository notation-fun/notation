cargo build --release --features native

cd ..\..\
bash -c "rm -rf release/windows/notation_viewer"
bash -c "mkdir -p release/windows/notation_viewer"
bash -c "cp -v target/release/notation_viewer.exe release/windows/notation_viewer/"
bash -c "cp -vr apps/notation_viewer/assets release/windows/notation_viewer/"
bash -c "rm -rf ~/win/Applications/notation_viewer"
bash -c "cp -vr release/windows/notation_viewer/ ~/win/Applications/notation_viewer/"
cd apps\notation_viewer
