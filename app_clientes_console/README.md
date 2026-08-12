# 📋 App Clientes Console

Aplicação de console desenvolvida em **Rust** com o objetivo de praticar os principais conceitos da linguagem através da implementação de um sistema simples de cadastro de clientes.

O projeto simula um pequeno sistema de gerenciamento de clientes utilizando um menu interativo no terminal, permitindo realizar operações básicas de cadastro, alteração, exclusão e listagem dos registros.

> **Projeto desenvolvido para fins de estudo e prática da linguagem Rust.**

---

## 🎯 Objetivos do projeto

Durante o desenvolvimento desta aplicação foram praticados diversos conceitos fundamentais do Rust, como:

* Organização do projeto em módulos (`mod`)
* Separação de responsabilidades
* Criação e utilização de `struct`
* Vetores (`Vec<T>`)
* Empréstimos (`&` e `&mut`)
* Ownership e Borrow Checker
* Funções e reutilização de código
* Entrada de dados pelo terminal
* Estruturas condicionais
* Estruturas de repetição (`loop`)
* Tratamento de `Option`
* Manipulação de coleções

---

## ✨ Funcionalidades

O sistema possui um menu interativo com as seguintes operações:

* ✅ Cadastrar cliente
* ✏️ Alterar cliente
* 🗑️ Excluir cliente
* 📄 Listar clientes cadastrados
* 🚪 Encerrar aplicação

Cada cliente possui os seguintes dados:

* ID
* Nome
* CPF
* Endereço

Os dados permanecem em memória durante a execução da aplicação.

---

## 🛠️ Tecnologias utilizadas

* Rust
* Cargo
* Biblioteca `clearscreen`

---

## 📂 Estrutura do projeto

```text
src/
│
├── main.rs
│
├── models/
│   ├── mod.rs
│   └── cliente.rs
│
└── tela/
    ├── mod.rs
    ├── menu.rs
    ├── ler.rs
    ├── operacoes_basicas.rs
    └── servico_cliente.rs
```

---

## ▶️ Como executar

Clone o repositório:

```bash
git clone https://github.com/wdwallaceduarte/app_clientes_console.git
```

Acesse a pasta do projeto:

```bash
cd app_clientes_console
```

Compile o projeto:

```bash
cargo build
```

Execute a aplicação:

```bash
cargo run
```

---

## 📚 Conceitos praticados

Este projeto foi desenvolvido com foco no aprendizado dos seguintes recursos da linguagem Rust:

* Criação de módulos
* Organização de projetos
* Structs
* Vetores
* Referências compartilhadas e mutáveis
* Funções
* Manipulação de Strings
* Entrada de dados
* Controle de fluxo
* Pattern Matching
* Option
* Boas práticas de separação de responsabilidades

---

## 👨‍💻 Autor

Desenvolvido por **Wallace Duarte** durante os estudos da linguagem Rust.

* GitHub: https://github.com/wdwallaceduarte
* LinkedIn: https://www.linkedin.com/in/wallace-duarte-9a241a29a
