use monkey_interpreter::repl;

fn main() {
    println!("This is the Monky programming language!");
    repl::start().unwrap();
}
