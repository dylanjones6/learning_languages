use library_sim::book::Book;
use library_sim::input::Cli;
use library_sim::librarian;
use clap::{arg, command, Command};
use library_sim::input::command_prompt;

fn main() {
    // let test = create_book(String::from("Title"), String::from("John Doe"), 2000, 150);
    // println!("{}!", test.author)
    //let mut book_arr: [book; 20];
    // let harry = Book::new("Harry Potter".to_string(),
    //                       "J.K. Asshole".to_string(), 1999, 400);
    // let mut book_vec: Vec<Book> = Vec::new();
    // book_vec.push(harry);
    // for i in &book_vec {
    //     println!("{}", i.title);
    // }
    // let hunger_games = Book::new("The Hunger Games".to_string(),
    // "Suzanne Collins".to_string(), 2008, 300);
    // println!("{hunger_games}");
    // &book_vec.iter().position()
    

    //librarian::add();

    // let book_list: Vec<&Book> = Vec::new();
    //let book_list: librarian::BookList = V
    let mut book_list = librarian::BookList::new();
    
    println!("What command would you like to run?");
    
    book_list.list.push( Book {title: "harry potter".to_string(), author: "jk asshole".to_string(), year_published: 2000, pages: 450});
    book_list.list.push( Book {title: "the hunger games".to_string(), author: "suzanne collins".to_string(), year_published: 2010, pages: 400});
    //crate::librarian::BookList::add(&mut book_list);
    //crate::librarian::
    crate::librarian::BookList::search(&book_list, None);
    command_prompt();
}
