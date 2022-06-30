use crate::exec::Executor;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DatabaseRule {
    pub name: String,
    // TODO: place rule
}
impl DatabaseRule {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn db_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct Db {
    rule: DatabaseRule,
    executor: Executor,
}
