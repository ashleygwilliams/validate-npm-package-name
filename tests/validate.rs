extern crate validate_npm_package_name;

use validate_npm_package_name::validate;

#[test]
fn test_valid_name() {
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
fn test_name_cannot_have_exclamation_mark() {
  let pkg = "crazy!";
  let result = validate(pkg);
  assert_eq!(result, false, "packages cannot contain an exclamation mark");
}
