#
**OBS.:** Caso a leganda na video aula volte a aparecer na plataforma da DIO 
1. Basta abrir o console `F12` e executar o comando: ` document.querySelector("iframe").src += "&cc_load_policy=0" ` 
#
## Comandos para o Rust 
`cargo run` - Executa o bloco de codigo para teste;
`cargo run -p "nome-da-pasta'` - Executa o bloco de codigo para teste desntro da subpasta;

`cargo doc --open` - Vai construir localmente a documentação fornecida por todas as suas dependências e abrí-las no seu navegador;

`code . -r` - abre o diretorio do terminal na janela atual do vscode;

`cargo install cargo-watch` - Instala o modo cargo run watch (vai executar o programa toda vez que salvar o codigo.);

`cargo wathc -x run` - faz o cargo run e executa em modo watch;

`cargo watch -x build` - faz o build e executa em modo watch;

`rustfmt src/main.rs` - para formatar o código arquivo apontado ex. src/main.rs;

`cargo fmt` - usado para formatar todos os arquivos

## Cargo Workspaces

No arquivo `Cargo.toml` na raiz adcionar `[workspace]`

Ex:   
[workspace]   
members = [  
    "desafio-basico",  
    "ownership-e-borrowing",  
    "manipulacao-de-strings",  
    "gerenciamento-de-arquivos-mod/0-modulos-app",  
]  
1. Dentro da pasta principal, quando for começar um novo tópico ou desafio, crie o subprojeto executando: `~/Documents/Full-Stack/DIO/Bootcamp Santander/treinamento_rust` 
     
   `cargo new "nome-do-projeto`
2. Para rodar o projeto especifico no terminal, basata digitar:  
    `cargo run -p "nome-do-projeto"`  
Ou  
Entrar na pasta do projeto
`cd nome-do-projeto` e rodar o comando `cargo run`. Quando quiser sair da pasta `cd..`

## Padrões Simples
```bash
>  Entrada/informação  
=> Resultado  
✓  Sucesso  
✗  Erro  
↲  Enter
println!("> Digite sua idade:");

println!("=> Você tem {} anos", idade);

println!("✓ Idade válida");

println!("✗ Idade inválida");
    println!("Digite Enter ↲  para continuar...");
