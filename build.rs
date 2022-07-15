fn main() {
    println!("cargo:rustc-link-lib=user32");
    cc::Build::new()
        .file("src/main.c")
        .compile("main");
}