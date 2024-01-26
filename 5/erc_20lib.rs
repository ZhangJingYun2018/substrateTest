#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use base_erc20::{BaseErc20, Error, Result};
    use ink::storage::Mapping;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink[topic]]
        from: Option<AccountId>,
        #[ink[topic]]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approve {
        #[ink[topic]]
        owner: AccountId,
        #[ink[topic]]
        spender: AccountId,
        value: Balance,
    }

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        #[inline]
        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });
            Ok(())
        }
    }

    impl BaseErc20 for Erc20 {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approve {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;
            self.allowances.insert((&from, &to), &(allowance - value));
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        type Event = <Erc20 as ink::reflect::ContractEventBase>::Type;

        fn assert_transfer_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
        }
        #[ink::test]
        fn new_work() {
            let erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.total_supply, 10000);
            assert_eq!(erc20.balance_of(accounts.alice), 10000);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(&emitted_events[0], None, Some(accounts.alice), 10000);
        }

        #[ink::test]
        fn transfer_work() {
            let mut erc20 = Erc20::new(10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.total_supply, 10000);
            assert_eq!(erc20.balance_of(accounts.alice), 10000);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.transfer(accounts.bob, 100), Ok(()));
            assert_eq!(erc20.balance_of(accounts.bob), 100);

            //失败测试
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(
                erc20.transfer(accounts.alice, 1000),
                Err(Error::InsufficientBalance)
            );
            assert_eq!(erc20.balance_of(accounts.alice), 9900);
            assert_eq!(erc20.balance_of(accounts.bob), 100);

            let emitted_events: Vec<ink::env::test::EmittedEvent> =
                ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(&emitted_events[0], None, Some(accounts.alice), 10000);
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(accounts.alice),
                Some(accounts.bob),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_work() {
            let mut erc20 = Erc20::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientAllowance)
            );
            assert_eq!(erc20.approve(accounts.bob, 100), Ok(()));
            assert_eq!(ink::env::test::recorded_events().count(), 2);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            assert_eq!(erc20.balance_of(accounts.eve), 10);
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 1000),
                Err(Error::InsufficientAllowance)
            );
            assert_eq!(erc20.balance_of(accounts.eve), 10);
            assert_eq!(erc20.balance_of(accounts.alice), 990);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(&emitted_events[0], None, Some(accounts.alice), 1000);
            assert_transfer_event(
                &emitted_events[2],
                Some(accounts.alice),
                Some(accounts.eve),
                10,
            );
        }

        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;
            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[ink_e2e::test]
            async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let total_supply = 1_000_000_000;
                let constructor = Erc20Ref::new(total_supply);
                let contract_acc_id = client
                    .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // when
                let total_supply_msg = build_message::<Erc20Ref>(contract_acc_id.clone())
                    .call(|erc20| erc20.total_supply());
                let total_supply_res = client
                    .call_dry_run(&ink_e2e::bob(), &total_supply_msg, 0, None)
                    .await;

                let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
                let transfer_to_bob = 500_000_000u128;
                let transfer = build_message::<Erc20Ref>(contract_acc_id.clone())
                    .call(|erc20| erc20.transfer(bob_account.clone(), transfer_to_bob));
                let _transfer_res = client
                    .call(&ink_e2e::alice(), transfer, 0, None)
                    .await
                    .expect("transfer failed");

                let balance_of = build_message::<Erc20Ref>(contract_acc_id.clone())
                    .call(|erc20| erc20.balance_of(bob_account));
                let balance_of_res = client
                    .call_dry_run(&ink_e2e::alice(), &balance_of, 0, None)
                    .await;

                // then
                assert_eq!(
                    total_supply,
                    total_supply_res.return_value(),
                    "total_supply"
                );
                assert_eq!(transfer_to_bob, balance_of_res.return_value(), "balance_of");

                Ok(())
            }

            #[ink_e2e::test]
            async fn e2e_allowances(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let total_supply = 1_000_000_000;
                let constructor = Erc20Ref::new(total_supply);
                let contract_acc_id = client
                    .instantiate("erc20", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // when

                let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
                let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

                let amount = 500_000_000u128;
                let transfer_from =
                    build_message::<Erc20Ref>(contract_acc_id.clone()).call(|erc20| {
                        erc20.transfer_from(bob_account.clone(), charlie_account.clone(), amount)
                    });
                let transfer_from_result = client
                    .call(&ink_e2e::charlie(), transfer_from, 0, None)
                    .await;

                assert!(
                    transfer_from_result.is_err(),
                    "unapproved transfer_from should fail"
                );

                // Bob approves Charlie to transfer up to amount on his behalf
                let approved_value = 1_000u128;
                let approve_call = build_message::<Erc20Ref>(contract_acc_id.clone())
                    .call(|erc20| erc20.approve(charlie_account.clone(), approved_value));
                client
                    .call(&ink_e2e::bob(), approve_call, 0, None)
                    .await
                    .expect("approve failed");

                // `transfer_from` the approved amount
                let transfer_from =
                    build_message::<Erc20Ref>(contract_acc_id.clone()).call(|erc20| {
                        erc20.transfer_from(
                            bob_account.clone(),
                            charlie_account.clone(),
                            approved_value,
                        )
                    });
                let transfer_from_result = client
                    .call(&ink_e2e::charlie(), transfer_from, 0, None)
                    .await;
                assert!(
                    transfer_from_result.is_ok(),
                    "approved transfer_from should succeed"
                );

                let balance_of = build_message::<Erc20Ref>(contract_acc_id.clone())
                    .call(|erc20| erc20.balance_of(bob_account));
                let balance_of_res = client
                    .call_dry_run(&ink_e2e::alice(), &balance_of, 0, None)
                    .await;

                // `transfer_from` again, this time exceeding the approved amount
                let transfer_from =
                    build_message::<Erc20Ref>(contract_acc_id.clone()).call(|erc20| {
                        erc20.transfer_from(bob_account.clone(), charlie_account.clone(), 1)
                    });
                let transfer_from_result = client
                    .call(&ink_e2e::charlie(), transfer_from, 0, None)
                    .await;
                assert!(
                    transfer_from_result.is_err(),
                    "transfer_from exceeding the approved amount should fail"
                );

                assert_eq!(
                    total_supply - approved_value,
                    balance_of_res.return_value(),
                    "balance_of"
                );

                Ok(())
            }
        }
    }
}
