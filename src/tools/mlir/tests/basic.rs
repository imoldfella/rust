#![feature(rustc_private)]

#[test]

fn print_hir_for_sample() {
    let sample = r#"
        fn main() { println!("hello world"); }
    "#;
    // Path to a temp file with this source
    use std::fs::File;
    use std::io::Write;
    use std::env;
    let tmp_dir = env::temp_dir();
    let src_path = tmp_dir.join("basic.rs");
    let mut file = File::create(&src_path).unwrap();
    file.write_all(sample.as_bytes()).unwrap();

    // Prepare the args, as if we called: mlir mlir_test.rs --crate-name test --edition=2024
    let args = vec![
        "mlir".to_string(),
        src_path.to_str().unwrap().to_string(),
        "--crate-name".to_string(), "test".to_string(),
        "--edition=2024".to_string(),
    ];

    // Call your main function (from main.rs)
    mlir::main_with_args(args);
}