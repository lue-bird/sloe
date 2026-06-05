mod sloe;

fn main() {
    let greeting = sloe::example(sloe::Blank {});
    print!("{}", greeting);
}
