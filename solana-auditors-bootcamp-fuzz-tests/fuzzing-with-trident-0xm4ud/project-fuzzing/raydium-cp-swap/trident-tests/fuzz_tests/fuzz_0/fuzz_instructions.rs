pub mod raydium_cp_swap_fuzz_instructions {
    use std::borrow::BorrowMut;
    use std::ops::RangeFrom;
    use std::str::FromStr;
    use std::u64;

    use crate::{accounts_snapshots::*, PROGRAM_NAME_RAYDIUM_CP_SWAP};
    use anchor_lang::Discriminator;
    use anchor_lang::{system_program, AccountDeserialize};
    use anchor_spl::token::Mint;
    use solana_sdk::epoch_info;
    use solana_sdk::feature_set::spl_token_v3_4_0;
    use solana_sdk::message::Message;
    use solana_sdk::program_pack::Pack;
    use solana_sdk::timing::timestamp;
    use std::mem;
    use std::slice;
    use trident_client::fuzzing::*;
    use trident_client::fuzzing::anchor_lang::AccountSerialize;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::account::{AccountSharedData, WritableAccount};
    use spl_token::state::Account as TokenAccount;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        CreateAmmConfig(CreateAmmConfig),
        //UpdateAmmConfig(UpdateAmmConfig),
       // UpdatePoolStatus(UpdatePoolStatus),
        //CollectProtocolFee(CollectProtocolFee),
        //CollectFundFee(CollectFundFee),
        Initialize(Initialize),
        Deposit(Deposit),
        Withdraw(Withdraw),
        SwapBaseInput(SwapBaseInput),
        SwapBaseOutput(SwapBaseOutput),
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfig {
        pub accounts: CreateAmmConfigAccounts,
        pub data: CreateAmmConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfigAccounts {
        pub owner: AccountId,
        pub amm_config: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfigData {
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfig {
        pub accounts: UpdateAmmConfigAccounts,
        pub data: UpdateAmmConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfigAccounts {
        pub owner: AccountId,
        pub amm_config: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfigData {
        pub param: u8,
        pub value: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatus {
        pub accounts: UpdatePoolStatusAccounts,
        pub data: UpdatePoolStatusData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: AccountId,
        pub pool_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatusData {
        pub status: u8,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFee {
        pub accounts: CollectProtocolFeeAccounts,
        pub data: CollectProtocolFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub amm_config: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub recipient_token_0_account: AccountId,
        pub recipient_token_1_account: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFeeData {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFee {
        pub accounts: CollectFundFeeAccounts,
        pub data: CollectFundFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFeeAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub amm_config: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub recipient_token_0_account: AccountId,
        pub recipient_token_1_account: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFeeData {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub creator: AccountId,
        pub amm_config: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub token_0_mint: AccountId,
        pub token_1_mint: AccountId,
        pub lp_mint: AccountId,
        pub creator_token_0: AccountId,
        pub creator_token_1: AccountId,
        pub creator_lp_token: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub create_pool_fee: AccountId,
        pub observation_state: AccountId,
        pub token_program: AccountId,
        pub token_0_program: AccountId,
        pub token_1_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
        pub rent: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range((u64::MAX -10^5) ..=(u64::MAX- 1)))]
        pub init_amount_0: u64,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range((u64::MAX -10^5) ..=(u64::MAX- 1)))]
        pub init_amount_1: u64,
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(1726495839 ..=(1726495839+20)))]
        pub open_time: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Deposit {
        pub accounts: DepositAccounts,
        pub data: DepositData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub owner_lp_token: AccountId,
        pub token_0_account: AccountId,
        pub token_1_account: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub lp_mint: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositData {
        pub lp_token_amount: u64,
        pub maximum_token_0_amount: u64,
        pub maximum_token_1_amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,

    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub owner_lp_token: AccountId,
        pub token_0_account: AccountId,
        pub token_1_account: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub lp_mint: AccountId,
        pub memo_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawData {
        pub lp_token_amount: u64,
        pub minimum_token_0_amount: u64,
        pub minimum_token_1_amount: u64,

    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInput {
        pub accounts: SwapBaseInputAccounts,
        pub data: SwapBaseInputData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInputAccounts {
        pub payer: AccountId,
        pub authority: AccountId,
        pub amm_config: AccountId,
        pub pool_state: AccountId,
        pub input_token_account: AccountId,
        pub output_token_account: AccountId,
        pub input_vault: AccountId,
        pub output_vault: AccountId,
        pub input_token_program: AccountId,
        pub output_token_program: AccountId,
        pub input_token_mint: AccountId,
        pub output_token_mint: AccountId,
        pub observation_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInputData {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutput {
        pub accounts: SwapBaseOutputAccounts,
        pub data: SwapBaseOutputData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutputAccounts {
        pub payer: AccountId,
        pub authority: AccountId,
        pub amm_config: AccountId,
        pub pool_state: AccountId,
        pub input_token_account: AccountId,
        pub output_token_account: AccountId,
        pub input_vault: AccountId,
        pub output_vault: AccountId,
        pub input_token_program: AccountId,
        pub output_token_program: AccountId,
        pub input_token_mint: AccountId,
        pub output_token_mint: AccountId,
        pub observation_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutputData {
        pub max_amount_in: u64,
        pub amount_out: u64,
    }
    impl<'info> IxOps<'info> for CreateAmmConfig {
        type IxData = raydium_cp_swap::instruction::CreateAmmConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateAmmConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CreateAmmConfig {
                index: 0,
                trade_fee_rate: 10,
                protocol_fee_rate: 1000,
                fund_fee_rate: 25000,
                create_pool_fee: 0,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {

 
            let owner = fuzz_accounts.creator.get_or_create_account(
                self.accounts.owner, 
                client, 
                10 * LAMPORTS_PER_SOL
            );

            /* 
            let (amm_config_pda, bump) = Pubkey::find_program_address(
                &[b"amm_config", &index.to_le_bytes()],
                &raydium_cp_swap::ID,
            );*/
            let index: u16 = 0;
            let mut amm_config_pda = fuzz_accounts.amm_config.get_or_create_account(
                self.accounts.amm_config, 
                &[b"amm_config", &index.to_le_bytes()], 
                &raydium_cp_swap::ID
            ).unwrap();


            let signers = vec![owner.clone()];
            let acc_meta = raydium_cp_swap::accounts::CreateAmmConfig {
                owner: owner.pubkey(),
                amm_config: amm_config_pda.pubkey,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }/*


    impl<'info> IxOps<'info> for UpdateAmmConfig {
        type IxData = raydium_cp_swap::instruction::UpdateAmmConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateAmmConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::UpdateAmmConfig {
                param: todo!(),
                value: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::UpdateAmmConfig {
                owner: todo!(),
                amm_config: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpdatePoolStatus {
        type IxData = raydium_cp_swap::instruction::UpdatePoolStatus;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdatePoolStatusSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::UpdatePoolStatus { status: todo!() };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::UpdatePoolStatus {
                authority: todo!(),
                pool_state: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CollectProtocolFee {
        type IxData = raydium_cp_swap::instruction::CollectProtocolFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CollectProtocolFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CollectProtocolFee {
                amount_0_requested: todo!(),
                amount_1_requested: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CollectProtocolFee {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                amm_config: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                recipient_token_0_account: todo!(),
                recipient_token_1_account: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CollectFundFee {
        type IxData = raydium_cp_swap::instruction::CollectFundFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CollectFundFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CollectFundFee {
                amount_0_requested: todo!(),
                amount_1_requested: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CollectFundFee {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                amm_config: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                recipient_token_0_account: todo!(),
                recipient_token_1_account: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }*/
    impl<'info> IxOps<'info> for Initialize {
        type IxData = raydium_cp_swap::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Initialize {
                init_amount_0: self.data.init_amount_0,
                init_amount_1: self.data.init_amount_1,
                open_time:self.data.open_time,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {

            let creator = fuzz_accounts.creator.get_or_create_account(
                100,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            
            let index: u16 = 0;
            let (amm_config_pda, bump) = Pubkey::find_program_address(
                &[b"amm_config", &index.to_le_bytes()],
                &raydium_cp_swap::ID,
            );

         
            let amm_config_data = raydium_cp_swap::states::AmmConfig {
                bump,
                disable_create_pool: false,
                index: 0,
                trade_fee_rate: 10, 
                protocol_fee_rate: 1000, 
                fund_fee_rate: 25000, 
                create_pool_fee: 0, 
                protocol_owner: creator.pubkey(), 
                fund_owner: creator.pubkey(), 
                padding: [0; 16],
            };

            
            let mut data: Vec<u8> = vec![];
        

            amm_config_data.try_serialize(&mut data).unwrap();

            client.set_account_custom(
                &amm_config_pda, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );
            /*    //If PDA was initialized we might just get() it.
            let amm_config_pda_acct = match fuzz_accounts.amm_config.get(self.accounts.amm_config).clone() {
                Some(amm_config_pda) => amm_config_pda.pubkey(),
                None => Pubkey::default(),
            };
            */
            let (authority, bump ) = Pubkey::find_program_address(
                &[b"vault_and_lp_mint_auth_seed"],
                &raydium_cp_swap::ID,
            );
      

            let min_auth = Pubkey::from_str("DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8").unwrap();
            

            let token_0_mint = fuzz_accounts.token_0_mint.get_or_create_account(
                self.accounts.token_0_mint, 
                client, 
                u8::MAX - 1, 
                &min_auth, 
                None,
            ).unwrap();
           
            let token_1_mint = fuzz_accounts.token_1_mint.get_or_create_account(
                self.accounts.token_1_mint, 
                client, 
                0, 
                &min_auth, 
                None,
            ).unwrap();
    
            let pool_fee = client.set_token_account(
                token_0_mint.key(), 
                creator.pubkey(), 
                1000, 
                None, 
                None, 
                0,
                 None
                );

            let (pool_address, _) = Pubkey::find_program_address(
                &[
                    b"pool", 
                    amm_config_pda.as_ref(), 
                    token_0_mint.as_ref(), 
                    token_1_mint.as_ref() 
                    ],
                &raydium_cp_swap::ID,
            );

            
            let (lp_mint, _) = Pubkey::find_program_address(
                &[
                    b"pool_lp_mint",
                    pool_address.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );


            let (pool_vault_address, _) = Pubkey::find_program_address(
                &[
                    b"pool_vault", 
                    pool_address.as_ref(), 
                    spl_token::ID.as_ref(), 
                    token_1_mint.as_ref() 
                    ],
                &raydium_cp_swap::ID,
            );


     
            let (creator_lp_token,_)= Pubkey::find_program_address(
                &[
                    creator.pubkey().as_ref(), 
                    &spl_token::ID.as_ref(), 
                    lp_mint.as_ref() 
                    ],
                &anchor_spl::associated_token::ID,
            );
         
            let (token_0_vault,_) = Pubkey::find_program_address(
                &[
                    b"pool_vault", 
                    pool_address.as_ref(), 
                    token_0_mint.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );
            
            let (token_1_vault,_) = Pubkey::find_program_address(
                &[
                    b"pool_vault", 
                    pool_address.as_ref(), 
                    token_1_mint.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );

            
            let (observation_state,_)= Pubkey::find_program_address(
                &[
                    b"observation", 
                    pool_address.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );


            let creator_token_0 = client.set_token_account(
                token_0_mint.key(), 
                creator.pubkey(), 
                u64::MAX, 
                None, 
                None, 
                0,
                 None
                );
            let creator_token_1 = client.set_token_account(
                token_1_mint.key(), 
                creator.pubkey(), 
                u64::MAX, 
                None, 
                None, 
                0,
                    None
                );

            let signers = vec![creator.clone()];
            let acc_meta = raydium_cp_swap::accounts::Initialize {
                creator: creator.pubkey(),
                amm_config: amm_config_pda,
                authority: authority,
                pool_state: pool_address,
                token_0_mint: token_0_mint,
                token_1_mint: token_1_mint,
                lp_mint: lp_mint,
                creator_token_0: creator_token_0,
                creator_token_1: creator_token_1,
                creator_lp_token: creator_lp_token,
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                create_pool_fee: pool_fee,
                observation_state: observation_state,
                token_program: anchor_spl::token::ID,
                token_0_program: anchor_spl::token::ID,
                token_1_program: anchor_spl::token::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                system_program: system_program::ID,
                rent: solana_sdk::sysvar::rent::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
            
        }

        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {

            if pre_ix.token_0_vault.data.borrow().len() == 0 {
                println!("token_0_vault doesn't exist before instruction");
            }
            // Post-instruction check
            // Now we expect it to be a TokenAccount
            let post_token_0_vault = anchor_spl::token::TokenAccount::try_deserialize(&mut &post_ix.token_0_vault.data.borrow()[..])
            .map_err(|_| FuzzingError::Custom(420))?;


            // Repeat for token_1_vault
            let post_token_1_vault = anchor_spl::token::TokenAccount::try_deserialize(&mut &post_ix.token_1_vault.data.borrow()[..])
                .map_err(|_| FuzzingError::Custom(422))?;

            if post_token_0_vault.amount == 0  {
                eprintln!("Token vault 0 {}, Token vault 1 {}",post_token_0_vault.amount, post_token_1_vault.amount);
                return Err(FuzzingError::Custom(422));
            }
            else{
                Ok(())
            }

   // Ok(())
        }

    }
    
    


    
    impl<'info> IxOps<'info> for Deposit {
        type IxData = raydium_cp_swap::instruction::Deposit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DepositSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Deposit {
                lp_token_amount: 10000000000,
                maximum_token_0_amount: 10000000000,
                maximum_token_1_amount: 20000000000,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {



            let owner = fuzz_accounts.creator.get_or_create_account(
                100,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            
            let index: u16 = 0;
            let (amm_config_pda, bump) = Pubkey::find_program_address(
                &[b"amm_config", &index.to_le_bytes()],
                &raydium_cp_swap::ID,
            );

         
            let amm_config_data = raydium_cp_swap::states::AmmConfig {
                bump,
                disable_create_pool: false,
                index: 0,
                trade_fee_rate: 10, 
                protocol_fee_rate: 1000, 
                fund_fee_rate: 25000, 
                create_pool_fee: 0, 
                protocol_owner: owner.pubkey(), 
                fund_owner: owner.pubkey(), 
                padding: [0; 16],
            };

            
            let mut data: Vec<u8> = vec![];
       
            amm_config_data.try_serialize(&mut data).unwrap();

            client.set_account_custom(
                &amm_config_pda, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );
            let (authority, bump ) = Pubkey::find_program_address(
                &[b"vault_and_lp_mint_auth_seed"],
                &raydium_cp_swap::ID,
            );

//token_0_account
            let token_0_mint = fuzz_accounts.token_0_mint.get_or_create_account(
                9, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();

//token_1_account
            let token_1_mint = fuzz_accounts.token_1_mint.get_or_create_account(
                10, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();



            let (pool_address, _) = Pubkey::find_program_address(
                &[
                    b"pool", 
                    amm_config_pda.as_ref(), 
                    token_0_mint.as_ref(), 
                    token_1_mint.as_ref() 
                    ],
                &raydium_cp_swap::ID,
            );

            let token_0_vault = client.set_token_account(
                token_0_mint.key(),
                authority,//owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );


            let token_1_vault = client.set_token_account(
                token_1_mint.key(),
                authority, 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );

            let token_0_account = client.set_token_account(
                token_0_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            let token_1_account = client.set_token_account(
                token_1_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            

            let lp_mint = Pubkey::new_unique();

            let m = spl_token::state::Mint {
                mint_authority: Some(authority).into(),
                supply: 1000000000000000, // Set this to whatever initial supply you want
                decimals: 9,
                is_initialized: true,
                freeze_authority: None.into(),
            };
  
            
            let mut data = vec![0; spl_token::state::Mint::LEN];
            spl_token::state::Mint::pack(m, &mut data);
      
            client.set_account_custom(
                &lp_mint,
                &AccountSharedData::create(
                    100,
                    data,
                    anchor_spl::token::ID,
                    false,
                    0
                )
            );


            let (observation_state,_)= Pubkey::find_program_address(
                &[
                    b"observation", 
                    pool_address.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );


            let pool_state = raydium_cp_swap::states::pool::PoolState {
                amm_config: amm_config_pda,
                pool_creator: owner.pubkey(),
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                lp_mint: lp_mint,
                token_0_mint: token_0_mint,
                token_1_mint: token_1_mint,
                token_0_program: anchor_spl::token::ID,
                token_1_program: anchor_spl::token::ID,
                observation_key: observation_state,
                auth_bump: bump,
                status: 0,
                lp_mint_decimals: 9,
                mint_0_decimals: 9,
                mint_1_decimals: 9,
                lp_supply: 1000000000000000,
                protocol_fees_token_0: 0,
                protocol_fees_token_1: 0,
                fund_fees_token_0: 0,
                fund_fees_token_1: 0,
                open_time: 1726261579,
                recent_epoch: 0,
                padding: [0; 31], 

            };
            let mut data = Vec::new();
            let discriminator = raydium_cp_swap::states::pool::PoolState::discriminator();
            data.extend_from_slice(&discriminator);
            // Get a raw pointer to the struct
            let ptr = &pool_state as *const _ as *const u8;

            // Iterate over each byte of the struct
            for i in 0..mem::size_of::<raydium_cp_swap::states::pool::PoolState>() {
                unsafe {
                    data.push(*ptr.add(i));
                }
            };
            

            client.set_account_custom(
                &pool_address, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );

            let owner_lp_token = client.set_token_account(
                lp_mint, 
                owner.pubkey(), 
                100000000000, 
                None, 
                None, 
                0, 
                None
            );


            let signers = vec![owner.clone()];
            let acc_meta = raydium_cp_swap::accounts::Deposit {
                owner: owner.pubkey(),
                authority: authority,
                pool_state: pool_address,
                owner_lp_token: owner_lp_token,
                token_0_account: token_0_account,
                token_1_account: token_1_account,
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                token_program: anchor_spl::token::ID,
                token_program_2022: anchor_spl::token_2022::ID,
                vault_0_mint: token_0_mint,
                vault_1_mint: token_1_mint,
                lp_mint: lp_mint,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Withdraw {
        type IxData = raydium_cp_swap::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Withdraw {
                lp_token_amount: 10000000000,//self.data.lp_token_amount,
                minimum_token_0_amount: 10000000,
                minimum_token_1_amount: 1000000,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {


            let owner = fuzz_accounts.creator.get_or_create_account(
                100,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            
            let index: u16 = 0;
            let (amm_config_pda, bump) = Pubkey::find_program_address(
                &[b"amm_config", &index.to_le_bytes()],
                &raydium_cp_swap::ID,
            );

         
            let amm_config_data = raydium_cp_swap::states::AmmConfig {
                bump,
                disable_create_pool: false,
                index: 0,
                trade_fee_rate: 10, 
                protocol_fee_rate: 1000, 
                fund_fee_rate: 25000, 
                create_pool_fee: 0, 
                protocol_owner: owner.pubkey(), 
                fund_owner: owner.pubkey(), 
                padding: [0; 16],
            };

            
            let mut data: Vec<u8> = vec![];
       
            amm_config_data.try_serialize(&mut data).unwrap();

            client.set_account_custom(
                &amm_config_pda, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );
            let (authority, bump ) = Pubkey::find_program_address(
                &[b"vault_and_lp_mint_auth_seed"],
                &raydium_cp_swap::ID,
            );

//token_0_account
            let token_0_mint = fuzz_accounts.token_0_mint.get_or_create_account(
                9, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();

//token_1_account
            let token_1_mint = fuzz_accounts.token_1_mint.get_or_create_account(
                10, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();



            let (pool_address, _) = Pubkey::find_program_address(
                &[
                    b"pool", 
                    amm_config_pda.as_ref(), 
                    token_0_mint.as_ref(), 
                    token_1_mint.as_ref() 
                    ],
                &raydium_cp_swap::ID,
            );

            let token_0_vault = client.set_token_account(
                token_0_mint.key(),
                authority,//owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );


            let token_1_vault = client.set_token_account(
                token_1_mint.key(),
                authority, 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );

            let token_0_account = client.set_token_account(
                token_0_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            let token_1_account = client.set_token_account(
                token_1_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            

            let lp_mint = Pubkey::new_unique();

            let m = spl_token::state::Mint {
                mint_authority: Some(authority).into(),
                supply: 1000000000000000,//self.data.lp_token_amount, // Set this to whatever initial supply you want
                decimals: 9,
                is_initialized: true,
                freeze_authority: None.into(),
            };
  
            
            let mut data = vec![0; spl_token::state::Mint::LEN];
            spl_token::state::Mint::pack(m, &mut data);
      
            client.set_account_custom(
                &lp_mint,
                &AccountSharedData::create(
                    100,
                    data,
                    anchor_spl::token::ID,
                    false,
                    0
                )
            );


            let (observation_state,_)= Pubkey::find_program_address(
                &[
                    b"observation", 
                    pool_address.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );


            let pool_state = raydium_cp_swap::states::pool::PoolState {
                amm_config: amm_config_pda,
                pool_creator: owner.pubkey(),
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                lp_mint: lp_mint,
                token_0_mint: token_0_mint,
                token_1_mint: token_0_mint,
                token_0_program: anchor_spl::token::ID,
                token_1_program: anchor_spl::token::ID,
                observation_key: observation_state,
                auth_bump: bump,
                status: 0,
                lp_mint_decimals: 9,
                mint_0_decimals: 9,
                mint_1_decimals: 9,
                lp_supply: 1000000000000000,//self.data.lp_token_amount,
                protocol_fees_token_0: 0,
                protocol_fees_token_1: 0,
                fund_fees_token_0: 0,
                fund_fees_token_1: 0,
                open_time: 1726261579,
                recent_epoch: 0,
                padding: [0; 31], 

            };
            let mut data = Vec::new();
            let discriminator = raydium_cp_swap::states::pool::PoolState::discriminator();
            data.extend_from_slice(&discriminator);
            // Get a raw pointer to the struct
            let ptr = &pool_state as *const _ as *const u8;

            // Iterate over each byte of the struct
            for i in 0..mem::size_of::<raydium_cp_swap::states::pool::PoolState>() {
                unsafe {
                    data.push(*ptr.add(i));
                }
            };
            

            client.set_account_custom(
                &pool_address, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );

            let owner_lp_token = client.set_token_account(
                lp_mint, 
                owner.pubkey(), 
                100000000000, 
                None, 
                None, 
                0, 
                None
            );



            let signers = vec![owner.clone()];
            let acc_meta = raydium_cp_swap::accounts::Withdraw {
                owner: owner.pubkey(),
                authority: authority,
                pool_state: pool_address,
                owner_lp_token: owner_lp_token,
                token_0_account: token_0_account,
                token_1_account: token_1_account,
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                token_program: anchor_spl::token::ID,
                token_program_2022: anchor_spl::token_2022::ID,
                vault_0_mint: token_0_mint,
                vault_1_mint: token_1_mint,
                lp_mint: lp_mint,
                memo_program: spl_memo::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SwapBaseInput {
        type IxData = raydium_cp_swap::instruction::SwapBaseInput;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapBaseInputSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::SwapBaseInput {
                amount_in: 100000000,
                minimum_amount_out: 100,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {



            let owner = fuzz_accounts.creator.get_or_create_account(
                100,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            
            let index: u16 = 0;
            let (amm_config_pda, bump) = Pubkey::find_program_address(
                &[b"amm_config", &index.to_le_bytes()],
                &raydium_cp_swap::ID,
            );

         
            let amm_config_data = raydium_cp_swap::states::AmmConfig {
                bump,
                disable_create_pool: false,
                index: 0,
                trade_fee_rate: 10, 
                protocol_fee_rate: 1000, 
                fund_fee_rate: 25000, 
                create_pool_fee: 0, 
                protocol_owner: owner.pubkey(), 
                fund_owner: owner.pubkey(), 
                padding: [0; 16],
            };

            
            let mut data: Vec<u8> = vec![];
       
            amm_config_data.try_serialize(&mut data).unwrap();

            client.set_account_custom(
                &amm_config_pda, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );
            let (authority, bump ) = Pubkey::find_program_address(
                &[b"vault_and_lp_mint_auth_seed"],
                &raydium_cp_swap::ID,
            );

//token_0_account
            let token_0_mint = fuzz_accounts.token_0_mint.get_or_create_account(
                9, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();

//token_1_account
            let token_1_mint = fuzz_accounts.token_1_mint.get_or_create_account(
                10, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();



            let (pool_address, _) = Pubkey::find_program_address(
                &[
                    b"pool", 
                    amm_config_pda.as_ref(), 
                    token_0_mint.as_ref(), 
                    token_1_mint.as_ref() 
                    ],
                &raydium_cp_swap::ID,
            );

            let token_0_vault = client.set_token_account(
                token_0_mint.key(),
                authority,//owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );


            let token_1_vault = client.set_token_account(
                token_1_mint.key(),
                authority, 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );

            let token_0_account = client.set_token_account(
                token_0_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            let token_1_account = client.set_token_account(
                token_1_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            

            let lp_mint = Pubkey::new_unique();

            let m = spl_token::state::Mint {
                mint_authority: Some(authority).into(),
                supply: 1000000000000000, // Set this to whatever initial supply you want
                decimals: 9,
                is_initialized: true,
                freeze_authority: None.into(),
            };
  
            
            let mut data = vec![0; spl_token::state::Mint::LEN];
            spl_token::state::Mint::pack(m, &mut data);
      
            client.set_account_custom(
                &lp_mint,
                &AccountSharedData::create(
                    100,
                    data,
                    anchor_spl::token::ID,
                    false,
                    0
                )
            );


            let (observation_state,_)= Pubkey::find_program_address(
                &[
                    b"observation", 
                    pool_address.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );

            let observation_struct = raydium_cp_swap::states::oracle::Observation {
                block_timestamp: timestamp(),
                cumulative_token_0_price_x32: 10,
                cumulative_token_1_price_x32: 10
            };
    
            let obs = raydium_cp_swap::states::oracle::ObservationState{
                initialized: true,
                observation_index: 0,
                pool_id: Pubkey::default(),
                observations: [observation_struct; 100],
                padding: [0; 4]
            };
            let mut data = Vec::new();
            let discriminator = raydium_cp_swap::states::oracle::ObservationState::discriminator();
            data.extend_from_slice(&discriminator);

            let ptr = &obs as *const _ as *const u8;

            // Iterate over each byte of the struct
            for i in 0..mem::size_of::<raydium_cp_swap::states::oracle::ObservationState>() {
                unsafe {
                    data.push(*ptr.add(i));
                }
            };
            

            client.set_account_custom(
                &observation_state, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );


            let pool_state = raydium_cp_swap::states::pool::PoolState {
                amm_config: amm_config_pda,
                pool_creator: owner.pubkey(),
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                lp_mint: lp_mint,
                token_0_mint: token_0_mint,
                token_1_mint: token_1_mint,
                token_0_program: anchor_spl::token::ID,
                token_1_program: anchor_spl::token::ID,
                observation_key: observation_state,
                auth_bump: bump,
                status: 0,
                lp_mint_decimals: 9,
                mint_0_decimals: 9,
                mint_1_decimals: 9,
                lp_supply: 1000000000000000,
                protocol_fees_token_0: 0,
                protocol_fees_token_1: 0,
                fund_fees_token_0: 0,
                fund_fees_token_1: 0,
                open_time: 1726261579,
                recent_epoch: 0,
                padding: [0; 31], 

            };
            let mut data = Vec::new();
            let discriminator = raydium_cp_swap::states::pool::PoolState::discriminator();
            data.extend_from_slice(&discriminator);
            // Get a raw pointer to the struct
            let ptr = &pool_state as *const _ as *const u8;

            // Iterate over each byte of the struct
            for i in 0..mem::size_of::<raydium_cp_swap::states::pool::PoolState>() {
                unsafe {
                    data.push(*ptr.add(i));
                }
            };
            
            client.set_account_custom(
                &pool_address, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );

            let owner_lp_token = client.set_token_account(
                lp_mint, 
                owner.pubkey(), 
                100000000000, 
                None, 
                None, 
                0, 
                None
            );

            let signers = vec![owner.clone()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: owner.pubkey(),
                authority: authority,
                amm_config: amm_config_pda,
                pool_state: pool_address,
                input_token_account: token_0_account,
                output_token_account: token_1_account,
                input_vault: token_0_vault,
                output_vault: token_1_vault,
                input_token_program: anchor_spl::token::ID,
                output_token_program: anchor_spl::token::ID,
                input_token_mint: token_0_mint,
                output_token_mint: token_1_mint,
                observation_state: observation_state,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SwapBaseOutput {
        type IxData = raydium_cp_swap::instruction::SwapBaseOutput;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapBaseOutputSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::SwapBaseOutput {
                max_amount_in: 10000000000000,
                amount_out: 100000000,
            };
            Ok(data)
        } 
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let owner = fuzz_accounts.creator.get_or_create_account(
                100,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            
            let index: u16 = 0;
            let (amm_config_pda, bump) = Pubkey::find_program_address(
                &[b"amm_config", &index.to_le_bytes()],
                &raydium_cp_swap::ID,
            );

         
            let amm_config_data = raydium_cp_swap::states::AmmConfig {
                bump,
                disable_create_pool: false,
                index: 0,
                trade_fee_rate: 10, 
                protocol_fee_rate: 1000, 
                fund_fee_rate: 25000, 
                create_pool_fee: 0, 
                protocol_owner: owner.pubkey(), 
                fund_owner: owner.pubkey(), 
                padding: [0; 16],
            };

            
            let mut data: Vec<u8> = vec![];
       
            amm_config_data.try_serialize(&mut data).unwrap();

            client.set_account_custom(
                &amm_config_pda, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );
            let (authority, bump ) = Pubkey::find_program_address(
                &[b"vault_and_lp_mint_auth_seed"],
                &raydium_cp_swap::ID,
            );

//token_0_account
            let token_0_mint = fuzz_accounts.token_0_mint.get_or_create_account(
                9, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();

//token_1_account
            let token_1_mint = fuzz_accounts.token_1_mint.get_or_create_account(
                10, 
                client, 
                9, 
                &owner.pubkey(), 
                None,
            ).unwrap();



            let (pool_address, _) = Pubkey::find_program_address(
                &[
                    b"pool", 
                    amm_config_pda.as_ref(), 
                    token_0_mint.as_ref(), 
                    token_1_mint.as_ref() 
                    ],
                &raydium_cp_swap::ID,
            );

            let token_0_vault = client.set_token_account(
                token_0_mint.key(),
                authority,//owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );


            let token_1_vault = client.set_token_account(
                token_1_mint.key(),
                authority, 
                100000000000000, 
                None, 
                None, 
                0, 
                None
            );

            let token_0_account = client.set_token_account(
                token_0_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            let token_1_account = client.set_token_account(
                token_1_mint.key(), 
                owner.pubkey(), 
                100000000000000, 
                None, 
                None, 
                0,
                None
                );

            let lp_mint = Pubkey::new_unique();

            let m = spl_token::state::Mint {
                mint_authority: Some(authority).into(),
                supply: 1000000000000000, // Set this to whatever initial supply you want
                decimals: 9,
                is_initialized: true,
                freeze_authority: None.into(),
            };
  
            
            let mut data = vec![0; spl_token::state::Mint::LEN];
            spl_token::state::Mint::pack(m, &mut data);
      
            client.set_account_custom(
                &lp_mint,
                &AccountSharedData::create(
                    100,
                    data,
                    anchor_spl::token::ID,
                    false,
                    0
                )
            );


            let (observation_state,_)= Pubkey::find_program_address(
                &[
                    b"observation", 
                    pool_address.as_ref(), 
                    ],
                &raydium_cp_swap::ID,
            );

            let observation_struct = raydium_cp_swap::states::oracle::Observation {
                block_timestamp: timestamp(),
                cumulative_token_0_price_x32: 10,
                cumulative_token_1_price_x32: 10
            };
    
            let obs = raydium_cp_swap::states::oracle::ObservationState{
                initialized: true,
                observation_index: 0,
                pool_id: Pubkey::default(),
                observations: [observation_struct; 100],
                padding: [0; 4]
            };
            let mut data = Vec::new();
            let discriminator = raydium_cp_swap::states::oracle::ObservationState::discriminator();
            data.extend_from_slice(&discriminator);

            let ptr = &obs as *const _ as *const u8;

            // Iterate over each byte of the struct
            for i in 0..mem::size_of::<raydium_cp_swap::states::oracle::ObservationState>() {
                unsafe {
                    data.push(*ptr.add(i));
                }
            };
            
            client.set_account_custom(
                &observation_state, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );

            let pool_state = raydium_cp_swap::states::pool::PoolState {
                amm_config: amm_config_pda,
                pool_creator: owner.pubkey(),
                token_0_vault: token_0_vault,
                token_1_vault: token_1_vault,
                lp_mint: lp_mint,
                token_0_mint: token_0_mint,
                token_1_mint: token_1_mint,
                token_0_program: anchor_spl::token::ID,
                token_1_program: anchor_spl::token::ID,
                observation_key: observation_state,
                auth_bump: bump,
                status: 0,
                lp_mint_decimals: 9,
                mint_0_decimals: 9,
                mint_1_decimals: 9,
                lp_supply: 1000000000000000,
                protocol_fees_token_0: 0,
                protocol_fees_token_1: 0,
                fund_fees_token_0: 0,
                fund_fees_token_1: 0,
                open_time: 1726261579,
                recent_epoch: 0,
                padding: [0; 31], 

            };
            let mut data = Vec::new();
            let discriminator = raydium_cp_swap::states::pool::PoolState::discriminator();
            data.extend_from_slice(&discriminator);
            // Get a raw pointer to the struct
            let ptr = &pool_state as *const _ as *const u8;

            // Iterate over each byte of the struct
            for i in 0..mem::size_of::<raydium_cp_swap::states::pool::PoolState>() {
                unsafe {
                    data.push(*ptr.add(i));
                }
            };
            

            client.set_account_custom(
                &pool_address, 
                &AccountSharedData::create(
                    10 * LAMPORTS_PER_SOL, 
                    data, 
                    raydium_cp_swap::ID,
                    false, 
                    0
                )
            );

            let owner_lp_token = client.set_token_account(
                lp_mint, 
                owner.pubkey(), 
                100000000000, 
                None, 
                None, 
                0, 
                None
            );

            let signers = vec![owner.clone()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: owner.pubkey(),
                authority: authority,
                amm_config: amm_config_pda,
                pool_state: pool_address,
                input_token_account: token_0_account,
                output_token_account: token_1_account,
                input_vault: token_0_vault,
                output_vault: token_1_vault,
                input_token_program: anchor_spl::token::ID,
                output_token_program: anchor_spl::token::ID,
                input_token_mint: token_0_mint,
                output_token_mint: token_1_mint,
                observation_state: observation_state,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    } 
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        amm_config: AccountsStorage<PdaStore>,
        //associated_token_program: AccountsStorage<todo!()>,
        //authority: AccountsStorage<todo!()>,
        create_pool_fee: AccountsStorage<TokenStore>,
        creator: AccountsStorage<Keypair>,
        /*creator_lp_token: AccountsStorage<todo!()>,
        //creator_token_0: AccountsStorage<todo!()>,
        //creator_token_1: AccountsStorage<todo!()>,
        //input_token_account: AccountsStorage<todo!()>,
        //input_token_mint: AccountsStorage<todo!()>,
        //input_token_program: AccountsStorage<todo!()>,
        //input_vault: AccountsStorage<todo!()>,
        lp_mint: AccountsStorage<todo!()>,
        memo_program: AccountsStorage<todo!()>,
        observation_state: AccountsStorage<todo!()>,
        output_token_account: AccountsStorage<todo!()>,
        output_token_mint: AccountsStorage<todo!()>,
        output_token_program: AccountsStorage<todo!()>,
        output_vault: AccountsStorage<todo!()>,
        owner: AccountsStorage<todo!()>,
        owner_lp_token: AccountsStorage<todo!()>,
        payer: AccountsStorage<todo!()>,
        pool_state: AccountsStorage<todo!()>,
        recipient_token_0_account: AccountsStorage<todo!()>,
        recipient_token_1_account: AccountsStorage<todo!()>,
        rent: AccountsStorage<todo!()>,
        //system_program: AccountsStorage<todo!()>,
        token_0_account: AccountsStorage<todo!()>,*/
        token_0_mint: AccountsStorage<MintStore>,
        //token_0_program: AccountsStorage<todo!()>,
        //token_0_vault: AccountsStorage<todo!()>,
        //token_1_account: AccountsStorage<todo!()>,
        token_1_mint: AccountsStorage<MintStore>,
        //token_1_program: AccountsStorage<todo!()>,
        //token_1_vault: AccountsStorage<todo!()>,
        //token_program: AccountsStorage<todo!()>,
        //token_program_2022: AccountsStorage<todo!()>,
        //vault_0_mint: AccountsStorage<todo!()>,
        //vault_1_mint: AccountsStorage<todo!()>,
    }
}
