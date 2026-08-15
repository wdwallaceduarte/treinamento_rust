/*
    DESAFIO CODDY!
    Crie um programa que verifica se alguém pode andar em uma montanha-russa. Os requisitos são:

    Deve ter pelo menos 12 anos
    Deve ter mais de 150cm de altura
    Se atenderem a ambos os requisitos, mas tiverem menos de 15 anos, precisam de supervisão de um adulto

    Imprima exatamente estas mensagens para cada caso:

    Se for muito jovem: Sorry, you're too young
    Se não for alto o suficiente: Sorry, you're not tall enough
    Se tiver menos de 15 anos e sem adulto: Sorry, you need an adult with you
    Se tiver menos de 15 anos com adulto: You can ride with adult supervision!
    Se tiver 15 anos ou mais e for alto o suficiente: You can ride by yourself!

*/
use std::io;

fn main() {
    let mut age_input = String::new();
    let mut height_input = String::new();
    let mut adult_input = String::new();

    println!("\nDigite sua idade:");
    io::stdin().read_line(&mut age_input).unwrap();
    println!("Digite sua altura em centímetros:");
    io::stdin().read_line(&mut height_input).unwrap();
    println!("Está acompanhado de um adulto? (true/false)");
    io::stdin().read_line(&mut adult_input).unwrap();

    let age: i32 = age_input.trim().parse().unwrap();
    let height: i32 = height_input.trim().parse().unwrap();
    let has_adult: bool = adult_input.trim().parse().unwrap();

    // Escreva seu código abaixo
    if age < 12 {
        println!("\n> Desculpe, você é muito jovem")
    } else if height < 150 {
        println!("\n> Desculpe você não é alto suficiente")
    } else if age < 15 && has_adult == false {
        println!("\n> Desculpe, você precisa de um adulto com você")
    } else if age < 15 && has_adult == true {
        println!("\n> Voce pode andar com o adulto supervisor")
    } else {
        print!("\nVocê pode andar sozinho")
    }
}
