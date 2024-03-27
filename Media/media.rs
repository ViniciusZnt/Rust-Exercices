fn calcular_media(notas: &[f64]) -> f64 {

    let mut soma: f64 = 0.0;

    for i in 0..notas.len() {
        soma += notas[i]; 
    }

    let media = soma / notas.len() as f64;

    media

}

fn main() {

    let notas = vec![7.5, 8.0, 6.5, 9.0, 7.0];

    let media = calcular_media(&notas);

    println!("A média das notas é: {}", media);

}