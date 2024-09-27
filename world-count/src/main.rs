use std::io;

fn main() {
    loop {
        println!("Digite a frase que deseja contar as palavras");

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let name_lower = name.to_lowercase();
        let worlds: Vec<&str> = name_lower.split_whitespace().collect();

        let mut vec_words = worlds.clone();
        vec_words.sort();
        println!("{}", worlds.len());
        println!("{:?}", vec_words)
    }
}
