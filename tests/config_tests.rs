// Test TOML parsing of the default config
#[test]
fn test_parse_default_config() {
    let toml_str = include_str!("../config/default.toml");
    let config: toml::Value = toml::from_str(toml_str).unwrap();

    assert_eq!(config["general"]["enabled"].as_bool(), Some(true));
    assert_eq!(config["general"]["mode"].as_str(), Some("auto"));
    // Default config has empty replacements (built-ins added at runtime)
    assert!(config.get("replacements").is_some());
    assert!(config["exclusions"]["apps"].as_array().is_some());
}

#[test]
fn test_parse_minimal_config() {
    let toml_str = r#"
[general]
enabled = false
mode = "listen"

[replacements]

[exclusions]
apps = []
"#;
    let config: toml::Value = toml::from_str(toml_str).unwrap();
    assert_eq!(config["general"]["enabled"].as_bool(), Some(false));
}
