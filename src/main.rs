mod ahorcado;

use crate::ahorcado::Ahorcado;

fn main() {
   	let mut ahorcado = Ahorcado::new("hola".to_string(), 5);
    ahorcado.descubrir_letra(&'h');
    ahorcado.descubrir_letra(&'a');
    ahorcado.mostrar_estado_palabra();
}
