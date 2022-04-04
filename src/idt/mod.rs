pub mod interrupt_type;
pub mod vector_gate;

use vector_gate::VectorGate;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct InterruptDescriptorTable {
    /// Divide error
    de: VectorGate,
}
