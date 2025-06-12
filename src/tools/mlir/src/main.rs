#![feature(dropck_eyepatch)]

fn main() {
    rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<_>>(), &mut MyCompilerCallbacks)
        .run()
        .unwrap();
}
