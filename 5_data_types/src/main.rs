mod compound_types;
mod scalar_types;

fn main() {
    scalar_types::numeric_types::integer();
    scalar_types::numeric_types::float();
    scalar_types::numeric_types::numeric_operations();
    scalar_types::boolean::boolean();
    scalar_types::char::char();
    compound_types::compound_types();
}
