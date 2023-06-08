fn main() {
    cc::Build::new()
        .file("src/crust.c")
        .compile("crust");
}