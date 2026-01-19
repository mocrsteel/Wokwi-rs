//! # Servo
//!
//! Servo interface for the Arduino Mega 2560 board.
//!
//! ## Servo control
//!
//! * 20 ms PWM period required = 50 Hz
//! * Control range: 0.5 to 2.5 ms for a 0 to 180 degrees rotation.
//!
//! ## Arduino Mega2560 interface
//!
//! * Timer/Counters
//!   * TC0: 8-bit
//!   * TC1: 16-bit
//!   * TC2: 8-bit
//!   * TC3: 16-bit
//!   * TC4: 16-bit
//!   * TC5: 16-bit
//!
//! ### 16-bit Timer/Counter application
//!
//! #### Prescaler selection
//!
//! Best possible control resolution (Direct clock, no prescaler):
//! * 16 bit register = 2^16 = 65 536 maximum range for TOP.
//! * Prescaler:
//!     * Direct: 16 MHz = 62.5 ns per clock tick --> 4 ms maximum cycle time.
//!     * Prescale8: 16M/8 = 2 MHz = 0.5 us per clock tick
//!         * 32.8 ms maximum cycle time
//!         * Resolution: 180 degrees in 4000 steps = 0.045 degrees per step.
//!     * Prescale64: 16M/64 = 250 kHz = 4 us per clock tick
//!         * 262 ms maximum cycle time
//!         * Resolution: 180 degrees in  500 steps = 0.36 degrees per tick.
//!
//! #### PWM configuration
//!
//! We want to achieve a 50 Hz PWM period. We'll use WGM mode 14 as with this mode
//! we're able to set TOP using ICRn. That will define the overall PWM frequency.
//! The duty will be set using the channel output compare OCRnx.
//!
//! The desired frequency calculation is defined in ATMega docs on page 148:
//!
//! f<sub>OCnxPWM</sub> = f<sub>csl_I/O</sub> / (N * (1 + TOP))
//!
//! where:
//! * f<sub>clk_I/O</sub> is the system clock frequency in Hz (16e6 Hz).
//! * f<sub>OCnxPWM</sub> is the PWM frequency in Hz.
//! * N is the prescale factor (1, 8, 64, 256 or 1024).
//!
//! ##### Prescale8
//!
//! To achieve a 50 Hz PWM frequency with Prescale8, we need to set TOP to 39 999 (0xC34F).
//! This will lead to a
//! ##### Prescale64
//!
//! To achieve a 50 Hz PWM frequency with Prescale64, we need to set TOP to 4 999 (0x1387).
//!
//! * TOP: 5003 = 0x138B
//!
//! We'll choose for Fast PWM with TOP set by ICRx (1, 3, 4 or 5).
//! OCRnx will be used to set the duty cyle.
//!
//! Control range with prescale64:
//! * 0.5 ms = 125 ticks
//! * 2.5 ms = 625 ticks

use arduino_hal::hal::port::{PB6, PB5, PE5, PE4, PE3, PH5, PH4, PH3, PL5, PL4, PL3};
use arduino_hal::pac::{tc1, tc3, tc4, tc5};
use arduino_hal::pac::{TC1, TC3, TC4, TC5};
use arduino_hal::port::{Pin, mode::{Input, Floating},};

use avr_device::generic::{Reg, RegisterSpec};
// Traits to define which ports and timers we accept for the servo driver.

/// Port pin linked to 16-bit timers TC1, TC3, TC4 or TC5.
pub trait ServoPin<TC> {}      // , OCRNXSpec

impl ServoPin<TC1> for Pin<Input<Floating>, PB6> {}  // , tc1::ocr1b::OCR1B_SPEC
impl ServoPin<TC1> for Pin<Input<Floating>, PB5> {}  // , tc1::ocr1a::OCR1A_SPEC
impl ServoPin<TC3> for Pin<Input<Floating>, PE5> {}  // , tc3::ocr3c::OCR3C_SPEC
impl ServoPin<TC3> for Pin<Input<Floating>, PE4> {}  // , tc3::ocr3b::OCR3B_SPEC
impl ServoPin<TC3> for Pin<Input<Floating>, PE3> {}  // , tc3::ocr3a::OCR3A_SPEC
impl ServoPin<TC4> for Pin<Input<Floating>, PH5> {}  // , tc4::ocr4c::OCR4C_SPEC
impl ServoPin<TC4> for Pin<Input<Floating>, PH4> {}  // , tc4::ocr4b::OCR4B_SPEC
impl ServoPin<TC4> for Pin<Input<Floating>, PH3> {}  // , tc4::ocr4a::OCR4A_SPEC
impl ServoPin<TC5> for Pin<Input<Floating>, PL5> {}  // , tc5::ocr5c::OCR5C_SPEC
impl ServoPin<TC5> for Pin<Input<Floating>, PL4> {}  // , tc5::ocr5b::OCR5B_SPEC
impl ServoPin<TC5> for Pin<Input<Floating>, PL3> {}  // , tc5::ocr5a::OCR5A_SPEC


/// Timer/Counter with 16-bit register TCNTn.
pub trait ServoTC {
    // type TCCRNASpec: RegisterSpec;
    // type TCCRNBSpec: RegisterSpec;
    // type TCCRNCSpec: RegisterSpec;
    // type OCRNASpec: RegisterSpec;
    // type OCRNBSpec: RegisterSpec;
    // type OCRNCSpec: RegisterSpec;
    // type ICRNSpec: RegisterSpec;

    // fn tccrna(&self) -> &Reg<Self::TCCRNASpec>;
    // fn tccrnb(&self) -> &Reg<Self::TCCRNBSpec>;
    // fn tccrnc(&self) -> &Reg<Self::TCCRNCSpec>;
    // fn ocrna(&self) -> &Reg<Self::OCRNASpec>;
    // fn ocrnb(&self) -> &Reg<Self::OCRNBSpec>;
    // fn ocrnc(&self) -> &Reg<Self::OCRNCSpec>;
    // fn icrn(&self) -> &Reg<Self::ICRNSpec>;
}

impl ServoTC for TC1 {}
impl ServoTC for TC3 {}
impl ServoTC for TC4 {}
impl ServoTC for TC5 {}
 

pub struct Servo<TC, PORT> {
    servo_pin: Pin<Input<Floating, PORT>>,
}

impl<TC> Servo<TC> {
    pub fn into_servo(tc: TC) -> Self {
        Servo { pin, tcn }
    }
}


// It won't actually run these tests, but it allows to test handling the Servo struct purely
// based on type definitions.
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn creates_new_servo() {
//         let dp = arduino_hal::Peripherals::take().unwrap();
//         let pins = arduino_hal::pins!(dp);
//         let tc3 = dp.TC3;

//         let servo = Servo::new(pins.d11.into_output().downgrade(), tc2);
//     }
// }
