/// Implement the adapter pattern on any example.

struct LegacyLibrary;

impl LegacyLibrary {
    fn perform_legacy_operation(&self) {
        println!("Legacy operation");
    }
}

trait Modern {
    fn modern_operation(&self);
}

struct Adapter {
    legacy: LegacyLibrary,
}

impl Modern for Adapter {
    fn modern_operation(&self) {
        print!("Adapter: ");
        self.legacy.perform_legacy_operation();
    }
}

fn call_modern_operation(modern: impl Modern) {
    modern.modern_operation();
}

fn main() {
    let legacy = LegacyLibrary;
    let adapter = Adapter { legacy };

    call_modern_operation(adapter);
}
