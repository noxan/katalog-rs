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
    }
}

fn main() {
    let home = home::home_dir().unwrap();
    let library = Library::new(String::from("Default"), home.join("Books"));

    library.init();

    println!("Library '{}' at {}", library.name, library.path.display());
}
