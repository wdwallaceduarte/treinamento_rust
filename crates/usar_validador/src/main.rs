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
    let validado: bool = vd::validadores::cpf(cpf.as_str());
    // let validado: bool = vd::validadores::cnpj(cnpj.as_str()); //Ou CNPJ

    if validado {
        println!("O CPF é válido!")
    } else {
        println!("O CPF é inválido!")
    }
}
