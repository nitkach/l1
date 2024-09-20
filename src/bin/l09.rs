/// The int64 variable is given.
/// Develop a program that sets the nth bit to 1 or 0.

struct BitManipilator(i64);

impl BitManipilator {
    fn new(number: i64) -> Self {
        Self(number)
    }

    fn binary_repr(&self) -> String {
        format!("{:b}", self.0)
    }

    fn set_nth(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    fn reset_nth(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }
}

fn main() {
    let mut bit_manipilator = BitManipilator::new(154_728_591);

    println!("{}", bit_manipilator.binary_repr());

    bit_manipilator.set_nth(4);
    bit_manipilator.set_nth(5);
    bit_manipilator.set_nth(6);

    println!("{}", bit_manipilator.binary_repr());

    bit_manipilator.reset_nth(0);
    bit_manipilator.reset_nth(1);
    bit_manipilator.reset_nth(2);

    println!("{}", bit_manipilator.binary_repr());
}
