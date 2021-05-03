pub struct Ahorcado {
    letras: Vec<Letra>,
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
        let longitud_palabra = palabra.len();
        let letras = palabra.chars().map(|x| Letra::Cubierta(x)).collect();

        Ahorcado {
            letras,
            intentos,
            letras_ingresadas: Vec::new(),
        }
    }

    pub fn is_letra(&self, letra: &char) -> bool {
        self.letras.iter().any(|x| match x {
            Letra::Descubierta(y) => letra == y,
            Letra::Cubierta(y) => letra == y,
        })
    }

    pub fn adivinar_letra(&self, letra: &char) {
        if self.is_letra(letra) {
            self.descubrir_letra(letra);
        } else {
            self.intentos -= 1;
            self.letras_ingresadas.push(*letra);
        }
    }

    pub fn descubrir_letra(&self, letra: &char) {
        self.letras.iter().map(|x| match x {
            Letra::Cubierta(y) => {
                if letra == y {
                    Letra::Descubierta(*y)
                } else {
                    Letra::Cubierta(*y)
                }
            },
            Letra::Descubierta(y) => Letra::Descubierta(*y),
        });
    }

    // |pub fn show(&self) {

    // }
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
        let ahorcado = Ahorcado::new("hola".to_string(), 5);
        ahorcado.descubrir_letra(&'h');
        let letra: char;
        if let Letra::Descubierta(x) = ahorcado.letras[0] {
            letra = x;
        }

        assert_eq!('h', letra);
    }
}