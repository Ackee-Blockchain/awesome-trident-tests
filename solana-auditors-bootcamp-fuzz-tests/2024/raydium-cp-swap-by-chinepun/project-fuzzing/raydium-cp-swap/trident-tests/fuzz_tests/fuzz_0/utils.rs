use std::sync::{Mutex, OnceLock};

use anchor_lang::{prelude::{Pubkey, Rent}, solana_program::{program_option::COption, program_pack::Pack}};
use anchor_spl::token::spl_token;
use trident_client::prelude::solana_sdk::account::AccountSharedData;
pub struct AmmConfigParameters
{
    pub index: u16
}

pub fn get_create_pool_receiver_path() -> String
{
    let path = "/home/chinedum/OSC/solana_sec/raydium-cp-swap/fuzzing-with-trident-chinepun/project-fuzzing/raydium-cp-swap/admin/create_pool_receiver.json";

    path.to_string()
}

pub fn get_admin_path() -> String
{
    let path = "/home/chinedum/OSC/solana_sec/raydium-cp-swap/fuzzing-with-trident-chinepun/project-fuzzing/raydium-cp-swap/admin/admin.json";

    path.to_string()
}

pub static GLOBAL_COUNTER: OnceLock<Mutex<AmmConfigParameters>> = OnceLock::new();

pub fn increment_index(){
    let counter = GLOBAL_COUNTER.get_or_init(|| Mutex::new(AmmConfigParameters { index: 0 }));  // Initialize if not already set
    let mut num = counter.lock().unwrap();  // Acquire the mutex
    // num.index += 1;
    num.index = 1;
    // return num.index;
}

pub fn return_index() -> u16
{
    let counter = GLOBAL_COUNTER.get_or_init(|| Mutex::new(AmmConfigParameters { index: 0 }));  // Initialize if not already set
    let num = counter.lock().unwrap();  // Acquire the mutex

    return num.index;
}

pub fn set_custom_token_account(
    custom_account: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Option<Pubkey>,
    is_native: Option<u64>,
    delegated_amount: u64,
    close_authority: Option<Pubkey>,
)
-> (Pubkey, AccountSharedData)
{
        let mint_account_key = custom_account;

        let delegate = match delegate {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let is_native = match is_native {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let close_authority = match close_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(spl_token::state::Account::LEN);

        let mut account =
            AccountSharedData::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

        let token_account = spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate,
            state: spl_token::state::AccountState::Initialized,
            is_native,
            delegated_amount,
            close_authority,
        };

        let mut data = vec![0u8; spl_token::state::Account::LEN];
        spl_token::state::Account::pack(token_account, &mut data[..]).unwrap();
        account.set_data_from_slice(&data);
        // ctx.set_account(&mint_account_key, &account);

        (mint_account_key, account)
}