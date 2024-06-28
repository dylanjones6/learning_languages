pub mod book {

    use std::fmt;

    pub struct Book {
        pub title: String,
        author: String,
        year_published: u32,
        pages: u32,
    }
    
    impl Book {
        pub fn new(title: String, author: String,
            year_published: u32, pages: u32) -> Self {
            Book { title, author, year_published, pages }
        }
    }
    
    impl fmt::Display for Book {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // let len0 = 78;
            let len1: i32 = self.title.len().try_into().unwrap();
            let len2: i32 = (4 + self.author.len()).try_into().unwrap();
            let year_pub_float: f64 = self.year_published as f64;
            let len3: i32 = 15 + year_pub_float.log10().floor() as i32;
            let pages_float: f64 = self.pages as f64;
            let len4: i32 = 12 + pages_float.log10().floor() as i32;
            let len_vec = vec![&len1, &len2, &len3, &len4];
            // println!("{}", len_vec[0]);
            let len_max = len_vec.iter().max().unwrap();
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
            for _ in 0..(**len_max - len2 + 1) {
                write!(f, " ").unwrap();
            }
            write!(f, "|\n").unwrap();

            write!(f, "| Published in: {}", self.year_published).unwrap();
            for _ in 0..(**len_max - len3 + 1) {
                write!(f, " ").unwrap();
            }
            write!(f, "|\n").unwrap();

            write!(f, "| {} pages long", self.pages).unwrap();
            for _ in 0..(**len_max - len4 + 1) {
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
    #[derive(Parser)]
    #[command(version, about, long_about = None)]

    struct Cli {
        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        Test {
            list: bool,
        }
    }
}


pub mod librarian {
    pub fn add() {
        
    }

    fn get_name() {

    }
}


// ----------------------
// | Harry Potter       |
// | By: J.K. Asshole   |
// | Published in: 1999 |
// | 500 pages long     |
// ----------------------
