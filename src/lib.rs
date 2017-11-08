#[macro_use]
extern crate lazy_static;

//extern crate node_builtins;
extern crate regex;

use regex::Regex;
//use node_builtins::BUILTINS;

lazy_static! {
  //static ref scopedPackagePattern: Regex = Regex::new(r"^(?:@([^/]+?)[/])?([^/]+?)$").unwrap();
  static ref SPECIAL_CHARACTERS_PATTERN: Regex = Regex::new(r"[~'!)(\\*]").unwrap();
  static ref START_WITH_PERIOD_PATTERN: Regex = Regex::new(r"^\.").unwrap();
  static ref START_WITH_UNDERSCORE_PATTERN: Regex = Regex::new(r"^_").unwrap();
}

pub fn validate(pkg_name: &str) -> bool {
  if SPECIAL_CHARACTERS_PATTERN.is_match(pkg_name) {
    return false
  } else if START_WITH_PERIOD_PATTERN.is_match(pkg_name) {
    return false
  } else if START_WITH_UNDERSCORE_PATTERN.is_match(pkg_name) {
    return false
  } else if pkg_name.to_string().trim() != pkg_name.to_string() {
    return false
  }
  true
}
