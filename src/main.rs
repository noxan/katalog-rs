use std::path::PathBuf;

struct Library {
    name: String,
    path: PathBuf,
}

impl Library {
    fn new(name: String, path: PathBuf) -> Library {
        Library { name, path }
    }

    fn init(&self) {
        if !self.path.exists() {
            std::fs::create_dir(&self.path).unwrap();
        }

        let files = std::fs::read_dir(&self.path).unwrap();
        for file in files {
            let path = file.unwrap().path();
            println!("{}", path.display());
        }
    }
}

fn main() {
    let root = home::home_dir().unwrap().join("Books");
    let library = Library::new(String::from("Default"), root);

    library.init();

    println!("Library '{}' at {}", library.name, library.path.display());
}
