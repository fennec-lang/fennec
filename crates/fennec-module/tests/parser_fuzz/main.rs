fn main() {
    bolero::check!().for_each(|input: &[u8]| {
        // Generating valid UTF-8 is a lot of wasted effort, however rewriting
        // all of the parsing code to operate on byte-strings is a lot of effort
        // that is for now unclear if is *really* beneficial.

        if let Ok(input) = std::str::from_utf8(input) {
            let _ = fennec_module::extract(input);
        }
    });
}
