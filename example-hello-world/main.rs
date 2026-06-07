mod sloe;

fn main() {
    sloe::origin_new!(result, Result);
    let greeting = sloe::vec_opt_span_build(sloe::greet(sloe::Name·result_origin {
        name: "world",
        result_origin: result,
    }));
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
