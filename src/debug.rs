use ufmt::{uWrite, uwrite};
use arduino_hal::pac::{TC0, TC1, TC2, TC3, TC4, TC5};
use arduino_hal::pac::tc0::{
    tcnt0::TCNT0_SPEC,
    tccr0a::TCCR0A_SPEC,
    tccr0b::TCCR0B_SPEC,
};
use avr_device::generic::{Reg, Readable};

/// Gathers all timers in one type for debug printing;
pub trait DebugTC {
    type TCNTSpec: Readable;
    type TCCRNASpec: Readable;
    type TCCRNBSpec: Readable;

    fn tcntn(&self) -> &Reg<Self::TCNTSpec>;
    fn tccrna(&self) -> &Reg<Self::TCCRNASpec>;
    fn tccrnb(&self) -> &Reg<Self::TCCRNBSpec>;
    // fn tccrnc();
    // fn ocrna();
    // fn ocrnb();
    // fn ocrnc();
    // fn icrn();
    // fn tifrn();
    // fn timskn();
}

impl DebugTC for TC0 {
    type TCNTSpec = TCNT0_SPEC;
    type TCCRNASpec = TCCR0A_SPEC;
    type TCCRNBSpec = TCCR0B_SPEC;
    fn tcntn(&self) -> &Reg<TCNT0_SPEC> {
        self.tcnt0()
    }
    fn tccrna(&self) -> &Reg<TCCR0A_SPEC> {
        self.tccr0a()
    }
    fn tccrnb(&self) -> &Reg<Self::TCCRNBSpec> {
        self.tccr0b()
    }
}
impl DebugTC for TC1 {}
impl DebugTC for TC2 {}
impl DebugTC for TC3 {}
impl DebugTC for TC4 {}
impl DebugTC for TC5 {}

pub fn debug_dump<W: uWrite, TCN: DebugTC>(serial: &mut W, tcn: &TCN) -> Result<(), <W as uWrite>::Error> {
    serial.write_str("\r")?;
    uwrite!(
        serial,
        "tTCNTn = {:#?};",
        tcn.tcntn().read().bits(),
    )?;
    uwrite!(
        serial,
        "tccr1a = {}; tccr1b = {};",
        tcn.tccr1a().read().bits(),
        tcn.tccr1b().read().bits()
    )?;
    uwrite!(
        serial,
        "tccr1c = {};",
        tcn.tccr1c().read().bits(),
    )?;
    uwrite!(
        serial,
        " tifr1 = {};"
        tcn.tifr1().read().bits(),
    )?;
    uwrite!(
        serial,
        " timsk1 = {};"
        tcn.timsk1().read().bits(),
    )?;
    uwrite!(
        serial,
        " icr1 = {};"
        tcn.icr1().read().bits(),
    )?;
    uwrite!(
        serial,
        " ocr1a = {};"
        tcn.ocr1a().read().bits(),
    )?;
    serial.write_str("\n")
}
