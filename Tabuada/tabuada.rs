use std::io;


fn tabuada(num: &mut i32) -> (){
    for i in 0..10{                                          //Caso estranho de mutabilidade
                                      //------No caso da variável resultado na função tabuada, não estamos reatribuindo um novo valor a ela em cada iteração do loop.
                                      // Em vez disso, estamos calculando um novo valor (*num * i) e imprimindo-o diretamente na chamada println!(). Portanto, não é necessário declarar
                                      // a variável resultado como mutável, pois não estamos alterando o valor da própria variável, apenas utilizando-a para armazenar temporariamente o
        let resultado = *num * i;// resultado do cálculo em cada iteração do loop.
        print!("O {} da tabudada eh {}\n",i,resultado);
    }
}
fn main(){
    print!("Digite um numero até 10 para imprimir a tabuada\n");
    let mut raw:String = String::new();

    io::stdin().read_line(&mut raw)
        .expect("Falha na leitura\n");

    let mut number: i32 = match raw.trim()
    .parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida\n");
            return ();
        }
    };

    tabuada(&mut number);
}