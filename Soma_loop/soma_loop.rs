fn main() {

   

    let mut numeros = Vec::new();



    println!("Digite uma sequência de números reais (digite 'fim' para encerrar):");

    loop {

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");

       


        if entrada.trim() == "fim" {

            break;

        }


       

        let numero: f64 = match entrada.trim().parse() {

            Ok(num) => num,

            Err(_) => {

                println!("Entrada inválida, por favor digite um número real válido.");

                continue;

            }

        };

        numeros.push(numero);

    }


    // Calcular a soma dos números pares

    let mut soma = 0.0;

    for num in numeros.iter() {

        if num % 2.0 == 0.0 {

            soma += num;

        }

    }


    // Imprimir a soma dos números pares

    println!("A soma dos números pares é: {}", soma);

}