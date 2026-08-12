use std::io;
use validador as vd;

fn main() {
    println!("Digite o CPF");
    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("Você digitou: {}", cpf.trim());
        },
        Err(e) => {
            println!("Erro ao ler entrada: {}", e);
        }
    }
    let validado: bool = vd::validar_cpf(cpf.as_str());

    if validado {
        println!("O CPF é válido!")
    } else {
        println!("O CPF é inválido!")
    }
}
