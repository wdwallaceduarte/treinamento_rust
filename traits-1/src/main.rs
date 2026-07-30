/* trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("Miaow!")
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    dog.speak();
    cat.speak();
} */


trait Display {
    fn display(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn mostrar(&self) -> String {
        format!("Point (x: {}, y: {})", self.x, self.y)
    }
}

impl Display for Point {
    fn display(&self) -> String {
        format!("Point (x: {}, y: {})", self.x, self.y)
    }
}

fn print_display(item: &impl Display) {
    println!("{}", item.display())
}

fn main() {
    let point = Point {x:5, y: 10};

    point.mostrar();

    print_display(&point);
}
