use clap::{Parser, Subcommand};
use todo3::todo::{add, list};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// adds item to list
    Add {
        item_name: String,
    },
    
    /// checks item on list
    Check {
        item_name: String,
    },

    /// prints out todo list
    List {
    },

    /// removes item from list permanently
    Remove {
        item_name: String,
    },

    // Test {
    //     #[arg(short, long)]
    //     list: bool
    // },

}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::Add { item_name }) => {
            //println!("adding {item_name}");
            add(item_name.to_owned());
        }
        Some(Commands::Check { item_name }) => {
            println!("checking off {item_name}");
        }
        Some(Commands::List {}) => {
            list();
        }
        Some(Commands::Remove { item_name }) => {
            println!("removing {item_name}");
        }
        None => {}
        // Some(Commands::Test { list }) => {
        //     if *list {
        //         println!("Printing testing lists...");
        //     } else {
        //         println!("Not printing testing lists...");
        //     }
        // }
        // None => {}
        //Some(Commands::Add) => println!("add selected"),
        //Some(Commands::Remove) => println!("remove selected"),

    }
}
