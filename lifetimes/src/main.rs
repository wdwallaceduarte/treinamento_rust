/*
    Em Rust, "Lifetimes" (tempo de vida) são uma forma de o compilador garantir que referências não persistam além da existência dos dados aos quais elas apontam, prevenido assim erros comuns como dangling pointers (ponteiros para dados já desalocados).

    Borrow checker = "checagem de empréstimos" foi projetado para prevenir erros.
*/

/*fn retorna_mensagem_de_teste() -> &String {
     let local = String::from("isso é um teste");
    &local //Tentativa de retornar uma referencia para uma string local
}
fn main() {
    let result = retorna_mensagem_de_teste();
    println!("lifetime error {}", result);
} */

// =============== Solução ===============

/*
    // 'a = uma anotação de lifetime (tempo de vida)
    São usados para garantir que referências a dados não outlive (sobrevivam mais que) os dados aos quais elas apontam.

    Eles são uma parte fundamental do sistema de tipos de Rust, permitindo que o compilador verifique em tempo de compilação que os dados referenciais não serão desalocados enquanto ainda existirem referências a eles, evitando assim dangling references (referências penduradas) e garantindo segurança de memórias.
 */

fn quem_e_maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    println!("Endereço de memória x: {:?}", x.as_ptr());
    println!("Endereço de memória y: {:?}", y.as_ptr());

    if x.chars().count() > y.chars().count() {
        x // retornando o ganhador
    } else {
        y // retornando o ganhador    
    }
}

fn main() {
    let string1 = String::from("abcd");
    println!("Endereço de memoria de string 1: {:?}", string1.as_ptr());

    let string2 = "xyz";
    println!("Endereço de memoria de string 2: {:?}", string2.as_ptr());

    let result = quem_e_maior(&string1.as_str(), string2);
    println!("A maior string é: {}", result);
    println!("Endereço de memoria de result o ganhador: {:?}", result.as_ptr());
}


