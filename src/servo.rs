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

use core::marker::PhantomData;

use arduino_hal::hal::port::{PB6, PB5, PE5, PE4, PE3, PH5, PH4, PH3, PL5, PL4, PL3};
use arduino_hal::pac::{tc1, tc3, tc4, tc5};
use arduino_hal::pac::{TC1, TC3, TC4, TC5};
use arduino_hal::port::mode;
use arduino_hal::port::{Pin, mode::{Input, Floating},};
use arduino_hal::simple_pwm::{IntoPwmPin, PwmPinOps};
use avr_device::generic::{Reg, RegisterSpec};
// Traits to define which ports and timers we accept for the servo driver.


pub struct ServoOutput<TC> {
    pub(crate) _timer: PhantomData<TC>,
}

pub trait IntoServoPin<TC, PIN> {
    fn into_servo(self, timer: &TC) -> Pin<ServoOutput<TC>, PIN>;
}

pub trait ServoPinOps<TC> {
    /// Implement traits and and types for Servo purposed pins/timers.
    type Degrees;

    fn enable(&mut self);
    fn disable(&mut self);
    fn set_angle(&self) -> Self::Degrees;
    fn get_max_angle(&self) -> Self::Degrees;
}

// impl <TC, PIN: ServoPinOps<TC>> IntoServoPin<TC, PIN> for Pin<mode::Output, PIN> {
//     fn into_servo(self, _timer: &TC) -> Pin<mode::ServoOutput<TC>, PIN> {
//         Pin {
//             pin: self.pin,
//             _mode: PhantomData,
//         }
//     }
// }


// /// Port pin linked to 16-bit timers TC1, TC3, TC4 or TC5.
// pub trait IntoServo<TC> {
//     fn into_servo(&self) -> ServoOutput<TC>;
// }      // , OCRNXSpec

// impl ServoPin<TC1> for Pin<Input<Floating>, PB6> {}  // , tc1::ocr1b::OCR1B_SPEC
// impl IntoServo<TC1> for Pin<Input<Floating>, PB5> {
//     fn into_servo(&self, timer: TC1) -> ServoOutput<TC1> {

//     }
// }  // , tc1::ocr1a::OCR1A_SPEC
// impl ServoPin<TC3> for Pin<Input<Floating>, PE5> {}  // , tc3::ocr3c::OCR3C_SPEC
// impl ServoPin<TC3> for Pin<Input<Floating>, PE4> {}  // , tc3::ocr3b::OCR3B_SPEC
// impl ServoPin<TC3> for Pin<Input<Floating>, PE3> {}  // , tc3::ocr3a::OCR3A_SPEC
// impl ServoPin<TC4> for Pin<Input<Floating>, PH5> {}  // , tc4::ocr4c::OCR4C_SPEC
// impl ServoPin<TC4> for Pin<Input<Floating>, PH4> {}  // , tc4::ocr4b::OCR4B_SPEC
// impl ServoPin<TC4> for Pin<Input<Floating>, PH3> {}  // , tc4::ocr4a::OCR4A_SPEC
// impl ServoPin<TC5> for Pin<Input<Floating>, PL5> {}  // , tc5::ocr5c::OCR5C_SPEC
// impl ServoPin<TC5> for Pin<Input<Floating>, PL4> {}  // , tc5::ocr5b::OCR5B_SPEC
// impl ServoPin<TC5> for Pin<Input<Floating>, PL3> {}  // , tc5::ocr5a::OCR5A_SPEC3

