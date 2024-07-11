pub mod book {

    use std::fmt;
    
    #[derive(PartialEq)]
    pub struct Book {
        pub title: String,
        pub author: String,
        pub year_published: u32,
        pub pages: u32,
    }
    
    impl Book {
        pub fn new(title: String, author: String,
            year_published: u32, pages: u32) -> Self {
            Self { title, author, year_published, pages }
        }
    }
    
    impl fmt::Display for Book {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // let len0 = 78;
            let len1: i32 = self.title.len().try_into().expect("Expected an int");
            let len2: i32 = (4 + self.author.len()).try_into().expect("Expected an int");
            let year_pub_float: f64 = self.year_published as f64;
            let len3: i32 = 15 + year_pub_float.log10().floor() as i32;
            let pages_float: f64 = self.pages as f64;
            let len4: i32 = 12 + pages_float.log10().floor() as i32;
            let len_vec = vec![&len1, &len2, &len3, &len4];
            // println!("{}", len_vec[0]);
            let len_max = len_vec.iter().max().expect("Expected an int");
            // println!("len1: {}", len1);
            // println!("len2: {}", len2);
            // println!("len3: {}", len3);
            // println!("len4: {}", len4);
            // println!("len_max: {}", len_max);
            for _ in 0..(**len_max + 4) {
                write!(f, "{}", String::from("-")).unwrap();
            }
            write!(f, "\n").unwrap();

            write!(f, "| {} ", self.title).unwrap();
            // let title_left = **len_max - len1;
            for _ in 0..(**len_max - len1) {
                write!(f, " ").unwrap();
            }
            write!(f, "|\n").unwrap();

            write!(f, "| By: {}", self.author).unwrap();
            for _ in 0..=**len_max - len2 {
                write!(f, " ").unwrap();
            }
            write!(f, "|\n").unwrap();

            write!(f, "| Published in: {}", self.year_published).unwrap();
            for _ in 0..=**len_max - len3 {
                write!(f, " ").unwrap();
            }
            write!(f, "|\n").unwrap();

            write!(f, "| {} pages long", self.pages).unwrap();
            for _ in 0..=**len_max - len4 {
                write!(f, " ").unwrap();
            }
            write!(f, "|\n").unwrap();

            for _ in 0..(**len_max + 4) {
                write!(f, "-").unwrap()
            }

            Ok(())
        }
    }
}

pub mod input {
    use clap::{Parser, Subcommand};
    use clap::{arg, command, Command};

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        Test {
            list: bool,
        }
    }
    pub fn command_prompt() -> String {
        let cmd = clap::command!()
            .propagate_version(true)
            // .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                clap::command!("add")
                    .about("adds book to book list"),
                    //.arg(arg!([NAME])),
            )
            .subcommand(
                clap::command!("list")
                    .about("prints the book list"),
            );
            let matches = cmd.get_matches();
            let matches = match matches.subcommand() {
                Some(("add", matches)) => {
                    matches;
                    // crate::librarian::BookList::add(&book_list);
                    String::from("add")
                }
                Some(("list", matches)) => {
                    matches;
                    String::from("list")
                }
                // todo
                Some(("help", matches)) => {
                    //&cmd.print_help();
                    String::from("help")
                
                }
                // // todo
                // Some(("h", matches)) => {

                // }
                Some(("quit", matches)) => std::process::exit(0),
                _ => unreachable!("Not add input!"),
            };
            matches
    }

}


pub mod librarian {
    use std::io;
    use crate::book::Book;

    pub struct BookList {
        pub list: Vec<Book>,
    }

    impl BookList {
        pub fn new() -> BookList {
            BookList {
                list: Vec::new()
            }
        }

        pub fn add(&mut self) {
            let title: String = get_book_content(0);
            let author: String = get_book_content(1);
            let year_pub: u32 = get_book_content(2).parse().expect("Could not parse into u32!");
            let pages: u32 = get_book_content(3).parse().expect("Could not parse into u32!");
            let book = Book::new(title.clone(), author, year_pub, pages);
            println!("{}", &book);
            if self.search(Some(title.clone())) == None {
                self.list.push(book);
            } else {
                eprintln!("{} already found in book list! Try adding \
                    something else", title);
            }
        }
        
        pub fn search(&self, title: Option<String>) -> Option<&Book> {
            let title = if title == None {get_book_content(0)} else {String::from(title.unwrap())};
            if self.list.len() > 0 {
                for book in &self.list {
                    // println!("{i}");
                    // println!("{}", book.title);
                    if title == book.title {
                        // println!("{}", &book);
                        return Some(book)
                    }
                }
            }
            None
        }


        pub fn remove(&self) -> Book {
            let title: String = get_book_content(0);
        }

        fn write(&self) {
            // let mut tmpfile: File = temp
            // for i in self.list {

            // }
        }

        pub fn list() {

        }

    }

    fn get_book_content(content_ind: u8) -> String {
        match content_ind {
            0 => println!("What is the name of the book?"),
            1 => println!("What is the name of the author?"),
            2 => println!("What year was the book published?"),
            3 => println!("How many pages are in the book?"),
            _ => eprintln!("Incorrect content_ind given!"),
        };
        // println!("What is the name of the book?");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Expected a string");
        let buf_trim = buffer.trim();
        // println!("{}", &buffer);
        String::from(buf_trim)
    }
}


// ----------------------
// | Harry Potter       |
// | By: J.K. Asshole   |
// | Published in: 1999 |
// | 500 pages long     |
// ----------------------
