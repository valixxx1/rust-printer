/*
MIT License

Copyright (c) 2024 valixxx1

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::{thread, time::Duration};
use std::io::{self, Write};
use std::env;
use std::fs;

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Too few arguments!");
        return;
    }

    let s = fs::read_to_string(&args[1])
        .expect("File doesn't exist!");

    for i in s.chars() {
        handle.write_all(i.to_string().as_bytes()).expect("Error stdout!");
        handle.flush().expect("Error flush!");
        thread::sleep(Duration::from_millis(50));
    }
}
