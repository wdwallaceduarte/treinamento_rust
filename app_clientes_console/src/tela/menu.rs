use crate::tela::ler::*;
use crate::tela::operacoes_basicas::{esperar, limpar_tela};


pub fn mostrar_menu() {    
    loop {
        limpar_tela();
        println!("\n\
            ================ Menu ===============\n\
            Esolha uma das opções abaixo:\n\n\
            1 - Cadastrar cliente\n\
            2 - Alterar cliente\n\
            3 - Excluir cliente\n\
            4 - Listar cliente\n\
            0 - Sair\n
        ");

        let opcao = ler_dados_int();
        limpar_tela();
        match opcao {            
            1 => println!("Opção 1 - Você selecionou a Opção para Cadastrar cliente"),
            2 => println!("Opção 2 - Você selecionou a Opção para Alterar cliente"),
            3 => println!("Opção 3 - Você selecionou a Opção para Excluir cliente"),
            4 => println!("Opção 4 - Você selecionou a Opção para Listar cliente"),
            0 => {
                println!("Finalizando...");
                return;
            },
            _ => println!("Opção inválida!")
        }

        // println!("Digite enter para continuar...");
        // ler_dados();
        esperar(2);
    }
}