default:
  just --list

doc:
    cargo doc --document-private-items --open


test-node:
    cd crates/node-package && npm run build && npm run test

run-example:
    cd crates/node-package && npm run build && npm run example
