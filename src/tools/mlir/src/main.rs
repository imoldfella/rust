

#![feature(rustc_private)]


fn main() {
    let args: Vec<String> = std::env::args().collect();
    mlir::main_with_args(args);
}