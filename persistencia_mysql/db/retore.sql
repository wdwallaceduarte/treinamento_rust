CREATE DATABASE IF NOT EXISTS clientes_rust-db;
use clientes_rust_db;

CREATE TABLE IF NOT EXISTS clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(255),
    telefone VARCHAR(20)
)

INSERT INTO clientes (nome, telefone) VALUES
('Wallace', '85 98723-6538'),
('Daniele', '85 98756-3268'),
('Danilo', '85 98756-3268');