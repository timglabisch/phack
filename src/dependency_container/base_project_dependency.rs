use std::collections::HashMap;

#[derive(Debug)]
pub struct BaseProjectDependencyContainer {
    pub dependencies: HashMap<String, String>
}

impl BaseProjectDependencyContainer {

    pub fn new() -> BaseProjectDependencyContainer
    {
        BaseProjectDependencyContainer { dependencies: HashMap::new() }
    }

    pub fn addDependency(&mut self, name : String, version : String) -> ()
    {
        self.dependencies.insert(name, version);
    }

}
