pub struct Ahorcado {
    letras: Vec<Letra>,
    intentos: i64,
    letras_ingresadas: Vec<char>,
}

enum Letra {
    Descubierta(char),
    Cubierta(char),
}

impl Ahorcado {
    pub fn new(palabra: String) -> Ahorcado {
        let longitud_palabra = palabra.len();
        let letras = palabra.chars().map(|x| Letra::Cubierta(x)).collect();

        Ahorcado {
            letras,
            intentos: MAX_INTENTOS,
            letras_ingresadas: Vec::new(),
        }
    }

    pub fn is_letra(&self, letra: &char) -> bool {
        self.letras.iter().any(|x| match x {
            Letra::Descubierta(y) => letra == y,
            Letra::Cubierta(y) => letra == y,
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_letra_se_encuentra_en_palabra() {
        let ahorcado = Ahorcado::new("hola".to_string());
        assert!(ahorcado.is_letra(&'h'));
    }

    #[test]
    pub fn test_letra_no_se_encuentra_en_palbra() {
        let ahorcado = Ahorcado::new("hola".to_string());
        assert!(!ahorcado.is_letra(&'j'));
    }
}
