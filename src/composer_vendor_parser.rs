use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use rustc_serialize::json;


#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJson {
    pub packages : HashMap<String, HashMap<String, ComposerVendorJsonPackageVersion>>
}


#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJsonPackageVersion {
    pub name : String,
    pub description : String,
    pub keywords : Vec<String>,
    pub homepage : String,
    pub version : String,
    pub version_normalized : String,
    pub license : Vec<String>,
    pub source : ComposerVendorJsonPackageVersionSource,
    pub require : HashMap<String, String>
}

#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJsonPackageVersionSource {
    //pub _type : String, // git for example
    pub url : String,
    pub reference : String
}

#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJsonPackageVersionDist {
    //pub _type : String, // git for example
    pub url : String,
    pub reference : String,
    pub shasum : String
}


impl ComposerVendorJson {

    pub fn fromFile(file : String) -> ComposerVendorJson {
        let mut f = File::open(file).unwrap();
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents);
        ComposerVendorJson::new(file_contents)
    }

    pub fn new(mut file_contents : String) -> ComposerVendorJson {
        let composer_vendor_json : ComposerVendorJson = json::decode(&mut file_contents).unwrap();

        composer_vendor_json
    }

}


#[test]
fn test_basic_parsing() {
    let f = ComposerVendorJson::new(include_str!("../fixture/composer_vendor.js").to_string());

    assert!(f.packages.contains_key("doctrine/common"));
    assert!(f.packages.get("doctrine/common").unwrap().contains_key("2.0.x-dev"));
    assert!(f.packages.get("doctrine/common").unwrap().contains_key("2.1.3"));

    let dep20xdev = f.packages.get("doctrine/common").unwrap().get("2.0.x-dev").unwrap();
    assert_eq!("doctrine/common", dep20xdev.name);
    assert_eq!("Common Library for Doctrine projects", dep20xdev.description);
    assert_eq!("doctrine/common", dep20xdev.name);
    assert_eq!(vec!["collections", "spl", "eventmanager", "annotations", "persistence"], dep20xdev.keywords);
    assert_eq!("http://www.doctrine-project.org", dep20xdev.homepage);
    assert_eq!("2.0.x-dev", dep20xdev.version);
    assert_eq!("2.0.9999999.9999999-dev", dep20xdev.version_normalized);
    assert_eq!(vec!["LGPL"], dep20xdev.license);
    assert!(dep20xdev.require.contains_key("php"));
    assert_eq!(dep20xdev.require.get("php").unwrap(), ">=5.3.2");
    assert!(dep20xdev.require.len() == 1);
    assert_eq!(dep20xdev.source.url, "https://github.com/doctrine/common.git");
    assert_eq!(dep20xdev.source.reference, "2ef2526003a722f7039b21abd5170ba80655bfea");
}
