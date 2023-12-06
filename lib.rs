#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod bug {
    use core::result::Result;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TestError {
        MULOverflow,
        DIVOverflow,
        ADDOverflow,
        SUBOverflow,
    }

    #[ink(storage)]
    pub struct Bug {
        last_timestamp: u128,
        value: u128,
    }

    impl Bug {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                last_timestamp: Self::env().block_timestamp() as u128,
                value: u128::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn update_timestamp_diffrent_operations_mul(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication = (1000000000000000000000000000000u128)
                .checked_mul(timestamp_delta)
                .unwrap_or(0);

            let value = multiplication.checked_mul(1000000000000).unwrap_or(0);

            // To don't let compiler ignore the value
            ink::env::debug_println!("value = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp_diffrent_operations_sub(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication = (1000000000000000000000000000000u128)
                .checked_mul(timestamp_delta)
                .unwrap_or(0);

            let value = multiplication.checked_sub(1000000000000).unwrap_or(0);

            // To don't let compiler ignore the value
            ink::env::debug_println!("value = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp_diffrent_operations_add(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication = (1000000000000000000000000000000u128)
                .checked_mul(timestamp_delta)
                .unwrap_or(0);

            let value = multiplication.checked_add(1000000000000).unwrap_or(0);

            // To don't let compiler ignore the value
            ink::env::debug_println!("value = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp_without_store(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication = (1000000000000000000000000000000u128)
                .checked_mul(timestamp_delta)
                .unwrap_or(0);

            let value = multiplication.checked_div(1000000000000).unwrap_or(0);

            // To don't let compiler ignore the division
            ink::env::debug_println!("division = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp(&mut self) {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            // let timestamp_delta = 10_000;

            // 10^30 * delta
            let multiplication = (1000000000000000000000000000000u128)
                .checked_mul(timestamp_delta)
                .unwrap_or(0);
            let division = multiplication.checked_div(1000000000000).unwrap_or(0);

            let value = division;
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
        // use ink_e2e::build_message;
        use ink::prelude::string::String;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        type Environment = <BugRef as ink::env::ContractEnv>::Env;
        use ink_e2e::ContractsBackend;

        use ink::env::call::utils::{Argument, ArgumentList, EmptyArgumentList};

        fn build_message(
            account_id: AccountId,
            selector: [u8; 4],
            message: String,
        ) -> ink_e2e::CallBuilderFinal<
            Environment,
            ArgumentList<Argument<String>, EmptyArgumentList>,
            (),
        > {
            ink::env::call::build_call::<Environment>()
                .call(account_id)
                .exec_input(
                    ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(selector))
                        .push_arg(message),
                )
                .returns::<()>()
        }

        #[ink_e2e::test]
        async fn check_timestamps_with_store(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = BugRef::new();
            let contract_account_id = client
                .instantiate("bug", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            const ARBITRARY_SELECTOR: [u8; 4] = [0xF9, 0xF9, 0xF9, 0xF9];
            let wildcard_message = "WILDCARD_MESSAGE 1".to_string();
            let wildcard = build_message(
                contract_account_id,
                ARBITRARY_SELECTOR,
                wildcard_message.clone(),
            );

            // let x = ink::env::call::build_call::<Environment>().call(contract_account_id.clone()).exec_input(
            //     ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(
            //         ARBITRARY_SELECTOR,
            //     ))
            //     .push_arg(1000000000000u128),
            // ).returns::<()>();

            // let msg = build_message::<BugRef>(contract_account_id.clone())
            //     .call(|bug| bug.get_timestamps());
            // let result = client
            //     .call(&ink_e2e::bob(), msg, 0, None)
            //     .await
            //     .expect("Getting timestamps failed")
            //     .return_value();

            // let mut last_timestamp = result.0;
            for n in 1..10 {
                // let msg = build_message::<BugRef>(contract_account_id.clone())
                //     .call(|bug| bug.get_timestamps());

                // let result = client
                //     .call(&ink_e2e::bob(), msg, 0, None)
                //     .await
                //     .expect("Getting timestamps failed")
                //     .return_value();

                // println!(
                //     "timestamp diff = {:?}  [{:?} - {:?}]",
                //     result.0 - last_timestamp,
                //     result.0,
                //     last_timestamp
                // );

                // last_timestamp = result.0;

                // let msg = build_message::<BugRef>(contract_account_id.clone())
                //     .call(|bug| bug.update_timestamp());

                // let result_dry_run = client.call_dry_run(&ink_e2e::bob(), &msg, 0, None).await;

                // let mut required_gas = result_dry_run.exec_result.gas_required;

                // println!(
                //     "Dry run gas required = {:?}",
                //     result_dry_run.exec_result.gas_required
                // );
                // required_gas.proof_size *= 2;
                // println!("Increased gas = {:?}", required_gas);
                // let result = client.call(&ink_e2e::bob(), msg, 100, None).await;
                // if result.is_ok() {
                // println!(
                //     "Call gas required = {:?}",
                //     result.unwrap().dry_run.exec_result.gas_required
                // );
                // }
                // else {
                //     println!("Call failed! Retrying...");
                //     let mut went_through = false;
                //     let mut attempt = 1;
                //     while !went_through && attempt < 10 {

                //         let msg = build_message::<BugRef>(contract_account_id.clone())
                //         .call(|bug| bug.update_timestamp());
                //         let result_dry_run = client.call_dry_run(&ink_e2e::bob(), &msg, 0, None).await;
                //         println!(
                //             "   Dry run gas required = {:?}",
                //             result_dry_run.exec_result.gas_required
                //         );
                //         let result = client.call(&ink_e2e::bob(), msg, 0, None).await;
                //         if result.is_ok() {
                //             went_through = true;
                //             println!(
                //              "   Call gas required = {:?}",
                //             result.unwrap().dry_run.exec_result.gas_required
                //             );
                //         } else {
                //             println!("  Call failed! Attempt = {:?}", attempt);
                //             attempt += 1;
                //         };
                //     }

                // }
                println!("");
            }

            Ok(())
        }

        // #[ink_e2e::test]
        // async fn check_timestamps_without_store(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     let constructor = BugRef::new();
        //     let contract_account_id = client
        //         .instantiate("bug", &ink_e2e::bob(), constructor, 0, None)
        //         .await
        //         .expect("instantiate failed")
        //         .account_id;

        //     for n in 1..200 {
        //         println!("n = {}", n);
        //         let msg = build_message::<BugRef>(contract_account_id.clone())
        //             .call(|bug| bug.update_timestamp_without_store());
        //         let _result = client
        //             .call(&ink_e2e::bob(), msg, 0, None)
        //             .await
        //             .expect("updating failed");
        //     }
        //     Ok(())
        // }

        // #[ink_e2e::test]
        // async fn check_timestamps_diffrent_operations_add(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     let constructor = BugRef::new();
        //     let contract_account_id = client
        //         .instantiate("bug", &ink_e2e::bob(), constructor, 0, None)
        //         .await
        //         .expect("instantiate failed")
        //         .account_id;

        //     for n in 1..200 {
        //         println!("n = {}", n);
        //         let msg = build_message::<BugRef>(contract_account_id.clone())
        //         .call(|bug| bug.update_timestamp_diffrent_operations_add());

        //         let result_dry_run = client
        //             .call_dry_run(&ink_e2e::bob(), &msg, 0, None)
        //             .await
        //             .return_value();

        //         println!("dry run error = {:?}", result_dry_run); // Ok(())

        //         if result_dry_run.is_err() {
        //             println!("dry run failed");
        //             println!("dry run error = {:?}", result_dry_run);
        //             return Ok(());
        //         }

        //         let _result = client
        //             .call(&ink_e2e::bob(), msg, 0, None)
        //             .await
        //             .expect("updating failed");
        //     }
        //     Ok(())
        // }

        // #[ink_e2e::test]
        // async fn check_timestamps_diffrent_operations_sub(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     let constructor = BugRef::new();
        //     let contract_account_id = client
        //         .instantiate("bug", &ink_e2e::bob(), constructor, 0, None)
        //         .await
        //         .expect("instantiate failed")
        //         .account_id;

        //     for n in 1..200 {
        //         println!("n = {}", n);
        //         let msg = build_message::<BugRef>(contract_account_id.clone())
        //         .call(|bug| bug.update_timestamp_diffrent_operations_add());

        //         let result_dry_run = client
        //             .call_dry_run(&ink_e2e::bob(), &msg, 0, None)
        //             .await
        //             .return_value();

        //         println!("dry run error = {:?}", result_dry_run); // Ok(())

        //         if result_dry_run.is_err() {
        //             println!("dry run failed");
        //             println!("dry run error = {:?}", result_dry_run);
        //             return Ok(());
        //         }

        //         let _result = client
        //             .call(&ink_e2e::bob(), msg, 0, None)
        //             .await
        //             .expect("updating failed");
        //     }
        //     Ok(())
        // }

        // #[ink_e2e::test]
        // async fn check_timestamps_diffrent_operations_mul(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     let constructor = BugRef::new();
        //     let contract_account_id = client
        //         .instantiate("bug", &ink_e2e::bob(), constructor, 0, None)
        //         .await
        //         .expect("instantiate failed")
        //         .account_id;

        //     for n in 1..200 {
        //         println!("n = {}", n);
        //         let msg = build_message::<BugRef>(contract_account_id.clone())
        //         .call(|bug| bug.update_timestamp_diffrent_operations_add());

        //         let result_dry_run = client
        //             .call_dry_run(&ink_e2e::bob(), &msg, 0, None)
        //             .await
        //             .return_value();

        //         println!("dry run error = {:?}", result_dry_run); // Ok(())

        //         if result_dry_run.is_err() {
        //             println!("dry run failed");
        //             println!("dry run error = {:?}", result_dry_run);
        //             return Ok(());
        //         }

        //         let _result = client
        //             .call(&ink_e2e::bob(), msg, 0, None)
        //             .await
        //             .expect("updating failed");
        //     }
        //     Ok(())
        // }
    }
}
