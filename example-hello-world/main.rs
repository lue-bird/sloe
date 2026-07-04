mod sloe;

fn main() {
    sloe::origin_new!(result, Result);
    let greeting = sloe::greet(sloe::Record·name·result_origin {
        name: "world",
        result_origin: result,
    });
    print!(
        "{}",
        greeting
            .vec
            .opt_span_slice(greeting.span.as_ref())
            .iter()
            .copied()
            .collect::<String>()
    );
}
