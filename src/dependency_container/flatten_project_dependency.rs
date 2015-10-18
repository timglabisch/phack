use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct FlattenProjectDependency {
    pub name : String,
    pub version : String,
    pub repositoryUrl : String
}

impl FlattenProjectDependency {

    pub fn new(name : String, version : String, repositoryUrl : String) -> FlattenProjectDependency
    {
            FlattenProjectDependency {
                name: name,
                version: version,
                repositoryUrl: repositoryUrl
            }
    }

}


#[derive(Debug, Clone)]
pub struct FlattenProjectDependencyContainer {
    pub dependencies : LinkedList<FlattenProjectDependency>
}

impl FlattenProjectDependencyContainer {

    pub fn new() -> FlattenProjectDependencyContainer
    {
        FlattenProjectDependencyContainer { dependencies: LinkedList::new() }
    }

    pub fn addFlattenDependency(&mut self, dependency : FlattenProjectDependency) -> ()
    {
        self.dependencies.push_back(dependency);
    }

}
