fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/xbyak.cpp")
        .include("xbyak")
        .flag("-EHsc")
        .compile("xbyak");
}
