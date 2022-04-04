#[derive(Debug, Clone, Copy)]
pub enum InterruptType {
    Fault,
    Trap,
    Interrupt,
    Abort,
}
