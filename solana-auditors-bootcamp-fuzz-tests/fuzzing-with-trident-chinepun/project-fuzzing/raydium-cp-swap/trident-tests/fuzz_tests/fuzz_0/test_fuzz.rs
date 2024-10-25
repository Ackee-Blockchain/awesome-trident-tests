use fuzz_instructions::raydium_cp_swap_fuzz_instructions::CreateAmmConfig;
use fuzz_instructions::raydium_cp_swap_fuzz_instructions::Initialize;
use fuzz_instructions::raydium_cp_swap_fuzz_instructions::UpdateAmmConfig;
use raydium_cp_swap::entry as entry_raydium_cp_swap;
use raydium_cp_swap::ID as PROGRAM_ID_RAYDIUM_CP_SWAP;
const PROGRAM_NAME_RAYDIUM_CP_SWAP: &str =  "raydium_cp_swap";
use fuzz_instructions::raydium_cp_swap_fuzz_instructions::FuzzInstruction as FuzzInstruction_raydium_cp_swap;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;
mod utils;
use utils::increment_index;

pub type FuzzInstruction = FuzzInstruction_raydium_cp_swap;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData 
{
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        increment_index();
        let init = FuzzInstruction::CreateAmmConfig(CreateAmmConfig::arbitrary(u)?);
        let initialize = FuzzInstruction::Initialize(Initialize::arbitrary(u)?);
        let update_amm_config = FuzzInstruction::UpdateAmmConfig(UpdateAmmConfig::arbitrary(u,)?);
        Ok(vec![init, initialize, update_amm_config])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_RAYDIUM_CP_SWAP, 
                &PROGRAM_ID_RAYDIUM_CP_SWAP, 
                processor!(convert_entry!(entry_raydium_cp_swap))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_RAYDIUM_CP_SWAP, &mut client);
        });
    }
}
