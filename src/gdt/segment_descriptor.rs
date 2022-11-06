#[derive(Debug)]
pub enum SegmentType {
}

#[derive(Debug)]
#[repr(C)]
pub struct SegmentDescriptor {
    pub base1: u8,

    pub g: bool,
    pub db: bool,
    pub l: bool,
    pub avl: bool,

    pub seg_limit1: u8,

    pub p: bool,
    pub dpl: u8,
    pub desc_type: SegmentType,
    pub base2: u8,

    pub base_addr: u16,
    pub seg_limit2: u16,
}


