// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

//! Give access to the real-time state of the mouse.
//!
//! Additional documentation is to be written here.

/// # An enumeration of the mouse buttons.
///
/// Long description.
///
pub enum Button {
    /// The left button
    Left,

    /// The right button
    Right,

    /// The middle button (usually is the wheel itself)
    Middle,

    /// The extra buttons (variadic, usually on the side of the mouse)
    Extra(u8)
}

/// # An enumeration of the mouse wheels.
///
/// Long description.
///
pub enum Wheel {
    Horizontal,
    Vertical
}

/// # Brief description
///
/// Long description.
///
pub struct Mouse {
}