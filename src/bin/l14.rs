/// Prepare an example of a program that is able to determine
/// the type of variable in runtime, use `std::any::*`.

fn main() {
    let examine_type = Ok::<_, bool>(vec![Some("types at runtime!")]);
    println!(
        "Type of variable: {}",
        std::any::type_name_of_val(&examine_type)
    );
}
