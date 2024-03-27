fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_de_caracteres = [false; 128]; // Assumindo caracteres ASCII
    
    for &c in input.as_bytes() {
        let indice = c as usize;
        if conjunto_de_caracteres[indice] {
            return false; 
        }
        conjunto_de_caracteres[indice] = true;
    }
    
    true 
}

fn main() {
    let string_de_teste = "cateto";
    
    if tem_caracteres_unicos(string_de_teste) {
        println!("A string possui todos os caracteres únicos.");
    } else {
        println!("A string não possui todos os caracteres únicos.");
    }
}


//  OU

// use std::io;

// fn tem_caracteres_unicos(input: &str) -> bool {
//     let first = &input[0..1];
//     for i in 0..input.len(){
//         if first == &input[i..i]{                   // [i..(i + 1)]: Isso é uma notação de fatia (slice) em Rust. Ele cria uma fatia que começa no índice i e vai até (i + 1) - 1, ou seja, apenas o caractere no índice i
//             println!("Naõ tem caracteres unicos"); //A razão pela qual usamos &input[i..(i + 1)] em vez de input[i] é devido à diferença nos tipos de dados retornados: input[i] retorna um caractere (char).
//             true;                                  //// &input[i..(i + 1)] retorna uma fatia de string (&str).
//         }else{                                    
//             println!("Tem caracteres unicos");
//             false;
//         }
//     }
//     true
// }

// fn main(){
//     let mut string = String::new();
//     io::stdin().read_line(&mut string).expect("Erro na leitura");

//     tem_caracteres_unicos(&string);

// }