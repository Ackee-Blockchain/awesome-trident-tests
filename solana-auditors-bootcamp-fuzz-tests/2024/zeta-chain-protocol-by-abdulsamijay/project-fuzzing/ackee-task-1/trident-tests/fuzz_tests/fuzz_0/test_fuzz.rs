use fuzz_instructions::solana_project_fuzz_fuzz_instructions::{Withdraw, Deposit, Initialize};
use solana_project_fuzz::entry as entry_solana_project_fuzz;
use solana_project_fuzz::ID as PROGRAM_ID_SOLANA_PROJECT_FUZZ;
const PROGRAM_NAME_SOLANA_PROJECT_FUZZ: &str =  "solana_project_fuzz";
use fuzz_instructions::solana_project_fuzz_fuzz_instructions::FuzzInstruction as FuzzInstruction_solana_project_fuzz;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_solana_project_fuzz;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {

    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        // Generate an `Initialize` instruction to set up the state
        let init = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        Ok(vec![init])
    }

    fn ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let deposit = FuzzInstruction::Deposit(Deposit::arbitrary(u)?);
        let withdraw = FuzzInstruction::Withdraw(Withdraw::arbitrary(u)?);
        Ok(vec![deposit, withdraw])
    }

    fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        // No post-instructions; return an empty vector
        Ok(vec![])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_SOLANA_PROJECT_FUZZ,
                &PROGRAM_ID_SOLANA_PROJECT_FUZZ,
                processor!(convert_entry!(entry_solana_project_fuzz))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_SOLANA_PROJECT_FUZZ, &mut client);
        });
    }
}
