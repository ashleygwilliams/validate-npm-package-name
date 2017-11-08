extern crate validate_npm_package_name;

use validate_npm_package_name::validate;

#[test]
fn valid_name() {
  let pkg_names = vec![
    "some-package",
    "example.com",
    "under_score",
    "period.js",
    "123numeric"
  ];
  for name in pkg_names {
    let result = validate(name);
    assert_eq!(result, true, "{} should be a valid package name", name);
  }
}

#[test]
fn name_cannot_have_special_characters() {
  let pkg_names = vec![
    "~tilde",
    "back\\slash",
    "t'ck",
    "exclamation!",
    "(openparens",
    "closeparens)",
    "asteri*sk",
  ];
  for name in pkg_names {
    let result = validate(name);
    assert_eq!(result, false, "package name should not contain special characters");
  }
}

#[test]
fn name_cannot_start_with_period_or_underscore() {
  let pkg_names = vec![
    ".dotstart",
    "_underscorestart",
  ];
  for name in pkg_names {
    let result = validate(name);
    assert_eq!(result, false, "package name should not start with a period or underscore");
  }
}
