use proc_macro_count::Count;
use proc_macro_try_from::FromPrimitive;

#[derive(Count, FromPrimitive, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    println!("Our enum has {} variants", Color::count());
    println!("The value represents Blue: {}", Color::Blue as usize);
}

