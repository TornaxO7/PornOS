use x86_64::PhysAddr;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageFrame<> {
    pub start: PhysAddr,
}
