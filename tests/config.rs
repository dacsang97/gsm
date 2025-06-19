use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

use gsm::config::{self, Config, ConfigError};

#[test]
fn parse_config_file() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("config.yaml");
    let mut file = File::create(&path).expect("create");
    let yaml = r#"
org: example
repositories:
  - repo1
  - repo2
env:
  KEY1: value1
  KEY2: value2
"#;
    file.write_all(yaml.as_bytes()).expect("write");
    drop(file); // close file

    let config: Config = config::load_config_from_file(&path).expect("load");
    assert_eq!(config.org, "example");
    assert_eq!(config.repositories, vec!["repo1", "repo2"]);
    assert_eq!(config.env.get("KEY1").unwrap(), "value1");
    assert_eq!(config.env.get("KEY2").unwrap(), "value2");
}

#[test]
fn file_not_found_returns_error() {
    let err = config::load_config_from_file("/tmp/does_not_exist.yaml").unwrap_err();
    matches!(err, ConfigError::FileReadError(_));
}

#[test]
fn invalid_yaml_returns_error() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("bad.yaml");
    let mut file = File::create(&path).expect("create");
    file.write_all(b"not: [valid").expect("write");
    drop(file);

    let err = config::load_config_from_file(&path).unwrap_err();
    matches!(err, ConfigError::YamlParseError(_));
}
