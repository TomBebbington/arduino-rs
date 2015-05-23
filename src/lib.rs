#![feature(asm)]

// Arduino doesn't have snake case
#![allow(non_snake_case)]
use std::mem;
use std::convert::From;

macro_rules! conv(
    ($a:ident = $b:ident) => (
        impl From<$a> for $b {
            fn from(v: $a) -> $b {
                unsafe { mem::transmute(v) }
            }
        }
        impl From<$b> for $a {
            fn from(v: $b) -> $a {
                unsafe { mem::transmute(v) }
            }
        }
    );
);

#[repr(u32)]
#[derive(Eq, PartialEq)]
pub enum Mode {
    Input,
    Output,
    InputPullUp
}
#[repr(u8)]
#[derive(Eq, PartialEq)]
pub enum DigitalValue {
    Low,
    High
}
pub static LOW:u8           = 0x00;
pub static HIGH:u8          = 0x01;
pub static CHANGE:u8        = 0x02;
pub static FALLING:u8       = 0x03;
pub static RISING:u8        = 0x04;

pub static EXTERNAL:u32     = 0x00;
pub static DEFAULT:u32      = 0x01;

pub mod ffi {
    #[link(name = "arduino")]
	extern "C" {
		pub fn init();

		pub fn pinMode(pin:u8, mode:u8);

		pub fn digitalWrite(pin:u8, value:u8);
		pub fn digitalRead(pin:u8) -> i32;

		pub fn analogReference(mode:u8); // TODO: This is an enum, and I _think_ the size is u8 on Cortex Mx
		pub fn analogRead(pin:u8) -> i32;
		pub fn analogWrite(pin:u8, value:i32);

		pub fn analogReadResolution(res:i32);
		pub fn analogWriteResolution(res:i32);

		pub fn tone(pin:u8, frequency:u32, duration:u32);
		pub fn noTone(pin:u8);

		pub fn shiftOut(data_pin:u8, clock_pin:u8, bit_order:u8, value:u8);
		pub fn shiftIn(data_pin:u8, clock_pin:u8, bit_order:u8) -> u8;
		pub fn pulseIn(pin:u8, state:u32, timeout:u32) -> u32;

		pub fn millis() -> u32;
		pub fn micros() -> u32;
		pub fn delay(ms:u32);
		pub fn delayMicroseconds(us:u32);

		pub fn attachInterrupt(pin:u8, callback:extern "C" fn(), mode:u32);
		pub fn detachInterrupt(pin:u8);
	}
}

pub fn init() { unsafe { ffi::init() } }

#[derive(Copy, Clone)]
pub struct DigitalPin(pub u8);
impl DigitalPin {
    /// Write a `HIGH` or `LOW` value to this pin
    pub fn write(self, value: DigitalValue) {
        unsafe { ffi::digitalWrite(self.0, value as u8) }
    }

    /// Reads the value from this digital pin, either `HIGH` or `LOW`
    pub fn read(self) -> DigitalValue {
        unsafe { mem::transmute(ffi::digitalRead(self.0) as u8) }
    }
}

#[derive(Copy, Clone)]
pub struct AnalogPin(pub u8);
impl AnalogPin {
    /// Write a `HIGH` or `LOW` value to this pin
    pub fn write(self, value: i32) {
        unsafe { ffi::analogWrite(self.0, value) }
    }

    /// Reads the value from this analog pin, either `HIGH` or `LOW`
    pub fn read(self) -> i32 {
        unsafe { mem::transmute(ffi::analogRead(self.0)) }
    }
}

#[derive(Copy, Clone)]
pub struct TonePin(pub u8);
impl TonePin {
    /// Emit a tone with the given frequency and duration from this pin
    pub fn tone(self, freq: u32, dur: u32) {
        unsafe { ffi::tone(self.0, freq, dur) }
    }
    /// Stop emitting the tone
    pub fn no_tone(self) {
        unsafe { ffi::noTone(self.0) }
    }
}

#[derive(Copy, Clone)]
pub struct Pin(pub u8);
conv!(Pin = DigitalPin);
conv!(Pin = AnalogPin);
conv!(Pin = TonePin);

impl Pin {
    /// Set the mode of this pin
    pub fn mode(self, mode:Mode) {
        unsafe { ffi::pinMode(self.0, mode as u8) }
    }
    /// Attach an interrupt with the mode saying when it should be triggered
    pub fn attach_interrupt(self, cb: fn(), mode: u32) {
        unsafe { ffi::attachInterrupt(self.0, mem::transmute(cb), mode) }
    }
    /// Disable any registered external interrupt on the given pin
    pub fn detach_interrupt(self) {
        unsafe { ffi::detachInterrupt(self.0) }
    }
}

/// Configures the analog voltage
pub fn analog_reference(mode:u8) {
    unsafe { ffi::analogReference(mode) }
}

/// Sets the size in bits of the value returned by reading an analog pin
pub fn analog_read_resolution(res:i32) { unsafe { ffi::analogReadResolution(res) } }

/// Sets the size in bits of the value returned by writing an analog pin
pub fn analog_write_resolution(res:i32) { unsafe { ffi::analogWriteResolution(res) } }

/// How many milliseconds since the program started
pub fn millis() -> u32 { unsafe { ffi::millis() } }

/// How many microseconds since the program started
pub fn micros() -> u32 { unsafe { ffi::micros() } }

// Delay the program by some milliseconds
pub fn delay(ms:u32) { unsafe { ffi::delay(ms) } }

// Delay the program by some microseconds
pub fn delay_micros(us:u32) { unsafe { ffi::delayMicroseconds(us) } }
