#[ink::trait_definition]
pub trait TraitDefinition {
    #[ink(message)]
    fn message(&self) {}
}

fn main() {}
