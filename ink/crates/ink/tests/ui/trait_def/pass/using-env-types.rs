use ink::env::ContractEnv;
use ink_env::Environment;

#[ink::trait_definition]
pub trait TraitDefinition {
    #[ink(message)]
    fn message_1(&self) -> <<Self as ContractEnv>::Env as Environment>::AccountId;
    #[ink(message)]
    fn message_2(&self, input: <<Self as ContractEnv>::Env as Environment>::Balance);
}

fn main() {}
