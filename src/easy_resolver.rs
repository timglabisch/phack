use std::collections::HashMap;

use cargo::core::source::{SourceId};
use cargo::core::{Dependency, PackageId, Summary, Registry};
use cargo::util::{CargoResult, ToUrl};
use cargo::core::resolver::{self, Method};

#[macro_export]
macro_rules! pkg {

    ($pkgid:expr, $pkgvers:expr) => ({
        let d: Vec<Dependency> = vec![];
        Summary::new(easy_resolver::pgk_idv($pkgid.to_string(), $pkgvers.to_string()), d, HashMap::new()).unwrap()
    });

    ($pkgid:expr, $pkgvers:expr => [$($deps:expr),+]) => ({
        let d: Vec<Dependency> = vec![$($deps),+];

        Summary::new(easy_resolver::pgk_idv($pkgid.to_string(), $pkgvers.to_string()), d, HashMap::new()).unwrap()
    });
}

pub fn pgk_idv(s: String, vers: String) -> PackageId {
    PackageId::new(&s, &vers, &registry_loc()).unwrap()
}

pub fn resolve<R: Registry>(deps: Vec<Dependency>, registry: &mut R)-> CargoResult<Vec<PackageId>> {
    let pkg = pkg_id("root");
    let summary = Summary::new(pkg, deps, HashMap::new()).unwrap();
    let method = Method::Everything;
    Ok(try!(resolver::resolve(&summary, method, registry)).iter().map(|p| {
        p.clone()
    }).collect())
}

pub fn dep(name: &str, req: &str) -> Dependency {
    let url = "http://example.com".to_url().unwrap();
    let source_id = SourceId::for_registry(&url);
    Dependency::parse(name, Some(req), &source_id).unwrap()
}

fn pkg_id(name: &str) -> PackageId {
    PackageId::new(name, "1.0.0", &registry_loc()).unwrap()
}

fn registry_loc() -> SourceId {
    let remote = "http://example.com".to_url().unwrap();
    SourceId::for_registry(&remote)
}
