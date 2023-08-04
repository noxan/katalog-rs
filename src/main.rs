use std::path::PathBuf;

struct Book {
    filename: PathBuf,
}

impl Book {
    fn new(filename: PathBuf) -> Book {
        Book { filename }
    }
}

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
            let book = Book::new(path);
            println!("{:?}", book.filename);
        }
    }
}

fn main() {
    let root = home::home_dir().unwrap().join("Books");
    let library = Library::new(String::from("Default"), root);

    println!("Library '{}' at {}", library.name, library.path.display());
    library.init();
}
