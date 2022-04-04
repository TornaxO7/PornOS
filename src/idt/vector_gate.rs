use super::interrupt_type::InterruptType;

#[derive(Debug, Clone)]
pub struct VectorGate {
    interrupt_type: InterruptType,
    has_error_code: bool,
}

impl VectorGate {
    pub fn new(interrupt_type: InterruptType, has_error_code: bool) -> Self {
        Self {
            interrupt_type,
            has_error_code,
        }
    }

    pub fn has_error_code(&self) -> bool {
        self.has_error_code
    }

    pub fn get_interrupt_type(&self) -> InterruptType {
        self.interrupt_type
    }
}
