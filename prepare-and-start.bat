@ECHO OFF

ECHO "wasm"
cd wasm\output\main
del /q .\*
cd ..\..
cargo b --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir output\main\  .\target\wasm32-unknown-unknown\release\web_page_main_canvas_bin.wasm

ECHO "start"
cd ..
simple-http-server -i -o --nocache .
