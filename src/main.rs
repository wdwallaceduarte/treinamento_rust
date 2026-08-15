/* Desafio 1
Você foi contratado como desenvolvedor júnior para um banco digital inovador chamado ByteBank. Sua primeira tarefa é criar uma funcionalidade simples, porém essencial: o sistema de verificação de saldo. Os clientes do ByteBank frequentemente consultam seus saldos para decidir se podem realizar uma compra. Para tornar o processo mais eficiente, o banco deseja um programa que, ao receber o saldo atual da conta e o valor de uma compra, informe se a transação pode ser realizada ou não. Sua solução será integrada ao aplicativo do banco, ajudando milhares de usuários a tomar decisões rápidas e seguras.

Implemente um programa que receba dois números inteiros positivos: o saldo disponível na conta e o valor da compra desejada. O programa deve verificar se o saldo é suficiente para cobrir a compra. Caso seja, exiba a mensagem "Compra aprovada". Caso contrário, exiba "Saldo insuficiente". Considere que não há taxas ou descontos, e que o valor da compra nunca será negativo. O saldo pode ser zero.

Entrada
Dois números inteiros positivos separados por espaço, representando respectivamente o saldo disponível e o valor da compra.

Saída
Uma string indicando o resultado da verificação: "Compra aprovada" se o saldo for suficiente, ou "Saldo insuficiente" caso contrário.

Exemplos
A tabela abaixo apresenta exemplos de entrada e saída:

Entrada | Saída
--------+---------------
100 50  | Compra aprovada
 30 40  | Saldo insuficiente
  0  0  | Compra aprovada
 75 75  | Compra aprovada */
// DESAFIO 1

/* use std::io;

fn main() {
    // Lê a linha de entrada do usuário (saldo e valor da compra)
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");

    // Converte a entrada em dois inteiros positivos
    let valores: Vec<u32> = entrada
        .split_whitespace()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let saldo = valores[0];
    let compra = valores[1];

    if saldo >= compra {
        println!("Compra aprovada");
    } else {
        println!("Saldo insuficiente");
    }
} */

/* //  DESAFIO 2

use std::io;

fn main() {
    // Lê uma linha da entrada padrão
    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.len() == 2 {
            let nome = parts[0];
            let tipo = parts[1];

            println!("Welcome, {}! Your account type is {}.", nome, tipo);
        } else {
            println!("Invalid input.");
        }
    }
} */

/* fn main() {
    // Inicialize as variáveis
    // Substitua os valores de placeholder abaixo pelos da tarefa
    let is_sunny: bool = true;
    let wind_speed: f64 = 9.0;
    let temperature: i32 = 22;
    let solar_panel_output: i32 = 14;
    let is_cloudy: bool = true;

    // A expressão lógica completa
    // Substitua o placeholder abaixo pela condição completa
    let result: bool = true;

    // Não apague as linhas abaixo
    println!("Checking conditions for solar energy production...");
    println!("1. Is it sunny? {}", is_sunny);
    println!("2. Is wind speed safe? {}", (wind_speed < 10.0));
    println!("3. Can panels produce more? {}", (solar_panel_output < 15));
    println!("4. Is temperature good OR no clouds? {}", (temperature > 20 || !is_cloudy));
    println!("\\nFinal result - Good day for solar energy production: {}", result);
} */
/*
use std::io;

fn main() {
    let mut n1_input = String::new();
    let mut n2_input = String::new();
    let mut op_input = String::new();

    println!("Digite um valor");
    io::stdin().read_line(&mut n1_input).unwrap();
    println!("Digite outro valor");
    io::stdin().read_line(&mut n2_input).unwrap();
    println!("Qual operador você deseja usar?");
    io::stdin().read_line(&mut op_input).unwrap();

    let n1: f64 = n1_input.trim().parse().unwrap();
    let n2: f64 = n2_input.trim().parse().unwrap();
    let op = op_input.trim();

    // Escreva seu código abaixo, use n1, n2 e op
    let mut result: f64 = 0.0;

    if op == "+" {
        result = n1 + n2
    }


    println!("{}", result);
}
 */

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

    println!("\nQual a idade?");
    io::stdin().read_line(&mut age_input).unwrap();
    println!("Qual a sua altura? *Somente numeros inteiros.");
    io::stdin().read_line(&mut height_input).unwrap();
    println!("Esta acompanhado de um adulto? *Entradas validas: true/false ");
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
