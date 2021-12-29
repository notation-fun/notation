cargo build --release --features native

cd ..\..\
bash -c "rm -rf release/windows/notation_viewer_cn"
bash -c "mkdir -p release/windows/notation_viewer_cn"
bash -c "cp -v target/release/notation_viewer_cn.exe release/windows/notation_viewer_cn/"
bash -c "cp -vr apps/notation_viewer_cn/assets release/windows/notation_viewer_cn/"
bash -c "rm -rf ~/win/Applications/notation_viewer_cn"
bash -c "cp -vr release/windows/notation_viewer_cn/ ~/win/Applications/notation_viewer_cn/"
cd apps\notation_viewer_cn
