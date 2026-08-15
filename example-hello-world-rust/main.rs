mod sloe;

fn main() {
    sloe::origin_new!(result, Result);
    let greeting = sloe::greet(sloe::Record·buf·name {
        name: sloe::Str::from_str("world").unwrap(),
        buf: sloe::Buf::new(result),
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
