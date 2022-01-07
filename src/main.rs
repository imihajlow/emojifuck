mod bf;

use argparse::ArgumentParser;
use argparse::StoreTrue;

use argparse::Store;
use std::fs;
use bf::BfMachine;
#[macro_use]
extern crate lazy_static;

fn main() {
    let mut fname = String::new();
    let mut print_classic = false;
    let mut print_hands = false;
    let mut print_emoji = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("An emoji-based Brainfuck interpreter");
        ap.refer(&mut print_classic).add_option(
            &["--print-classic"],
            StoreTrue,
            "Print loaded program in classic dialect",
        );
        ap.refer(&mut print_hands).add_option(
            &["--print-hands"],
            StoreTrue,
            "Print loaded program in hands dialect",
        );
        ap.refer(&mut print_emoji).add_option(
            &["--print-emoji"],
            StoreTrue,
            "Print loaded program in random emoji dialect",
        );
        ap.refer(&mut fname)
            .required()
            .add_argument("file", Store, "Program file");
        ap.parse_args_or_exit();
    }

    let text = match fs::read_to_string(&fname) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening {}: {}", fname, e);
            std::process::exit(1);
        }
    };
    let mut machine = BfMachine::new(&text);
    if print_classic {
        println!("{}", machine.get_program_string_classic());
    }
    if print_hands {
        println!("{}", machine.get_program_string_emoji_hands());
    }
    if print_emoji {
        println!("{}", machine.get_program_string_emoji_random());
    }
    if !print_emoji && !print_classic && !print_hands {
        match machine.run(&mut std::io::stdin(), &mut std::io::stdout()) {
            Ok(_) => (),
            Err(e) => println!("Error executing {}: {}", fname, e)
        }
    }
}
