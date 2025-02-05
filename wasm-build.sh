#!/usr/bin/env sh
# wasm_js
#/bin/bash -l -c "export PATH=\"/home/janc/.cargo/bin:$PATH\" && cd /home/janc/mcg-visual && export RUSTFLAGS='--cfg getrandom_backend=\"wasm_js\"' && /home/janc/.cargo/bin/wasm-pack build --target web -- '--color=always'"
# js
/bin/bash -l -c "export PATH=\"/home/janc/.cargo/bin:$PATH\" && cd /home/janc/mcg-visual && /home/janc/.cargo/bin/wasm-pack build --target web -- '--color=always'"
