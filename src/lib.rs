#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
//! ### Printer operator box

use embedded_hal::digital::v2::{InputPin, OutputPin};

/// Driver for the operator box
pub struct OpBox<I: InputPin, O: OutputPin> {
    clk_pin: O,
    latch_pin: O,
    output_pin: O,
    input_pin: I,
    state_out: [bool; 16],
}

impl<E, I: InputPin<Error = E>, O: OutputPin<Error = E>> OpBox<I, O> {
    /// Creates an new driver with the given pins.
    pub fn new(clk_pin: O, latch_pin: O, output_pin: O, input_pin: I) -> Result<Self, E> {
        let mut driver = Self {
            clk_pin,
            latch_pin,
            output_pin,
            input_pin,
            state_out: [false; 16],
        };

        driver.set_get_shift_registers()?;

        Ok(driver)
    }

    /// Set the LED
    pub fn set_d1(&mut self, state: bool) -> Result<(), E> {
        self.state_out[0] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d2(&mut self, state: bool) -> Result<(), E> {
        self.state_out[1] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d3(&mut self, state: bool) -> Result<(), E> {
        self.state_out[2] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d4(&mut self, state: bool) -> Result<(), E> {
        self.state_out[3] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d5(&mut self, state: bool) -> Result<(), E> {
        self.state_out[4] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d6(&mut self, state: bool) -> Result<(), E> {
        self.state_out[5] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d7(&mut self, state: bool) -> Result<(), E> {
        self.state_out[6] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d8(&mut self, state: bool) -> Result<(), E> {
        self.state_out[7] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the LED
    pub fn set_d9(&mut self, state: bool) -> Result<(), E> {
        self.state_out[8] = state;
        self.set_get_shift_registers().map(|_| ())
    }

    /// Set the 7 segement display
    pub fn set_segment_display(&mut self, display: SegmentDisplay) -> Result<(), E> {
        // Get the output config
        use SegmentDisplay::*;
        let display_state = match display {
            One => [false, false, false, false, true, true, false],
            _ => [false; 7],
        };
        // Copy to internal state
        self.state_out[8..].clone_from_slice(&display_state);

        self.set_get_shift_registers().map(|_| ())
    }

    /// Read the button
    pub fn get_s1(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[0])
    }

    /// Read the button
    pub fn get_s2(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[1])
    }

    /// Read the button
    pub fn get_s3(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[2])
    }

    /// Read the button
    pub fn get_s4(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[3])
    }

    /// Read the button
    pub fn get_s5(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[4])
    }

    /// Read the button
    pub fn get_s6(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[5])
    }

    /// Read the button
    pub fn get_s7(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[6])
    }

    /// Read the button
    pub fn get_s8(&mut self) -> Result<bool, E> {
        self.set_get_shift_registers().map(|inputs| inputs[7])
    }

    fn set_get_shift_registers(&mut self) -> Result<[bool; 8], E> {
        // Make sure latch is low
        self.latch_pin.set_low()?;

        let mut inputs = [false; 8];

        // Clock the state out to the first shift registers
        for (&output, input) in self.state_out[..8].iter().zip(inputs.iter_mut()) {
            self.clk_pin.set_low()?;
            // Set the ouput
            if output {
                self.output_pin.set_high()?;
            } else {
                self.output_pin.set_low()?;
            }
            self.clk_pin.set_high()?;
            // Read the input
            *input = self.input_pin.is_high()?;
        }

        // Clock the state out to the second shift registers
        for &output in self.state_out[8..].iter() {
            self.clk_pin.set_low()?;
            // Set the ouput
            if output {
                self.output_pin.set_high()?;
            } else {
                self.output_pin.set_low()?;
            }
            self.clk_pin.set_high()?;
        }

        // Latch the state to the outputs of the shift registers
        self.latch_pin.set_high()?;

        Ok(inputs)
    }
}

/// Possible States for the display
#[allow(missing_docs, non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentDisplay {
    Zero,
    One,
    Two,
    Thre,
    Four,
    Fife,
    Six,
    Six_Tail,
    Seven,
    Seven_Tail,
    Eight,
    Nine,
    A,
    b,
    C,
    c,
    d,
    E,
    F,
    G,
    H,
    h,
    I,
    J,
    L,
    n,
    O,
    o,
    P,
    q,
    r,
    S,
    t,
    U,
    u,
    y,
    Custom([bool; 7]),
}
