# 🎢 Montanha-Russa — Desafio Coddy

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Um programa interativo em Rust que determina se uma pessoa pode andar em uma montanha-russa com base em **idade**, **altura** e **acompanhamento de adulto**.

---

## 📋 Regras do Desafio

Para poder andar na montanha-russa, a pessoa deve atender **todas** as condições aplicáveis:

- ✅ Ter **pelo menos 12 anos** de idade.
- ✅ Ter **mais de 150 cm** de altura.
- ✅ Se tiver **menos de 15 anos**, deve estar **acompanhada de um adulto**.
- ✅ Se tiver **15 anos ou mais**, pode andar sozinha (desde que atenda aos requisitos de idade e altura).

---

## 🧠 Lógica de Decisão

| Condição | Mensagem de Saída |
|----------|-------------------|
| Idade < 12 | `Sorry, you're too young` |
| Altura ≤ 150 cm | `Sorry, you're not tall enough` |
| Idade < 15 e sem adulto | `Sorry, you need an adult with you` |
| Idade < 15 e com adulto | `You can ride with adult supervision!` |
| Idade ≥ 15 e altura > 150 cm | `You can ride by yourself!` |

---

## 🦀 Tecnologias

- **Rust** — linguagem principal
- `std::io` — para entrada e saída no terminal
- Estruturas condicionais (`if` / `else if` / `else`)
- Tipos primitivos: `i32` e `bool`

---

## 🚀 Como Executar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) instalado (versão 1.70+ recomendada)

### Passos

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/nome-do-repositorio.git

# Entre no diretório
cd nome-do-repositorio

# Execute o programa
cargo run