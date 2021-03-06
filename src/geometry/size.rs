// Copyright (c) 2020 - BytePlug
//
// This source file is part of Distilled Multimedia Library which is released
// under the MIT license. Please refer to the LICENSE file that can be found
// at the root of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, January 2020

/// Brief description
///
/// The **Size struct** is not documented yet. Pull requests are welcome.
///
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size<T = i32> {
    pub width:  T,
    pub height: T
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size::<T> {
            width: width,
            height: height
        }
    }
}
