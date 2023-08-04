use std::{io::BufReader, path::PathBuf};

struct Book {
    filename: PathBuf,
}

impl Book {
    fn new(filename: PathBuf) -> Book {
        Book { filename }
    }

    fn read(&self) {
        println!("Reading {}", self.filename.display());

        let file = std::fs::File::open(&self.filename).unwrap();
        let reader = BufReader::new(&file);
        let zip = zip::ZipArchive::new(reader).unwrap();

        let files = zip.file_names();
        for file in files {
            println!("{}", file);
        }
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
