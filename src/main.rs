use minigrep::Config;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem: {}", e);
        process::exit(1);
    });

    println!("config query: {:}", config.query);
    println!("config filename: {:}", config.filename);
    println!("");

    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem: {}", e);
        process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use minigrep::Config;
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
