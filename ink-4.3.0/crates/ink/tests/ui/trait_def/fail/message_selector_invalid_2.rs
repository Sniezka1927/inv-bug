#[ink::trait_definition]
pub trait TraitDefinition {
    #[ink(message, selector)]
    fn message(&self);
}

fn main() {}
