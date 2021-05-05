use std::fmt;

pub struct Ahorcado {
    letras: Vec<Letra>,
    intentos: i64,
    letras_acertadas: Vec<char>,
    letras_fallidas: Vec<char>,
    estado: EstadoDelJuego,
}

enum Letra {
    Descubierta(char),
    Cubierta(char),
}

pub enum EstadoDelJuego {
    Inicio,
    EnProgreso,
    Victoria,
    Derrota,
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
    pub fn new(palabra: &str, intentos: i64) -> Ahorcado {
        let letras = palabra
            .to_string()
            .chars()
            .map(|x| Letra::Cubierta(x))
            .collect();

        Ahorcado {
            letras,
            intentos,
            estado: EstadoDelJuego::Inicio,
            letras_acertadas: Vec::new(),
            letras_fallidas: Vec::new(),
        }
    }

    pub fn comenzar(&mut self) {
        self.estado = EstadoDelJuego::EnProgreso;
    }

    pub fn is_letra(&self, letra: &char) -> bool {
        self.letras.iter().any(|x| match x {
            Letra::Descubierta(y) => letra == y,
            Letra::Cubierta(y) => letra == y,
        })
    }

    pub fn adivinar_letra(&mut self, letra: &char) {
        if self.is_letra(letra) {
            self.descubrir_letra(letra);
            self.letras_acertadas.push(*letra);

            if self.victoria() {
                self.estado = EstadoDelJuego::Victoria;
            }
        } else {
            self.intentos -= 1;
            self.letras_fallidas.push(*letra);

            if self.derrota() {
                self.estado = EstadoDelJuego::Derrota;
            }
        }
    }

    pub fn descubrir_letra(&mut self, nueva_letra: &char) {
        self.letras.iter_mut().for_each(|letra| {
            if let Letra::Cubierta(val) = *letra {
                if val == *nueva_letra {
                    *letra = Letra::Descubierta(val);
                }
            }
        });
    }
    pub fn letra_descubierta(&self, letra: &char) -> bool {
        self.letras.iter().any(|x| match x {
            Letra::Descubierta(y) => letra == y,
            Letra::Cubierta(_) => false,
        })
    }

    pub fn finalizado(&self) -> bool {
        match self.estado {
            EstadoDelJuego::Derrota | EstadoDelJuego::Victoria => true,
            _ => false,
        }
    }

    fn victoria(&self) -> bool {
        self.letras.iter().all(|l| match l {
            Letra::Descubierta(_) => true,
            _ => false,
        })
    }

    fn derrota(&self) -> bool {
        self.intentos == 0
    }
}

impl fmt::Display for Ahorcado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palabra = self
            .letras
            .iter()
            .fold("".to_string(), |mut palabra, x| match *x {
                Letra::Descubierta(l) => {
                    palabra.push(l);
                    palabra + " "
                }
                Letra::Cubierta(_) => palabra + "_ ",
            });

        match self.estado {
            EstadoDelJuego::Inicio => write!(
                f,
                "Debes descubir la siguiente palabra: {} \n\
                Te quedan {} intentos.\n",
                palabra, self.intentos
            ),

            EstadoDelJuego::Victoria => write!(
                f,
                "Ganaste\n\
                La palabra era: {}\n\
                Te quedaron {} intentos\n",
                palabra, self.intentos
            ),

            EstadoDelJuego::Derrota => write!(
                f,
                "Perdiste\n\
                Te quedaste sin Intentos\n"
            ),

            EstadoDelJuego::EnProgreso => {
                let concatenar_letras = |mut palabra: String, x: &char| -> String {
                    palabra.push(*x);
                    palabra + " "
                };
                let letras_acertadas = self
                    .letras_acertadas
                    .iter()
                    .fold("".to_string(), concatenar_letras);
                let letras_fallidas = self
                    .letras_fallidas
                    .iter()
                    .fold("".to_string(), concatenar_letras);
                write!(
                    f,
                    "La palabra hasta el momento es: {} \n\
                    Adivinaste las siguientes letras: {} \n\
                    Fallaste con las siguientes letras: {} \n\
                    Te quedan {} intentos.\n",
                    palabra, letras_acertadas, letras_fallidas, self.intentos
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_letra_se_encuentra_en_palabra() {
        let ahorcado = Ahorcado::new("hola", 5);
        assert!(ahorcado.is_letra(&'h'));
    }

    #[test]
    pub fn test_letra_no_se_encuentra_en_palbra() {
        let ahorcado = Ahorcado::new("hola", 5);
        assert!(!ahorcado.is_letra(&'j'));
    }

    #[test]
    pub fn test_descubrir_letra() {
        let mut ahorcado = Ahorcado::new("hola", 5);
        ahorcado.descubrir_letra(&'h');
        assert!(ahorcado.letra_descubierta(&'h'));
    }

    #[test]
    pub fn test_letra_descubierta() {
        let mut ahorcado = Ahorcado::new("hola", 5);
        ahorcado.letras[0] = Letra::Descubierta('h');
        assert!(ahorcado.letra_descubierta(&'h'));
    }
}
