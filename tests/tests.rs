use std::collections::HashMap;

use mollusk_svm::{program::keyed_account_for_system_program, result::Check, Mollusk};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use pinocchio::sysvars::clock::Epoch;
use pinocchio_counter::{Counter, Increment, Initialize};
// use pinocchio_counter::ID;
// use solana_pubkey::Pubkey;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

const ID: Pubkey = solana_sdk::pubkey!("22222222222222222222222222222222222222222222");

// Alternative using an Array of bytes
// pub const ID: [u8; 32] = [
//    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
//    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
//    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
//    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
// ];
#[test]
fn test() {
    // let mollusk = Mollusk::new(&Pubkey::new_from_array(ID), "target/deploy/pinocchio_counter");
    let mollusk = Mollusk::new(&ID, "target/deploy/pinocchio_counter");

    // let (counter, _) = Pubkey::find_program_address(&[b"counter"], &ID);
    let user = Pubkey::new_unique();
    let counter = Pubkey::new_unique();

    let accounts_to_start_with = [
        (counter, Account::new(0, 0, &system_program::ID)),
        (user, Account::new(1_000_000_000, 0, &system_program::ID)),
        keyed_account_for_system_program(),
    ];
    let mollusk = mollusk.with_context(HashMap::from_iter(accounts_to_start_with));


    pub fn keyed_account_for_counter(
      mollusk: &mut Mollusk,
      key: Pubkey,
  ) -> (Pubkey, Account) {
      let counter_account = Counter {
          count: 0,
      };
  
      let lamports = mollusk
          .sysvars
          .rent
          .minimum_balance(core::mem::size_of::<Counter>());
  
      let data = counter_account.count.to_le_bytes().to_vec();
  
      let account = Account {
          lamports,
          data,
          owner: ID,
          executable: false,
          rent_epoch: Epoch::default(),
      };
  
      (key, account)
  }
    let init_ix = Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(counter, true),
            AccountMeta::new(user, true),
            AccountMeta::new(system_program::ID, false),
        ],
        data: vec![*Initialize::DISCRIMINATOR],
    };

    let mut expected_counter_state = Counter { count: 0 };
    //Initialize Counter
    mollusk.process_and_validate_instruction(
        &init_ix,
        &[
            Check::success(),
            Check::account(&counter)
                .data(&expected_counter_state.count.to_le_bytes().to_vec())
                .build(),
        ],
    );

    //Increment
    expected_counter_state.count = 1;
    let increment_ix = Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(counter, false),
            AccountMeta::new(user, true),
        ],
        data: vec![*Increment::DISCRIMINATOR],
    };

    mollusk.process_and_validate_instruction(
        &increment_ix,
        &[
            Check::success(),
            Check::account(&counter)
                .data(&expected_counter_state.count.to_le_bytes().to_vec())
                .build(),
        ],
    );

    let mut mollusk1 = Mollusk::new(&ID, "target/deploy/pinocchio_counter");
    let (_, counteraccount) =  keyed_account_for_counter(& mut mollusk1, counter);

    MolluskComputeUnitBencher::new(mollusk1)
        .bench((
            "Initialize Counter",
            &init_ix,
            &[
                (counter, Account::new(0, 0, &system_program::ID)),
                (user, Account::new(1_000_000_000, 0, &system_program::ID)),
                keyed_account_for_system_program(),
            ],
        ))
        .bench((
            "Increment Counter",
            &increment_ix,
            &[
                (counter, counteraccount),
                (user, Account::new(1_000_000_000, 0, &system_program::ID)),
                keyed_account_for_system_program(),
            ],
        ))
        .must_pass(true)
        .out_dir("./target/benches")
        .execute();
}
