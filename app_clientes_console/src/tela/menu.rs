use crate::models::cliente::Cliente;
use crate::tela::ler::*;
use crate::tela::operacoes_basicas::*;
use crate::tela::servico_cliente::*;



pub fn mostrar_menu(clientes: &mut Vec<Cliente>) { 
    limpar_tela();   
    loop {
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
        match opcao {            
            1 => incluir_cliente(clientes),
            2 => alterar_cliente(clientes),
            3 => excluir_cliente(clientes),
            4 => listar_cliente(clientes),
            0 => {
                println!("Finalizando...");
                return;
            },
            _ => println!("Opção inválida!")
        }

        // println!("Digite enter para continuar...");
        // ler_dados();
    }
}