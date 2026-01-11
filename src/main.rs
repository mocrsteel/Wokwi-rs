#![no_std]
#![no_main]

use arduino_hal::default_serial;
use arduino_hal::simple_pwm::{IntoPwmPin, Timer0Pwm, Timer1Pwm, Timer2Pwm, Timer3Pwm, Timer4Pwm};
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::pac::TC1;
use panic_halt as _;

use ufmt::{uWrite, uwrite};

mod servo;

pub fn debug_dump<W: uWrite>(serial: &mut W, timer1: &TC1) -> Result<(), <W as uWrite>::Error> {
    serial.write_str("\r")?;
    uwrite!(
        serial,
        "tTCNTn = {:#?};",
        timer1.tcnt1().read().bits(),
    )?;
    uwrite!(
        serial,
        "tccr1a = {}; tccr1b = {};",
        timer1.tccr1a().read().bits(),
        timer1.tccr1b().read().bits()
    )?;
    uwrite!(
        serial,
        "tccr1c = {};",
        timer1.tccr1c().read().bits(),
    )?;
    uwrite!(
        serial,
        " tifr1 = {};"
        timer1.tifr1().read().bits(),
    )?;
    uwrite!(
        serial,
        " timsk1 = {};"
        timer1.timsk1().read().bits(),
    )?;
    uwrite!(
        serial,
        " icr1 = {};"
        timer1.icr1().read().bits(),
    )?;
    uwrite!(
        serial,
        " ocr1a = {};"
        timer1.ocr1a().read().bits(),
    )?;
    serial.write_str("\n")
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 115200);

    let _ = debug_dump(&mut serial, &dp.TC1);
    // let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale1024);
    // let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale256);
    // let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    // let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale8);
    // let timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Direct);
    
    // let mut d4 = pins.d4.into_output().into_pwm(&timer0);
    // let mut d12 = pins.d12.into_output().into_pwm(&timer1);
    // let mut d10 = pins.d10.into_output().into_pwm(&timer2);
    // let mut d2 = pins.d2.into_output().into_pwm(&timer3);
    // let mut d6 = pins.d6.into_output().into_pwm(&timer4);
    
    // d4.enable();
    // d12.enable();
    // d10.enable();
    // d2.enable();    
    // d6.enable();
    
    
    
    // pin 11 PWM setup TC1 channel A
    let tc1 = dp.TC1;
    tc1.tccr1a().reset();
    tc1.tccr1b().reset();
    tc1.tccr1c().reset();
    // Compare mode set-up in register TCCR1A
    // - bits 7:6 - COM1A1:0 = Output compare mode for OC1A (for channel A)
    // - bits 5:4 - COM1B1:0 = Output compare mode for OC1B (for channel B)
    // - bits 3:2 - COM1C1:0 = Output compare mode for OC1C (for channel C)
    tc1.tccr1a().write(|w| unsafe {w.com1a().bits(0b10).wgm1().bits(0b10)}); // non-inverting mode Compare output mode for fast-pwm
    tc1.tccr1b().write(|w| unsafe {w.wgm1().bits(0b11).cs1().prescale_8()});
    
    // Setting TOP to IRC1 to achieve 50 Hz cycle.
    tc1.icr1().write(|w| w.set(39999u16));
    
    // Setting output compare on channel C to define the PWM duty cycle.
    tc1.ocr1a().write(|w| w.set(4000u16));
    
    // Toggle pin 11 to output the OC3A output.
    pins.d11.into_output();
    
    // tTCNTn = 168; 10101000
    // tccr1a = COM1A 10  COM1B 00 COM1C 00 WGM11-10 00;
    // tccr1b = ICNC1 00 RES 0 WGM13-12 10 CS12-11-10 010;
    // tifr1 = 0;
    // timsk1 = 0;
    // icr1 = 39999;
    // ocr1a = 4000;

    // pin 3 PWM setup with TC3 channel B
    let tc3 = dp.TC3;
    tc3.tccr3a().reset();
    tc3.tccr3b().reset();
    tc3.tccr3c().reset();
    tc3.tccr3a().write(|w| unsafe{ w.com3c().bits(0b10)});
    tc3.tccr3b().write(|w| unsafe{ w.wgm3().bits(14).cs3().prescale_64()});
    
    // Setting TOP to define PWM cycle.
    tc3.icr3().write(|w| w.set(4999u16));
    
    // Setting duty cycle on channel C.
    tc3.ocr3c().write(|w| w.set(1000u16));
    
    pins.d3.into_output();
    
    let _ = debug_dump(&mut serial, &tc1);

    loop {}
}
