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

pub fn uumain(args: Vec<String>) -> i32 {
    for arg in args {
        println!("{}", arg);
    }
    0
}
