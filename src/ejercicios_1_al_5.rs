
pub fn suma_multiplos_5_and_9 (number: u32){


    let mut result_suma: u32 = 0;


    let numero_ingresado: u32 = number;

    let mut numeros_multiplos: u32 = 0;
    let mut numeros_no_multiplos: u32 = 0;


    for numero in 1..=numero_ingresado{
            if numero % 3 == 0 || numero % 5 == 0{
                result_suma+=numero;
                numeros_multiplos+=1;
            }else{
                 numeros_no_multiplos+=1;

            }

       

    }
         println!("La suma de los multiplos de 3 o 5 hasta {} es: {}", number, result_suma);
    println!("Cantidad de multiplos encontrados: {}", numeros_multiplos);
    println!("Cantidad de numeros que no son multiplos de 3 ni de 5: {}", numeros_no_multiplos);

}


pub fn suma_pares_fibonacci_hasta_4milliones(limite: u64) -> Vec<u64>{

    // En primera instanciase debe entender que fibonacci es la suma de los dos numeros anteriores al 
    // actual y pasado matematicamente podria decirse que es esta formula.
    // F(N)= F(N-1) + F(N-2)

    let mut secuencia: Vec<u64>= vec![0,1];

    let mut _fibonnacci_pares : Vec<u64> = Vec::new();

    loop {

        let len = secuencia.len();
        let siguiente = secuencia[len-1] + secuencia[len-2];


        if siguiente > limite {
            break;
        }

        secuencia.push(siguiente);

        if siguiente % 2 == 0 {
            _fibonnacci_pares.push(siguiente);
        }
    }

    

    return _fibonnacci_pares;
}



// Problem3
pub fn calcular_numero_primo_mayor (mut n: u64) -> Vec<u64>{

let mut factores = Vec::new();
let mut divisor = 2;

// Un punto importante aca es saber que un numero puede ser descompuestro en la multipli
// cacion de numeros primos. 
// De la siguiente manera, dividis el numero / 2 y si da 0 se continua sobre 2 dividiendo
//  y cuando da distintoa 0 ahi se avanza a 3 y asi susecivamente hasta que el modulo da distinto de 0


// Si n es menor que 1 no sigue 
while n > 1 {
    while n % divisor == 0 {
        
        factores.push(divisor);
        n/=divisor;

       

    }
     if divisor == 2{
            divisor = 3;

        }else {
            divisor += 2;
        }    



}



factores
}

// Chequea si un numero es palíndromo usando aritmetica
fn es_palindromo(mut n: u32) -> bool {
    let original = n;
    let mut rev = 0;

    while n > 0 {
        rev = rev * 10 + (n % 10); // agrego el ultimo dugito de n al final de rev
        n /= 10;                   // saco el ultimo dígito de n
    }

    original == rev
}

// Busca el mayor palindromo como producto de dos números de 3 digitos.
// Devuelve (a, b, producto, cantidad_de_chequeos).
pub fn palindromo_mayor_3_digitos_trazado() -> (u32, u32, u32, u64) {
    let mut mejor_prod: u32 = 0;
    let mut mejor_a: u32 = 0;
    let mut mejor_b: u32 = 0;
    let mut chequeos: u64 = 0;

    // Recorremos de 999 hacia 100 (valores grandes primero)
    for a in (100..=999).rev() {
        // Poda por fila: si ni el mejor caso (a * 999) supera al record, terminamos
        let tope_fila = a * 999;
        if tope_fila <= mejor_prod {
            println!("No se puede superar el record actual ({}). Corto en a = {}.", mejor_prod, a);
            break;
        }

        // Por simetria, b ≤ a (evita duplicar trabajo)
        for b in (100..=a).rev() {
            let prod = a * b;
            chequeos += 1;

            // Como b baja, prod tambien baja.
            // Si ya no supera al record, corto esta fila no hace falta imprimir\
            if prod <= mejor_prod {
                break;
            }

            // Si es palindromo y mejora el record, guardamos y mostramos
            if es_palindromo(prod) {
                mejor_prod = prod;
                mejor_a = a;
                mejor_b = b;
                println!("Nuevo palindromo mayor: {} = {} × {}", mejor_prod, mejor_a, mejor_b);
            }
        }
    }

    // Resumen final
    println!(
        "Mayor palindromo: {} = {} × {} (chequeos: {})",
        mejor_prod, mejor_a, mejor_b, chequeos
    );

    (mejor_a, mejor_b, mejor_prod, chequeos)
}
