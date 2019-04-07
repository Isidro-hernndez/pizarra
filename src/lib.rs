/*!
Pizarra pretende ser una herramienta para hacer presentaciones en vivo, por que
creo que tener un pizarrón y gises es lo mejor para explicar cosas.

La mejor forma de utilizar pizarra es teniendo una tabla de dibujo.

# Atajos de teclado

Pizarra por el momento se puede controlar con algunos atajos de teclado que
permiten cambiar la herramienta en uso, colores y otras configuraciones.

## Herramientas de dibujo

* `Ctrl+R` Rectángulo
* `Ctrl+L` Línea

## Colores

* `Alt+G` Verde
* `Alt+R` Rojo
* `Alt+B` Azul
* `Alt+Y` Amarillo
* `Alt+O` Naranja
* `Alt+W` Blanco

# Archivo de salida

Pizarra guarda actualmente un archivo svg en el directorio en que es ejecutada
con el dibujo hecho durante la sesión. El nombre del archivo es único y lleva
la fecha en hora local en que se hizo.

*/

pub mod color;
pub mod poly;
pub mod storage;
pub mod serialize;
pub mod app;

pub use poly::Tool;
pub use app::App;
