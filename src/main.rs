extern crate semver;
extern crate cargo;
#[macro_use]
mod easy_resolver;
use cargo::core::{Dependency, Summary};
use std::collections::HashMap;
use easy_resolver::*;

fn main() {

    let mut registry = vec!(
        pkg!("foo", "1.0.0"),
        pkg!("foo", "2.0.0"),
        pkg!("foo", "0.1.0"),
        pkg!("foo", "0.2.0"),
        pkg!("food", "0.2.0"),
        pkg!("d1", "0.4.2" => [dep("foo", "1"), dep("food", "0.2.*")]),
        pkg!("d2", "0.4.2" => [dep("foo", "2")]),
        pkg!("d3", "0.4.2" => [dep("foo", "0.1")]),
        pkg!("d4", "0.4.2" => [dep("foo", "0.2")]),


        pkg!("base_project", "1.0.0" => [dep("d1", "*")]),
    );

    let packages = vec![
        dep("base_project", "*"),
    ];

    let res = resolve(packages, &mut registry).unwrap();

    for package in res {
        println!("{} {}", package.name(), package.version());
    }
}
