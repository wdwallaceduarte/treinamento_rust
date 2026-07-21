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

use std::io;

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
} 

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

