#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ComponentDefinition {
    pub name: String,
    pub provided_services: Vec<ProvidedService>,
    pub required_services: Vec<RequiredService>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ProvidedService {
    pub name: String
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RequiredService {
    pub name: String,
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

