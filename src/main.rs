fn main() {
    let home = home::home_dir().unwrap();
    let root = home.join("Books");

    println!("Directory: {}", root.display());

    if !root.exists() {
        std::fs::create_dir(&root).unwrap();
    }

    println!("Hello, world!");
}
