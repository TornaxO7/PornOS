#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GateType {
    Reserved1       = 0b0000,
    TSS16Available  = 0b0001,
    LDT             = 0b0010,
    TSS16Busy       = 0b0011,
    CallGate16      = 0b0100,
    TaskGate        = 0b0101,
    InterruptGate16 = 0b0110,
    TrapGate16      = 0b0111,
    Reserved2       = 0b1000,
    TSS32Available  = 0b1001,
    Reserved3       = 0b1010,
    TSS32Busy       = 0b1011,
    CallGate32      = 0b1100,
    Reserved4       = 0b1101,
    InterruptGate32 = 0b1110,
    TrapGate32      = 0b1111,
}

impl GateType {
    /// Converts itself into the bits which can then be simply ORed with the
    /// bits of GateOptions.
    pub fn to_gate_option_bits(self) -> u16 {
        u16::from((self as u8) << 8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GateOptions(u16);

impl GateOptions {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn set_present(&mut self, value: bool) {
        if value {
            self.0 |= 1 << 16;
        } else {
            self.0 &= !(1 << 16);
        }
    }

    pub fn set_dpl(&mut self) {
        todo!()
    }

    pub fn set_type(&mut self, gate_type: GateType) {
        self.0 &= !(0b1111 << 8);
        self.0 |= gate_type.to_gate_option_bits();
    }

    pub unsafe fn set_ist(&mut self, index: u8) {
        const MIN: u8 = 0;
        const MAX: u8 = 0b111;

        if index > MAX {
            panic!("Index has to be in the range: [{}, {}]", MIN, MAX);
        }

        self.0 &= !0b111u16;
        self.0 |= u16::from(index);
    }
}

impl Default for GateOptions {
    fn default() -> Self {
        Self::none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Gate {
    offset1: u16,
    segment_selector: u16,
    options: GateOptions,
    offset2: u16,
    offset3: u32,
    reserved: u32,
}

impl Gate {
    pub fn empty() -> Self {
        Gate::default()
    }
}
