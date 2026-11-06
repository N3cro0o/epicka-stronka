@ECHO OFF

ECHO "prep"
cd wasm\output\main
del /q .\*
cd ..\bg
del /q .\*
cd ..\..
ECHO "wasm"
cargo b --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir output\main\  .\target\wasm32-unknown-unknown\release\web_page_main_canvas_bin.wasm
wasm-bindgen --target web --out-dir output\bg\  .\target\wasm32-unknown-unknown\release\web_page_background_canvas.wasm

ECHO "start"
cd ..
simple-http-server -i -o --nocache .
