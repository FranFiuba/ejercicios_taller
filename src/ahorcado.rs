pub struct Ahorcado {
    palabra_actual: Vec<Letra>,
    intentos: i64,
    letras_ingresadas: Vec<char>,
}

enum Letra {
    Descubierta(char),
    Cubierta(char),
}

// Bienvenido al ahorcado de FIUBA!

// La palabra hasta el momento es: _ _ _ _ _ _
// Adivinaste las siguientes letras: 
// Te quedan 5 intentos.
// Ingresa una letra: r

// La palabra hasta el momento es: _ _ _ _ _ r
// Adivinaste las siguientes letras: r
// Te quedan 5 intentos.
// Ingresa una letra: c

impl Ahorcado {
    pub fn new(palabra: String, intentos: i64) -> Ahorcado {
        let palabra_actual = palabra.chars().map(|x| Letra::Cubierta(x)).collect();

        Ahorcado {
            palabra_actual,
            intentos,
            letras_ingresadas: Vec::new(),
        }
    }

    pub fn is_letra(&self, letra: &char) -> bool {
        self.palabra_actual.iter().any(|x| match x {
            Letra::Descubierta(y) => letra == y,
            Letra::Cubierta(y) => letra == y,
        })
    }

    pub fn adivinar_letra(mut self, letra: &char) {
        if self.is_letra(letra) {
            self.descubrir_letra(letra);
        } else {
            self.intentos -= 1;
            self.letras_ingresadas.push(*letra);
        }
    }

    pub fn descubrir_letra(& mut self, letra: &char) {
        for i in 0..self.palabra_actual.len(){
            let letra_actual = &self.palabra_actual[i];
            match *letra_actual {
                Letra::Cubierta(letra_cubierta) =>{
                    if *letra == letra_cubierta {
                        self.palabra_actual[i] = Letra::Descubierta(*letra);
                    }
                },
                _ => {},
            }
        }
        
        
    }

    pub fn mostrar_estado_palabra(&self){
        for letra in self.palabra_actual.iter(){
            match letra {
                Letra::Descubierta(letra_descubierta) => {
                    print!("{}", *letra_descubierta);
                },
                Letra::Cubierta(letra_cubierta) =>{
                    print!("_");
                },
            }
        }
        println!("");
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_letra_se_encuentra_en_palabra() {
        let ahorcado = Ahorcado::new("hola".to_string(), 5);
        assert!(ahorcado.is_letra(&'h'));
    }

    #[test]
    pub fn test_letra_no_se_encuentra_en_palbra() {
        let ahorcado = Ahorcado::new("hola".to_string(), 5);
        assert!(!ahorcado.is_letra(&'j'));
    }

    #[test]
    pub fn test_descubrir_letra() {
        let mut ahorcado = Ahorcado::new("hola".to_string(), 5);
        ahorcado.descubrir_letra(&'h');
        let _letra: char;
        
        let _letra = &ahorcado.palabra_actual[0];

        ahorcado.mostrar_estado_palabra();
        
        assert_eq!(" ","h___");
    }
}