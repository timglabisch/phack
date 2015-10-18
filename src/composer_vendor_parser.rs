use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJson {
    pub packages : Vec<ComposerVendorJsonPackage>
}

#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJsonPackage {
    pub packages : HashMap<String, Vec<ComposerVendorJsonPackageVersion>>
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
//    pub type : String, // git for example
    pub url : String,
    pub reference : String
}

#[derive(Debug, RustcDecodable)]
pub struct ComposerVendorJsonPackageVersionDist {
//    pub type : String, // git for example
    pub url : String,
    pub reference : String,
    pub shasum : String
}


impl ComposerVendorJson {

    pub fn new(file : String) -> ComposerVendorJson {

        let mut f = File::open(file).unwrap();
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents);

        let composer_vendor_json : ComposerVendorJson = json::decode(&mut file_contents).unwrap();

        composer_vendor_json
    }

}


#[test]
fn test_name() {
    let f = ComposerVendorJson::new("/Users/tim/proj_/rust/phack/fixture/composer_vendor.js".to_string());

    println!("{:?}", f);
}
