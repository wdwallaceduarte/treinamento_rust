/* // ==== Função duplicada por objetivo ====


fn contar_posicoes_inteiros(array: &[i32]) -> usize {
    array.len()
}

fn contar_posicoes_floats(array: &[f64]) -> usize {
    array.len()
}
fn contar_posicoes_strings(array: &[&str]) -> usize {
    array.len()
}

fn main() {
    let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
    let array_floats = [1.1, 2.2, 3.3, 4.4, 5.5];
    let array_strings = ["um", "dois", "tres"];

    println!("Posições no array de inteiros: {}", contar_posicoes_inteiros(&array_inteiros));
    println!("Posições no array de flotas: {}", contar_posicoes_floats(&array_floats));
    println!("Posições no array de strings: {}", contar_posicoes_strings(&array_strings));
} */

/* // ==== Resolução do generics
fn contar_posicoes<T>(array: &[T]) -> usize {
    array.len()
}

fn main() {
    let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
    let array_floats = [1.1, 2.2, 3.3, 4.4, 5.5];
    let array_strings = ["um", "dois", "tres"];

    println!("Posições no array de inteiros: {}", contar_posicoes(&array_inteiros));
    println!("Posições no array de flotas: {}", contar_posicoes(&array_floats));
    println!("Posições no array de strings: {}", contar_posicoes(&array_strings));
} */

/* // ==== Função duplicada por objetivo ====
fn quantidade_digitos_inteiro(i: i32) -> usize {
    i.to_string().chars().count()
}

fn quantidade_digitos_float(f: f64) -> usize {
    f.to_string().chars().count()
}

fn quantidade_caracteres_strubg(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let int_val: i32 = 12345;
    let float_val: f64 = 123.45;
    let string_val: &str ="Olá Wallace";

    println!("Quantidade de dígitos no inteiros: {}", quantidade_digitos_inteiro(int_val));
    println!("Quantidade de dígitos no flotas: {}", quantidade_digitos_float(float_val));
    println!("Quantidade de caracteres na strings: {}", quantidade_caracteres_strubg(string_val));
} */

/* // ==== Resolução do generics ====
trait ContaCaracteres {
    fn conta_caracteres(&self) -> usize;
}

impl ContaCaracteres for i32 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for f64 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for String {
    fn conta_caracteres(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> ContaCaracteres for &'a str {
   fn conta_caracteres(&self) -> usize {
       self.chars().count()
   }
}

fn quandatidade_caracteres<T: ContaCaracteres>(valor: T) -> usize {
    valor.conta_caracteres()
}

fn main() {
    let int_val: i32 = 12345;
    let float_val: f64 = 123.45;
    let str_val: &str ="Olá Wallace";
    let string_val: String = "Olá Duarte".to_string();

    println!("Quantidade de caracteres no inteiros: {}", quandatidade_caracteres(int_val));
    println!("Quantidade de caracteres no flotas: {}", quandatidade_caracteres(float_val));
    println!("Quantidade de caracteres na str: {}", quandatidade_caracteres(str_val));
    println!("Quantidade de caracteres na strings: {}", quandatidade_caracteres(string_val));
} */

//==== Resolução do generics ====

/*
    O trait Dissplay da biblioteca padrão pode ser utilizado para converter os tipos em uma forma que possa ser representada como uma string.
    Uma vez que um tipo implemente Displau, ele pode ser convertido em String e, em seguida, podemos contar os caracteres.
*/

/* use std::fmt::Display; // Trait que tem uma função comum `to_string()`

fn quandatidade_caracteres<T: Display>(valor:T) -> usize {
    valor.to_string().chars().count()
}

fn main() {
    let int_val: i32 = 12345;
    let float_val: f64 = 123.45;
    let str_val: &str ="Olá Wallace";

    println!("Quantidade de caracteres no inteiros: {}", quandatidade_caracteres(int_val));
    println!("Quantidade de caracteres no flotas: {}", quandatidade_caracteres(float_val));
    println!("Quantidade de caracteres na str: {}", quandatidade_caracteres(str_val));
} */

/* struct Point<T> {
    x: T,
    y: T
}

fn main() {
    let int_point = Point { x: 5, y: 10};
    let float_point = Point { x: 1.0, y: 5.0};
    let string_point = Point { x: 1.0, y: 5.0};

} */

/* struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn retorna_valor_de_x(&Self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.retorna_valor_de_x())
} */

/* use std::fmt::Display;

fn print<T: Display>(item: T) {
    println!("{}", item)
}

fn main() {
    print(1);// Int
    print(String::from("hello")); // String
    print("hello"); // &str
    print(1.5); // &f64
} */

/* struct Pair<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pair<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
}

fn main() {
    let pair = Pair::new(5, 10.5);
    println!("{}", pair.x);

    let pair2 = Pair::new(5, "O valor de y");
    println!("{}", pair2.y)
} */

/* use std::cmp::PartialOrd;
use std::fmt::Debug;

fn compare_and_display<T, U>(a: T, b: U)
where
    T: PartialOrd + Debug,
    U: Into<T>,
{
    let b: T = b.into();
    if a > b {
        println!("{:?} is greater than {:?}", a, b);
    } else {
        println!("{:?} is not greater than {:?}", a, b)
    }
}

fn main() {
    compare_and_display(10, 5);
} */

/* trait DatabaseService {
    fn save_message(&self, message: &str);
    fn show_message(&self) -> String;
}

struct MySQLService;

impl DatabaseService for MySQLService {
    fn save_message(&self, message: &str) {
        println!("Saving '{}' to MySQL", message);
        // Aqui iria a lógica para salvar a mensagem no MySQL
    }

    fn show_message(&self) -> String {
        let message = "Message from MySQL";
        println!("Fetching message from MySQL: {}", message);
        // Aqui iria a lógica para buscar a mensagem do MySQL
        message.to_string()
    }
}

struct PostgreSQLService;

impl DatabaseService for PostgreSQLService {
    fn save_message(&self, message: &str) {
        println!("Saving '{}' to PostgreSQL", message);
        // Aqui iria a lógica para salvar a mensagem no MySQL
    }

    fn show_message(&self) -> String {
        let message = "Message from PostgreSQL";
        println!("Fetching message from PostgreSQL: {}", message);
        // Aqui iria a lógica para buscar a mensagem do MySQL
        message.to_string()
    }
}

struct GenericService<T: DatabaseService> {
    database_service: T,
}

impl <T: DatabaseService> GenericService<T> {
    fn newd(database_service: T) -> Self {
        GenericService { database_service }
    }

    fn save_message(&self, message: &str) {
        self.database_service.save_message(message);
    }

    fn show_message(&self) -> String {
        self.database_service.show_message()
    }
}

fn main() {
    let mysql_service = MySQLService;
    let postgres_service = PostgreSQLService;

    let mysql_generic_service = GenericService::newd(mysql_service);
    let postgres_generic_service = GenericService::newd(postgres_service);

    mysql_generic_service.save_message("Hello, World");
    let message_from_mysql = mysql_generic_service.show_message();
    println!("{}", message_from_mysql);

    postgres_generic_service.save_message("Hello, World");
    let message_from_postgres = postgres_generic_service.show_message();
    println!("{}", message_from_postgres);
} */

/* use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Serialize)]

struct Produto {
    id: u32,
    nome: String,
    preco: f64
}

#[derive(Serialize)]

struct Cliente {
    id: u32,
    nome: String,
    email: String
}

// Função genérica para imprimir propriedades e valores de uma struct
fn imprimir_propriedades<T: Serialize>(item: &T) {
    let json = to_string_pretty(item).unwrap_or_else( |_| "Falha na serializa".to_string());
    println!("{}", json)
}

fn main() {
    let produto = Produto {
        id: 1,
        nome: "Caneta bic preta".to_string(),
        preco: 1.50,
    };

    let cliente = Cliente {
        id: 101,
        nome: "Wallace".to_string(),
        email: "wd.wallaceduarte@gmail.com".to_string()
    };

    imprimir_propriedades(&produto);
    imprimir_propriedades(&cliente);
} */

use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Serialize)]

struct Produto {
    id: u32,
    nome: String,
    preco: f64
}

#[derive(Serialize)]

struct Cliente {
    id: u32,
    nome: String,
    email: String
}

// Função genérica para imprimir propriedades e valores de uma struct
fn imprimir_propriedades(item: &impl Serialize) {
// fn imprimir_propriedades(item: &dyn Serialize) { // nesse caso não consegue usar o "dynamic"
    let json = to_string_pretty(item).unwrap_or_else( |_| "Falha na serializa".to_string());
    println!("{}", json)
}

fn main() {
    let produto = Produto {
        id: 1,
        nome: "Caneta bic preta".to_string(),
        preco: 1.50,
    };

    let cliente = Cliente {
        id: 101,
        nome: "Wallace".to_string(),
        email: "wd.wallaceduarte@gmail.com".to_string()
    };

    imprimir_propriedades(&produto);
    imprimir_propriedades(&cliente);
}

/* 
==== Conclusão ====
Codigo 1 usa generics com trait bonds explicitamente, o que é útil para quando você quer clareza total sobre a genericidade e está preparado para lidar com a verbosidade.

Codigo 2 simplifica a assinatura da função usando impl Trait, tornando o código mais limpo e fácil de ler, mantendo a eficiência da monomorfização. Se fosse usado &dyn Serialize, introduzuria polimorfismo dinâmico com uma ligeira penalidade de desempenho, mas com benefícios de flexibilidade.
*/
