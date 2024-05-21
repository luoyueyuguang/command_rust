use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config  = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //unwarp写法
    //run(config).unwrap_or_else(|err| {
    //    println!("Application error: {}", err);
    //    process::exit(1);
    //});

    //不利用正确时的返回值
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}

