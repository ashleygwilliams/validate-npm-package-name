extern crate validate_npm_package_name;

use validate_npm_package_name::validate;

#[test]
fn valid_name() {
  assert_eq!(true, validate("some-package"), "some-package should be a valid package name");
  assert_eq!(true, validate("example.com"), "example.com should be a valid package name");
  assert_eq!(true, validate("under_score"), "under_score should be a valid package name");
  assert_eq!(true, validate("dot.js"), "dot.js should be a valid package name");
  assert_eq!(true, validate("123numeric"), "123numeric should be a valid package name");
}

#[test]
fn name_cannot_have_special_characters() {
  assert_eq!(false, validate("~tilde"), "package name should not contain special characters");
  assert_eq!(false, validate("back\\slash"), "package name should not contain special characters");
  assert_eq!(false, validate("t'ck"), "package name should not contain special characters");
  assert_eq!(false, validate("exclamation!"), "package name should not contain special characters");
  assert_eq!(false, validate("(openparens"), "package name should not contain special characters");
  assert_eq!(false, validate("closeparens)"), "package name should not contain special characters");
  assert_eq!(false, validate("asteri*sk"), "package name should not contain special characters");
}

#[test]
fn name_cannot_start_with_period_or_underscore() {
  assert_eq!(false, validate(".dotstart"), "package name should not start with a period or underscore");
  assert_eq!(false, validate("_underscorestart"), "package name should not start with a period or underscore");
}

#[test]
fn name_cannot_have_leading_or_trailing_space() {
  assert_eq!(false, validate(" leadingspace"), "package name should not have leading or trialing space");
  assert_eq!(false, validate("trailingspace "), "package name should not have leading or trialing space");
}

#[test]
fn name_cannot_be_on_blacklist() {
  assert_eq!(false, validate("node_modules"), "package name should not be on blacklist");
  assert_eq!(false, validate("favicon.ico"), "package name should not be on blacklist");
}

#[test]
fn name_cannot_be_a_builtin_module() {
  assert_eq!(false, validate("http"), "package name should not be a builtin module");
  assert_eq!(false, validate("fs"), "package name should not be a builtin module");
}

#[test]
fn name_cannot_be_longer_than_214_chars() {
  let bad_name = "ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyou-";
  let good_name = "ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyou";
  assert_eq!(false, validate(bad_name), "package name should not be longer than 214 chars");
  assert_eq!(true, validate(good_name), "package name should not be longer than 214 chars");
}

#[test]
fn name_cannot_have_uppercase_characters() {
  assert_eq!(false, validate("MiXeDCase"), "package name should be all lowercase characters");
  assert_eq!(true, validate("lowercase"), "package name should be all lowercase characters");
}
