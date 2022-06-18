use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn app_config(file_name: &str, level: Option<&str>) -> Config {
    let level = match level {
        Some(l) => convert_to_level(l),
        None => Ok(LevelFilter::Info),
    };
    if level.is_err() {
        println!(
            "{}, falling back to default",
            level.err().unwrap_or("no value for log level")
        )
    }
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build(file_name)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("file")
                .build(level.unwrap_or(LevelFilter::Info)),
        )
        .unwrap();

    config
}

fn convert_to_level(level: &str) -> Result<LevelFilter, &str> {
    match &*level.to_lowercase() {
        "info" => Ok(LevelFilter::Info),
        "error" => Ok(LevelFilter::Error),
        "debug" => Ok(LevelFilter::Debug),
        "warn" => Ok(LevelFilter::Warn),
        _ => Err("failed to convert value into Level"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::fs::{remove_file, File};
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn test_logger_config_creation() {
        let file_name = "test.file";
        log4rs::init_config(app_config(file_name, Some("info"))).unwrap();
        log::info!("test logging");

        let path = Path::new(file_name);
        let display = path.display();
        assert!(path.exists());
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {
                print!("{} contains:\n{}", display, s);
                assert!(s.contains("test logging"));
                let reg_match = Regex::new("\\d{4}-\\d{2}-\\d{1,2}T\\d{1,2}:\\d{1,2}:\\d{1,2}\\.\\d+\\+\\d{2}:\\d{2} - INFO - test logging").unwrap();
                assert!(reg_match.is_match(&s))
            }
        }
        remove_file(path).expect(&*format!("unable to remove file: {}", file_name));
    }

    #[test]
    fn test_use_wrong_level() {
        let file_name = "test.file";
        log4rs::init_config(app_config(file_name, Some("error"))).unwrap();
        log::info!("test logging");

        let path = Path::new(file_name);

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };
        let mut s = String::new();
        file.read_to_string(&mut s).expect("couldn't read file");
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_convert_to_level() {
        let levels = [
            "info", "INFO", "ERROR", "error", "debug", "DEBUG", "warn", "WARN",
        ];
        let result = levels
            .iter()
            .map(|l| convert_to_level(l))
            .filter(|x| x.is_err())
            .count();
        assert_eq!(result, 0);
        let failure = convert_to_level("fail");
        assert!(failure.is_err());
        assert_eq!(failure.err().unwrap(), "failed to convert value into Level")
    }
}
