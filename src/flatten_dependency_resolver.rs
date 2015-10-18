use dependency_container::flatten_project_dependency::{FlattenProjectDependency, FlattenProjectDependencyContainer};
use dependency_container::base_project_dependency::BaseProjectDependencyContainer;
use curl::http;

pub struct FlattenDependencyResolver;

impl FlattenDependencyResolver {

    pub fn resolveFlattenDependency(base_project_dependency : BaseProjectDependencyContainer) -> FlattenProjectDependencyContainer  {

        let flattenContainer = &mut FlattenProjectDependencyContainer::new();

        for (name, version) in base_project_dependency.dependencies {

            if name.contains("/") == false {
                println!("SKIP - {}", name);
                continue;
            }

            let resp = http::handle()
                .get(format!("https://packagist.org/p/{}.json", name))
                .exec().unwrap();

            flattenContainer.addFlattenDependency(
                FlattenProjectDependency::new(name, version, String::from_utf8(resp.move_body()).unwrap())
            )
        }

        flattenContainer.clone()
    }

}
