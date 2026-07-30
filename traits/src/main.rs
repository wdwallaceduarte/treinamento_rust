trait Pessoa {
    fn mostra(&self);
}

struct PessoaFisica {
    id: i32,
    nome: String,
    cpf: String,
}

impl Pessoa for PessoaFisica {
    fn mostra(&self) {
        println!(
            "\
            ID: {}\n\
            Nome: {}\n\
            CPF: {}\n\
        ",
            self.id, 
            self.nome, 
            self.cpf,
        )
    }
}
struct PessoaJuridica {
    id: i32,
    nome: String,
    cnpj: String,
}

impl Pessoa for PessoaJuridica {
        fn mostra(&self) {
            println!(
            "\
            ID: {}\n\
            Nome: {}\n\
            CPF: {}\n\
        ",
            self.id, 
            self.nome, 
            self.cnpj,
        )
        }
}

fn exibe_documento(pessoa: &dyn Pessoa) {
    pessoa.mostra();
}
fn main() {
    let pf = PessoaFisica { id: 1, nome: "João".to_string(), cpf: "123.456.789-00".to_string()};
    let pj = PessoaJuridica { id: 2, nome: "Wallace".to_string(), cnpj: "88.888.888/888-88".to_string()};

    // Exibindo os documentos
    exibe_documento(&pf);
    exibe_documento(&pj);
}
