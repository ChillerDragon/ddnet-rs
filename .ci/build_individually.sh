#!/bin/bash

cargo build -p api
cargo build -p base
cargo build -p base-log
cargo build -p graphics-types
cargo build -p native
cargo build -p ui-base
cargo build -p wasm-runtime
cargo build -p api-macros
cargo build -p base-fs
cargo build -p cache
cargo build -p graphics
cargo build -p hiarc
cargo build -p network
cargo build -p ui-generic
cargo build -p wasm-runtime-types
cargo build -p api-ui
cargo build -p base-http
cargo build -p config
cargo build -p graphics-backend
cargo build -p hiarc-macro
cargo build -p pool
cargo build -p ui-wasm-manager
cargo build -p api-wasm-macros
cargo build -p base-io
cargo build -p config-fs
cargo build -p graphics-backend-traits
cargo build -p image
cargo build -p sound
cargo build -p wasm-logic-fs
cargo build -p av-encoder
cargo build -p base-io-traits
cargo build -p config-macro
cargo build -p graphics-base-traits
cargo build -p math
cargo build -p sql
cargo build -p wasm-logic-graphics

cargo build -p api-state
cargo build -p client-containers
cargo build -p client-map
cargo build -p client-render-base
cargo build -p client-ui
cargo build -p game-config-fs
cargo build -p server
cargo build -p shared
cargo build -p vanilla
cargo build -p game-state-wasm
cargo build -p api-ui-game
cargo build -p client-extra
cargo build -p client-render
cargo build -p client-types
cargo build -p game-config
cargo build -p master-server-types
cargo build -p game-database
cargo build -p game-base
cargo build -p game-network