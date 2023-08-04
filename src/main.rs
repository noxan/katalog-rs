fn main() {
    let home = home::home_dir().unwrap();
    let root = home.join("Books");

    println!("Directory: {}", root.display());

    println!("Hello, world!");
}
