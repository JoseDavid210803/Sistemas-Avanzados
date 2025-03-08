fn main() {
    
    
    //let x = 5;
   // x = 10; No es Valido

    //let mut y = 10;
    //y=20;
    let x=5;
    let x = x+1; //Nueva variable
    println!("El valor de x es: {}", x);

    //Variables
    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool =true;
    let caracter: char ='a';
//Tupla -> structs // creacion de tupla llamada Firulais
    let firulais: (i32, f64, char) =(43, 4.1416, 'b');
    println!("Tupla(Firulais) forma1: {:?}", firulais);
    println!("Tupla(firulais) forma2: ({}, {}, {})", firulais.0, firulais.1, firulais.2);

    let arreglo: [i32; 3]=[1,2,3];




}
