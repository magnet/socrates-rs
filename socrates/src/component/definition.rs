use super::super::service::query::ServiceQuery;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ComponentDefinition {
    pub name: String,
    pub provides: Vec<Provide>,
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Provide {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reference {
    pub name: String,
    pub svc_name: String, // This is just for human-readability
    pub svc_query: ServiceQuery,
    pub options: ReferenceOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ReferenceOptions {
    pub cardinality: Cardinality,
    pub policy: Policy,
    pub policy_option: PolicyOption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cardinality {
    Optional,
    Mandatory,
    Multiple,
}

impl Default for Cardinality {
    fn default() -> Cardinality {
        Cardinality::Mandatory
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Policy {
    Static,
    Dynamic,
}

impl Default for Policy {
    fn default() -> Policy {
        Policy::Static
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PolicyOption {
    Greedy,
    Reluctant,
}

impl Default for PolicyOption {
    fn default() -> PolicyOption {
        PolicyOption::Greedy
    }
}
