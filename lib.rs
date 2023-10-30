#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod bug {
    #[ink(storage)]
    pub struct Bug {
        last_timestamp: u128,
        value: u128,
    }

    impl Bug {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                last_timestamp: u128::default(),
                value: u128::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn update_timestamp(&mut self) {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            // let value = 1000000000000000000000000000000 * timestamp_delta / 100;
            let value = (1000000000000000000000000000000 * timestamp_delta) / 1000000000000;

            if value == 0 {
                ink::env::debug_println!("Value is zero");
            }

            self.last_timestamp = current_timestamp as u128;
            self.value = value;
        }

        #[ink(message)]
        pub fn get_timestamps(&self) -> (u128, u128) {
            (self.last_timestamp, self.value)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = BugRef::new();
            let contract_account_id = client
                .instantiate("bug", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let msg = build_message::<BugRef>(contract_account_id.clone())
                .call(|bug| bug.get_timestamps());
            let result = client
                .call(&ink_e2e::bob(), msg, 0, None)
                .await
                .expect("Getting timestamps failed")
                .return_value();

            let mut last_timestamp = result.0;

            for n in 1..10 {
                let msg = build_message::<BugRef>(contract_account_id.clone())
                    .call(|bug| bug.get_timestamps());
                let result = client
                    .call(&ink_e2e::bob(), msg, 0, None)
                    .await
                    .expect("Getting timestamps failed")
                    .return_value();

                println!(
                    "timestamp diff = {:?}  [{:?} - {:?}]",
                    result.0 - last_timestamp,
                    result.0,
                    last_timestamp
                );

                last_timestamp = result.0;

                let msg = build_message::<BugRef>(contract_account_id.clone())
                    .call(|bug| bug.update_timestamp());
                let result = client
                    .call(&ink_e2e::bob(), msg, 0, None)
                    .await
                    .expect("updating failed");
            }

            Ok(())
        }
    }
}
