use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

struct Book {
    filename: PathBuf,
}

impl Book {
    fn new(filename: PathBuf) -> Book {
        Book { filename }
    }

    fn read_file(&self, filename: &str, mut archive: zip::ZipArchive<BufReader<&File>>) -> String {
        let mut contents = String::new();
        let mut file = archive.by_name(filename).unwrap();
        file.read_to_string(&mut contents).unwrap();
        return contents;
    }

    fn read(&self) {
        println!("Reading {}", self.filename.display());

        let file = std::fs::File::open(&self.filename).unwrap();
        let reader = BufReader::new(&file);
        let mut archive = zip::ZipArchive::new(reader).unwrap();

        let files = archive.file_names();
        for file in files {
            println!("{}", file);
        }

        let filename = "META-INF/container.xml";
        // println!("{}", self.read_file(filename, archive));

        println!("{}", self.read_file("OPS/package.opf", archive));
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
