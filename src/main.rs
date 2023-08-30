fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input = &args[1];
    let output = rustc_demangle::demangle(input).to_string();
    println!("{output}");
}
