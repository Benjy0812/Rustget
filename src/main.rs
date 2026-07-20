fn main() {
    println!("Welcome to Rustget!");
    println!("Chose (1, 2, 3, 4)");
}

enum Commands {
    Isntall(String),
    Remove(String),
    Search(String),
    List,
}

fn install() {}

fn listInstalled() {}

fn uninstall() {}

fn search() {}
