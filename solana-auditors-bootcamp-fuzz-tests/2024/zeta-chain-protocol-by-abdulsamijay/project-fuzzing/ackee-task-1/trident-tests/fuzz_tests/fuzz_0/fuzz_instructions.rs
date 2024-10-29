pub mod solana_project_fuzz_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use secp256k1::{Message, PublicKey, Secp256k1, SecretKey}; // For ECDSA signing
    use sha3::{Digest, Keccak256}; // For Keccak-256 hashing
    use solana_program::keccak::hash;
    use solana_program::secp256k1_recover::secp256k1_recover;
    use solana_sdk::{native_token::LAMPORTS_PER_SOL, nonce};
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Deposit(Deposit),
        Withdraw(Withdraw),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub signer: AccountId,
        pub pda: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub tss_address: [u8; 20],
        pub chain_id: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Deposit {
        pub accounts: DepositAccounts,
        pub data: DepositData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositAccounts {
        pub signer: AccountId,
        pub pda: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Debug)]
    pub struct DepositData {
        pub amount: u64,
        pub memo: Vec<u8>,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawAccounts {
        pub signer: AccountId,
        pub pda: AccountId,
        pub to: AccountId,
    }
    #[derive(Debug)]
    pub struct WithdrawData {
        pub amount: u64,
        pub nonce: u64,
    }

    impl<'a> Arbitrary<'a> for WithdrawData {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let amount = u.int_in_range(1 * LAMPORTS_PER_SOL..=5 * LAMPORTS_PER_SOL)?;

            Ok(WithdrawData {
                amount: amount,
                nonce: 0, // Use the provided nonce as is
            })
        }
    }

    impl<'a> Arbitrary<'a> for DepositData {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let amount = u.int_in_range(1 * LAMPORTS_PER_SOL..=5 * LAMPORTS_PER_SOL)?;
            let memo_length = u.int_in_range(20..=512)?; // Generate a random length between 20 and 512
            let memo: Vec<u8> = (0..memo_length)
                .map(|_| u.arbitrary::<u8>())
                .collect::<Result<_, _>>()?; // Generate Vec<u8> with the specified length
            Ok(DepositData { amount, memo })
        }
    }

    fn derive_tss_address(secret_key: &SecretKey) -> [u8; 20] {
        // Initialize the Secp256k1 context
        let secp = Secp256k1::new();

        // Derive the PublicKey from the SecretKey
        let public_key = PublicKey::from_secret_key(&secp, secret_key);

        // Serialize the public key in uncompressed format (65 bytes)
        let public_key_uncompressed = public_key.serialize_uncompressed();

        // Compute Keccak-256 hash of the public key (skip the first byte to match Ethereum's use)
        let mut hasher = Keccak256::new();
        hasher.update(&public_key_uncompressed[1..]); // Skip the 0x04 prefix

        let hash = hasher.finalize();

        // Extract the last 20 bytes of the Keccak-256 hash for the Ethereum-style address
        let mut tss_address = [0u8; 20];
        tss_address.copy_from_slice(&hash[12..]); // Take the last 20 bytes

        tss_address
    }

    impl<'info> IxOps<'info> for Initialize {
        type IxData = solana_project_fuzz::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let secret_key =
                SecretKey::from_slice(&[0x01; 32]).expect("32 bytes, within curve order");
            let tss_address = derive_tss_address(&secret_key);

            let data = solana_project_fuzz::instruction::Initialize {
                tss_address,
                chain_id: 1u64, // Use the existing chain_id from self.data
            };

            Ok(data)
        }

        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                1 * LAMPORTS_PER_SOL,
            );

            let pda_acc = fuzz_accounts
                .pda
                .get_or_create_account(self.accounts.pda, &[b"meta"], &solana_project_fuzz::ID)
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = solana_project_fuzz::accounts::Initialize {
                signer: signer.pubkey(),
                pda: pda_acc.pubkey,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Deposit {
        type IxData = solana_project_fuzz::instruction::Deposit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DepositSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = solana_project_fuzz::instruction::Deposit {
                amount: self.data.amount,
                memo: self.data.memo.clone(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                self.data.amount,
            );

            let pda_acc = fuzz_accounts
                .pda
                .get_or_create_account(self.accounts.pda, &[b"meta"], &solana_project_fuzz::ID)
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = solana_project_fuzz::accounts::Deposit {
                signer: signer.pubkey(),
                pda: pda_acc.pubkey,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }

    impl<'info> IxOps<'info> for Withdraw {
        type IxData = solana_project_fuzz::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let chain_id = 1u64; // Example chain ID
            let nonce = self.data.nonce;
            let amount = self.data.amount;
            let to = _fuzz_accounts.to.get(
                self.accounts.to, 
            ).unwrap();
            // Create the buffer for hashing
            let mut buffer = Vec::new();
            buffer.extend_from_slice(&chain_id.to_be_bytes());
            buffer.extend_from_slice(&nonce.to_be_bytes());
            buffer.extend_from_slice(&amount.to_be_bytes());
            buffer.extend_from_slice(&to.pubkey().to_bytes());

            // Compute Keccak-256 hash
            let message_hash = hash(&buffer[..]).to_bytes();

            // Sign the message with recovery enabled
            let secp = Secp256k1::new();
            let secret_key =
                SecretKey::from_slice(&[0x01; 32]).expect("32 bytes, within curve order");
            let msg = Message::from_digest_slice(&message_hash).expect("32-byte hash");

            let recoverable_signature: secp256k1::ecdsa::RecoverableSignature = secp.sign_ecdsa_recoverable(&msg, &secret_key);
            let (recovery_id, signature) = recoverable_signature.serialize_compact();

            // Convert the signature to a [u8; 64] array
            let mut signature_array = [0u8; 64];
            signature_array.copy_from_slice(&signature);

            // Convert RecoveryId to u8
            let recovery_id = recovery_id.to_i32() as u8;

            // Create the Withdraw instruction data
            let data = solana_project_fuzz::instruction::Withdraw {
                amount: amount,
                nonce: nonce,
                signature: signature_array,
                recovery_id,
                message_hash,
            };

            Ok(data)
        }

        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                self.data.amount,
            );

            let to_addr =
                fuzz_accounts
                    .to
                    .get_or_create_account(self.accounts.to, client, self.data.amount);

            let pda_acc = fuzz_accounts
                .pda
                .get_or_create_account(self.accounts.pda, &[b"meta"], &solana_project_fuzz::ID)
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = solana_project_fuzz::accounts::Withdraw {
                signer: signer.pubkey(),
                pda: pda_acc.pubkey,
                to: to_addr.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        pda: AccountsStorage<PdaStore>,
        signer: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<todo!()>,
        to: AccountsStorage<Keypair>,
    }
}
