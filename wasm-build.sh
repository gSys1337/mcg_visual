#!/usr/bin/env sh
# wasm_js; if rand crate fixes its dependencies, more specific: if it doesn't depend on getrandom <0.3
#/bin/bash -l -c "export PATH=\"/home/janc/.cargo/bin:$PATH\" && cd /home/janc/mcg-visual && export RUSTFLAGS='--cfg getrandom_backend=\"wasm_js\"' && /home/janc/.cargo/bin/wasm-pack build --target web -- '--color=always'"
# js; if get random 0.2.15 is dependency
/bin/bash -l -c "export PATH=\"/home/janc/.cargo/bin:$PATH\" && cd /home/janc/mcg-visual && /home/janc/.cargo/bin/wasm-pack build --target web -- '--color=always'"
