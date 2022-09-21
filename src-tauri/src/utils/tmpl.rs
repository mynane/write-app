///! Some config file template

/// template for clash core `config.yaml`
pub const CONFIG_YAML: &[u8] = br#"# Default Config 

spctl_master_disable: false
theme: light
lang: ~
jira: ~
"#;

/// template for `profiles.yaml`
pub const PROFILES_CONFIG: &[u8] = b"# Profiles Config for Clash Verge

current: ~
items: ~
";

/// template for `app.yaml`
pub const APP_CONFIG: &[u8] = b"# Defaulf Config For APP


";

/// template for `profiles.yaml`
pub const REPOSITORY_CONFIG: &[u8] = b"# PDefault Config for Repository

basic_dir: ~
items: ~
";
