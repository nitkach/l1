use std::io::BufRead;

/// A sequence of lines is received at the input through the standard
/// input stream, the lines can be repeated.
/// It is necessary to output lines to the standard
/// output stream, eliminating repetitions, without using `std::collections::*`.

fn main() {
    let mut buf = String::new();
    let handler = std::io::stdin().lock();

    for line in handler.lines() {
        match line {
            Ok(string) => {
                if string != buf {
                    println!("stdout: {string}");
                    buf = string;
                }
            }
            Err(err) => {
                println!("Error occured: {err}");
                return;
            }
        };
    }
}
