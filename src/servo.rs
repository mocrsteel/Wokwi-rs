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
