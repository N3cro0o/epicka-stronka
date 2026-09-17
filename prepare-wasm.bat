@ECHO OFF

ECHO "jajo"
cd wasm\output\main
del /q .\*
cd ..\..\web-page-main-canvas\
cargo b --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir ..\output\main\  .\target\wasm32-unknown-unknown\release\web_page_main_canvas_bin.wasm
