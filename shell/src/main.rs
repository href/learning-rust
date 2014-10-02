use std::io;

#[deriving(PartialEq)]
enum Action {
    Quit,
    Print,
    Random,
    Nothing
}

mod utils {
    use std::rand;

    pub fn random_number(max: uint) -> uint {
        (rand::random::<uint>() % max) + 1u
    }
}

static shell_prefix: &'static str = "> ";
static result_prefix: &'static str = "=> ";

fn parse_input(input: &str) -> Action {  
    match input {
        "random" => Random,
          "quit" => Quit,
             "q" => Quit,
              "" => Nothing,
               _ => Print
    }
}

fn print_random() {
    println!("{}{}", result_prefix, utils::random_number(1000u));
}

fn main() {
    println!("Rust shell");

    loop {
        io::print(shell_prefix);

        let raw = io::stdin().read_line().ok().expect("Failed to read line");
        let normalized = raw.as_slice().trim();

        match parse_input(normalized) {
            Quit => break,
            Print => println!("{} {}", result_prefix, normalized),
            Random => print_random(),
            Nothing => continue
        }
    }
}