mod enums;
mod models;

use enums::*;
use models::Pessoa;

fn main() {
    let wallace = Pessoa::new(
        "Wallace", 
        "333.222.333-00", 
        Tipo::Fisica
    );

    wallace.show();

    println!("{}", "-".to_string().repeat(20)); // Imprimie uma linha divisória

    let c_e_c = Pessoa::new(
        "C&C", 
        "222.222.222-22", 
        Tipo::Juridica
    );

    c_e_c.show();

    println!("{}", "-".to_string().repeat(20)); // Imprimie uma linha divisória

    let sexo_f = Sexo::Feminino;
    let sexo_m = Sexo::Masculino;
    let sexo_o = Sexo::Outros;

    print!("{}", sexo_string(sexo_f));
    print!("{}", sexo_string(sexo_m));
    print!("{}", sexo_string(sexo_o));

}

fn sexo_string(sexo: Sexo) -> &'static str  {
    match sexo {
        Sexo::Feminino => "Feminino",
        Sexo::Masculino => "Masculino",
        Sexo::Outros => "Outros"
    }
}
