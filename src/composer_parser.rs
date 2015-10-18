use rustc_serialize::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use curl::http;
use dependency_container::base_project_dependency::BaseProjectDependencyContainer;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ComposerJson {
    pub require: HashMap<String, String>
}

pub fn getBaseProjectDependencyContainer() -> BaseProjectDependencyContainer {

    let mut f = File::open("/Users/tim/proj_/symfony/symfony/composer.json").unwrap();
    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents);


    let mut d = BaseProjectDependencyContainer::new();

    // Deserialize using `json::decode`
    let composer_json: ComposerJson = json::decode(&mut file_contents).unwrap();


    for (a, b) in composer_json.require {
        d.addDependency(a.to_string(), b.to_string());
    }

    d

}
