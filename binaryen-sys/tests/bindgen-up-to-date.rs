// This is a smoke test that the pre-generated `src/bindings.rs` file doesn't
// need to be updated. We check in a generated version so downstream consumers
// don't have to get `bindgen` working themselves.
//
// If bindgen or binaryen changes you can run tests with `BLESS=1` to regenerate
// the source file, otherwise this will test on CI that the file doesn't need to
// be regenerated.

#[test]
fn test_bindings_up_to_date() {
    let expected = bindgen::Builder::default()
        .header("wrapper.h")
        // See https://github.com/rust-lang-nursery/rust-bindgen/issues/947
        .trust_clang_mangling(false)
        .generate_comments(true)
        // https://github.com/rust-lang-nursery/rust-bindgen/issues/947#issuecomment-327100002
        .layout_tests(false)
        .allowlist_function("Binaryen.*")
        .allowlist_type("Binaryen.*")
        .allowlist_function("TypeBuilder.*")
        .allowlist_type("TypeBuilder.*")
        .allowlist_function("Relooper.*")
        .allowlist_type("Relooper.*")
        .allowlist_function(".*ExpressionRunner.*")
        .allowlist_type(".*ExpressionRunner.*")
        .allowlist_function("translateToFuzz")
        .allowlist_recursively(false)
        .generate()
        .expect("Unable to generate bindings")
        .to_string();

    if std::env::var("BLESS").is_ok() {
        std::fs::write("src/bindings.rs", expected).unwrap();
    } else {
        let actual = include_str!("../src/bindings.rs");
        if expected == actual {
            return;
        }

        for diff in diff::lines(&expected, &actual) {
            match diff {
                diff::Result::Both(_, s) => println!(" {}", s),
                diff::Result::Left(s) => println!("-{}", s),
                diff::Result::Right(s) => println!("+{}", s),
            }
        }

        panic!("differences found, need to regenerate bindings");
    }
}
