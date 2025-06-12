./x.py build --stage 1 src/tools/mlir

strings build/aarch64-apple-darwin/stage1/bin/rustc | grep CFG_RELEASE_CHANNEL