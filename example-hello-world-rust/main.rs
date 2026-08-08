mod sloe;

fn main() {
    sloe::origin_new!(result, Result);
    let greeting = sloe::greet(sloe::Record·name·result_origin {
        name: sloe::Str::from_str("world").unwrap(),
        result_origin: result,
    });
    print!(
        "{}",
        greeting
            .buf
            .span_slice(&greeting.span)
            .iter()
            .copied()
            .collect::<String>()
    );
}
