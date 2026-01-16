use std::env;
// mtkcli -i input.csv

struct Args {
    pwd: String,
    input: String,
}

impl Args {
    fn new() -> Args {
        Args { pwd: String::new(), input: String::new() }
    }
}

fn main() {
    let env_args_collection: Vec<String> = env::args().collect();
    //获取pwd
    let mut args = Args::new();
    args.pwd = env_args_collection[0].clone();

    let mut index = 1;
    while index < env_args_collection.len() {
        if env_args_collection[index] == "-i" && env_args_collection[index + 1] != "" {
            args.input = env_args_collection[index + 1].clone();
            break;
        }
        index += 1;
    }
    println!("pwd: {}, input: {}", args.pwd, args.input);
}
