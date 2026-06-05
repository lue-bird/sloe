mod sloe;

fn main() {
    sloe::origin_new!(result, Result);
    let greeting = sloe::example(sloe::Blank);
    print!("{}", greeting);
}
