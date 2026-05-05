// the idiomatic way to do this is to represent the argument in struct

struct Config {
    pattern: String,
    path: String,
    case_sensitive: bool,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let pattern = if let Some(string) = args.get(1) {
            string.clone()
        } else {
            return Err("error getting the pattern");
        };
        let path = if let Some(path) = args.get(2) {
            path.clone()
        } else {
            return Err("error getting the pattern");
        };
        let case_sensitive = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            pattern,
            path,
            case_sensitive,
        })
    }
}
