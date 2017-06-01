#![crate_name = "uu_dd"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Agoston Szepessy <agszepp@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE file
 * that was distributed with this source code.
 */

#[macro_use]
extern crate uucore;

use std::fs;

const NAME: &'static str = "dd";
static SUMMARY: &'static str = "Copy a file, converting and formatting according to the operands.";
static LONG_HELP: &'static str = "bs=BYTES        read and write up to BYTES bytes at a time";

enum Status {
    None,
    Noxfer,
    Progress,
}

struct Operands {
    input_file: String,
    output_file: String,
    input_bs: u64,          // input block size in bytes
    output_bs: u64,         // output block size in bytes
    conv_bs: u64,   // conversion block size in bytes
    skip: u64,              // skip n input_bs byte blocks before copying
    seek: u64,              // skip n output_bs bye blocks in output file before copying
    count: u64,             // copy n input_bs byte blocks from input file
    status: Status,
    conv: Vec<String>,      // convert the file as specified
}

impl Operands {
    pub fn new() -> Operands {
        Operands {
            input_file: String::new(),
            output_file: String::new(),
            input_bs: 512,          // copy 512 bytes at a time by default
            output_bs: 512,         // copy 512 bytes at a time by default
            conv_bs: 512,
            skip: 0,
            seek: 0,
            count: 0,
            status: Status::None,
            conv: Vec::<String>::new()
        }
    }
}

pub fn uumain(args: Vec<String>) -> i32 {
    let syntax = format!("Usage: {0} [OPERANDS]...
    or {0} [OPTIONS]", NAME);

    let mut opts = new_coreopts!(&syntax, SUMMARY, LONG_HELP);

    let mut operands = Operands::new();

    // bs operand overrides ibs, and obs so keep track of whether user
    // specifies it
    let mut bs_specified = false;

    // extract all operands from arguments, and then parse options passed in
    for i in 0..args.len() {
        for (j, c) in args[i].chars().enumerate() {
            if c == '=' {
                let operand = &args[i][..j];
                match operand {
                    "bs" => {
                        if let Some(value) = calculate_size(&args[i][(j + 1)..].to_string()) {
                            operands.input_bs = value;
                            operands.output_bs = value;
                        }
                        else {
                            println!("Invalid input");
                        }
                    },
                    "if" => operands.input_file = args[i][(j + 1)..].to_string(),
                    "of" => operands.output_file = args[i][(j + 1)..].to_string(),
                    _ => println!("other operand")
                };
            }
        }
    }

    0
}

/*
 * Calculates the size of the input by multiplying the value by the size of the
 * unit.
 */
fn calculate_size(input: &String) -> Option<u64> {
    // Check if input starts with a number
    if let Some(x) = input.chars().nth(0) {
        if x.is_numeric() {

            // Characters that make up the size
            let mut value_str = String::new();

            // Units of the size
            let mut units = String::new();

            // Separate value from unit
            for(i, c) in input.chars().enumerate() {
                if c.is_numeric() {
                    value_str.push(c);
                }
                else {
                    units.push(c);
                }
            }

            let unit_size: u64 = match &*units {
                "kB" => 1000,
                "k" | "K" | "KiB" => 1 << 10,
                "MB" => 1000000,
                "M" | "MiB" => 1 << 20,
                "GB" => 1000000000,
                "G" | "GiB" => 1 << 30,
                "TB" => 1000000000000,
                "T" | "TiB" => 1 << 40,
                "PB" => 1000000000000000,
                "P" | "PiB" => 1 << 50,
                "EB" => 1000000000000000000,
                "E" | "EiB" => 1 << 60,
                /* Out of range
                "ZB" => 1000000000000000000000,
                "Z" | "ZiB" => 1 << 70,
                "YB" => 1000000000000000000000000,
                "Y" | "YiB" => 1 << 80,
                */
                _ => return None
            };

            // Calculate total size by multiplying input value by the size of
            // the unit
            match value_str.parse::<u64>() {
                Ok(value) => Some(value * unit_size),
                Err(_) => None
            }

        }
        // Input doesn't start with a number
        else {
            None
        }
    }
    // Empty input string
    else {
        None
    }
}
