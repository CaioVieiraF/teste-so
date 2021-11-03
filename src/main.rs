use rand;
use std::io::{self, Write};

fn main() {
    let number = rand::random::<u8>();

    println!("Eu pensei em um número entre 0 e 255,");
    let guess = input("Qual você acha que é? ").expect("Erro ao ler entrada!");
    let guess = guess
        .parse::<u8>()
        .expect("O valor inserido deve ser um número.");

    if number == guess {
        println!("Parabéns! Você acertou o número!");
    } else {
        println!("Opa! o número era {}, não {}.", number, guess);
    }
}

fn input(msg: &str) -> Result<String, io::Error> {
    print!("{}", msg);
    io::stdout().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    Ok(s.trim().to_owned())
}
