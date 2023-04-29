// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MissingVersionError {}

impl fmt::Display for MissingVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MissingVersionError")
    }
}

impl Error for MissingVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None 
    }
}

#[derive(Debug)]
pub struct ParserError {} 

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParserError")
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None 
    }
}
