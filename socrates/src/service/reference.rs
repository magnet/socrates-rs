use super::*;

pub type ServiceRanking = i32;

/// Total ordering between two services
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceCoreProps {
    pub ranking: ServiceRanking,
    pub id: ServiceId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceRef {
    pub core: ServiceCoreProps,
    pub name: String,
}
