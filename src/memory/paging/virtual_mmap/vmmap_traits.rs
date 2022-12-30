use x86_64::structures::paging::PageSize;

use super::{VMMapperGeneral, VMMapperMap, VMmapperUnmap};

/// The main trait which all vm-mappers need to implement.
pub trait VMMapper<P: PageSize>: VMMapperGeneral<P> + VMMapperMap<P> + VMmapperUnmap<P> {}
