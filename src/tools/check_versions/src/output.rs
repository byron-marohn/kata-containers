// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use std::fs::OpenOptions;
use std::error::Error;
use std::io::Write;

use crate::cli::Args;

pub fn write_output(content: String, args: &Args) -> Result<(), Box<dyn Error>> {
    match &args.outfile {
        Some(outfile) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(outfile.as_path())?;
            file.write_all(content.as_bytes())?;
        },
        None => ()
    };

    if !args.quiet {
        println!("{}", content);
    }

    Ok(())
}
