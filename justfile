default:
  just --list

doc:
    cargo doc --document-private-items --open

# https://nexte.st/index.html
test-rust:
    cargo nextest run

test-node:
    cd crates/node-package && npm run build && npm run test

run-example:
    cd crates/node-package && npm run build && npm run example
