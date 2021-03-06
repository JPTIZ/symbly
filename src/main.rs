#[macro_use]
extern crate pest_derive;

extern crate structopt;

use structopt::StructOpt;

mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Only parse source code, do not compile nor execute it.
    #[structopt(long = "parse-only")]
    parse_only: bool,
}

fn parse() {
    println!(">>> Parsing...");
    let tree = parser::parse(
        "r1 <- r2 + 3"
    ).expect("Error parsing source code");

    println!("Resulting tree: {:?}", tree);

    println!("Done!");
}

fn compile() {
    println!(">>> Compiling...");
    // TODO!
    println!("Done!");
}

fn execute() {
    println!(">>> Executing...");
    // TODO!
    println!("Done!");
}

fn main() {
    let opt = Opt::from_args();

    if opt.parse_only {
        parse();
    } else {
        parse();
        compile();
        execute();
    }
}
