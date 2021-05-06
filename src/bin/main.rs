use ejercicios_taller::Ahorcado;
use std::io::{self, Write};
use std::fs;

fn main() {
    let mut palabra = fs::read_to_string("file.txt").expect("Error leyendo palabra");
    palabra.pop();
    let mut ahorcado = Ahorcado::new(&palabra, 5);

    println!("");
    println!("Bienvenido al ahorcado de FIUBA!\n");
    println!("{}", ahorcado);

    ahorcado.comenzar();

    while !ahorcado.finalizado() {
        let mut input = String::new();

        print!("Ingresa una letra:");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Error leyendo la letra.\n");

        if input_valido(&input) {
            let letra = input.trim().chars().next().unwrap();
            ahorcado.adivinar_letra(&letra);
            println!("{}", ahorcado);
        } else {
            println!("No se ingreso un dato valido\n");
        }
    }
}

fn input_valido(input: &str) -> bool {
    input.trim().len() == 1 && input.chars().next().unwrap().is_alphabetic()
}
