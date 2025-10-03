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

