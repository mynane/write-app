///! Some config file template

/// template for clash core `config.yaml`
pub const CLASH_CONFIG: &[u8] = br#"# Default Config For Clash Core

"#;

/// template for `profiles.yaml`
pub const PROFILES_CONFIG: &[u8] = b"# Profiles Config for Clash Verge

current: ~
items: ~
";

/// template for `app.yaml`
pub const APP_CONFIG: &[u8] = b"# Defaulf Config For APP


";

/// template for new a profile item
pub const ITEM_LOCAL: &str = "# Profile Template for clash verge

proxies:

proxy-groups:

rules:
";

/// enhanced profile
pub const ITEM_MERGE: &str = "# Merge Template for clash verge
# The `Merge` format used to enhance profile


";

/// enhanced profile
pub const ITEM_SCRIPT: &str = "// Should define the `main` function
// The argument to this function is the clash config
// or the result of the previous handler
// so you should return the config after processing
function main(params) {
  return params;
}
";
