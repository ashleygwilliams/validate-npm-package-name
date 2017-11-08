#[macro_use]
extern crate lazy_static;

//extern crate node_builtins;
extern crate regex;

use regex::Regex;
//use node_builtins::BUILTINS;

lazy_static! {
  //static ref scopedPackagePattern: Regex = Regex::new(r"^(?:@([^/]+?)[/])?([^/]+?)$").unwrap();
  static ref SPECIAL_CHARACTERS_PATTERN: Regex = Regex::new(r"[~'!()*]").unwrap();
}

pub fn validate(pkg_name: &str) -> bool {
  if SPECIAL_CHARACTERS_PATTERN.is_match(pkg_name) {
    return false
  }
  true
}
