use std::env;
use std::fs;
use std::process;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error found: {}", err);
        process::exit(1)
    });

    println!("config: {:?}", config);
    run(config);
    Ok(())
}

fn run(config: Config) {
    let content =
        fs::read_to_string(&config.filename).expect("Something wrong with reading the file");
    println!("content {}:\n{}", &config.filename, content);
}

#[derive(Debug)]
struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            Err("usage: minigrep file regex")
        } else {
            Ok(Config {
                query: args[1].to_string(),
                filename: args[2].to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn test_valid_config() -> Result<(), &'static str> {
        let args = &vec![
            "minigrep".to_string(),
            "filename".to_string(),
            "query".to_string(),
        ];
        Config::new(&args).map(|_| ())
    }

    #[test]
    fn test_invalid_config() -> Result<(), &'static str> {
        let args = &vec!["minigrep".to_string(), "filename".to_string()];
        let config = Config::new(&args);
        match config {
            Ok(_) => Err("shouldn't happen!"),
            Err(_) => Ok(()),
        }
    }
}
