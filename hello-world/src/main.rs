#[derive(Debug)] //ativa o debug para struct
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter); // faz o print da struct de forma formatada
}
