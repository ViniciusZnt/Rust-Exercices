fn eh_primo(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let limite = (num as f32).sqrt() as i32 + 1;
    for i in 2..limite {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numero = 13;
    let resultado = eh_primo(numero);
    println!("O número {} é primo? {}", numero, resultado);
}