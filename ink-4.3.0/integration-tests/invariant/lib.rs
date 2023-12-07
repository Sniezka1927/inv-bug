#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod e2e_call_runtime {
    #[ink(storage)]
    #[derive(Default)]
    pub struct Contract {
        last_timestamp: u64,
        timestamp_value: u128,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                last_timestamp: Self::env().block_timestamp(),
                timestamp_value: 0,
            }
        }

        #[ink(message)]
        pub fn get_contract_balance(&self) -> Balance {
            self.env().balance()
        }
        #[ink(message)]
        pub fn update_timestamp(&mut self) {
            let current_timestamp = self.env().block_timestamp();
            let timestamp_delta: u128 = (current_timestamp - self.last_timestamp).into();

            // // 10^30 * delta
            let multiplication =
                (1000000000000000000000000000000u128) * (timestamp_delta);
            // //.unwrap_or(0);
            let division = multiplication / (1000000000000); //.unwrap_or(0);

            self.last_timestamp = current_timestamp;
            // self.timestamp_value = division;
        }

        #[ink(message)]
        pub fn get_timestamps(&self) -> (u64, u128) {
            (self.last_timestamp, self.timestamp_value)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::{build_message, subxt::dynamic::Value};

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn test_timestamps(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let contract_acc_id = client
                .instantiate("e2e_call_runtime", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            let transfer_amount = 100_000_000_000u128;

            let msg = build_message::<ContractRef>(contract_acc_id.clone())
                .call(|contract| contract.get_timestamps());
            let result = client
                .call(&ink_e2e::alice(), msg, 0, None)
                .await
                .expect("Getting timestamps failed")
                .return_value();

            let mut last_timestamp = result.0;

            for n in 1..10 {
                let msg = build_message::<ContractRef>(contract_acc_id.clone())
                    .call(|contract| contract.get_timestamps());
                let result = client
                    .call(&ink_e2e::alice(), msg, 0, None)
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

                let msg = build_message::<ContractRef>(contract_acc_id.clone())
                    .call(|contract| contract.update_timestamp());
                let result = client
                    .call(&ink_e2e::alice(), msg, 0, None)
                    .await
                    .expect("Getting timestamps failed")
                    .return_value();
            }
            Ok(())
        }

        // #[ink_e2e::test]
        // async fn call_runtime_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     // given
        //     let constructor = ContractRef::new();
        //     let contract_acc_id = client
        //         .instantiate("e2e_call_runtime", &ink_e2e::alice(), constructor, 0, None)
        //         .await
        //         .expect("instantiate failed")
        //         .account_id;
        //     let transfer_amount = 100_000_000_000u128;

        //     // when
        //     let call_data = vec![
        //         // A value representing a `MultiAddress<AccountId32, _>`. We want the
        //         // "Id" variant, and that will ultimately contain the
        //         // bytes for our destination address
        //         Value::unnamed_variant("Id", [Value::from_bytes(&contract_acc_id)]),
        //         // A value representing the amount we'd like to transfer.
        //         Value::u128(transfer_amount),
        //     ];

        //     let get_balance = build_message::<ContractRef>(contract_acc_id.clone())
        //         .call(|contract| contract.get_contract_balance());
        //     let pre_balance = client
        //         .call_dry_run(&ink_e2e::alice(), &get_balance, 0, None)
        //         .await
        //         .return_value();

        //     // Send funds from Alice to the contract using Balances::transfer
        //     client
        //         .runtime_call(&ink_e2e::alice(), "Balances", "transfer", call_data)
        //         .await
        //         .expect("runtime call failed");

        //     // then
        //     let get_balance = build_message::<ContractRef>(contract_acc_id.clone())
        //         .call(|contract| contract.get_contract_balance());
        //     let get_balance_res = client
        //         .call_dry_run(&ink_e2e::alice(), &get_balance, 0, None)
        //         .await;

        //     assert_eq!(
        //         get_balance_res.return_value(),
        //         pre_balance + transfer_amount
        //     );

        //     Ok(())
        // }
    }
}