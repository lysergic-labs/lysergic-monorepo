use {
    crate::{
        error::YieldTokenizerError,
        get_principal_token_address, get_yield_token_address, get_yield_tokenizer_address,
        instruction::{Expiry, YieldTokenizerInstruction},
        state::{YieldTokenizerState, LSD_LEN},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh1::try_from_slice_unchecked,
        clock,
        entrypoint::ProgramResult,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction, system_program,
        sysvar::{rent, Sysvar},
    },
    spl_associated_token_account::get_associated_token_address,
    spl_token,
};

struct YieldTokenizer {}

impl YieldTokenizer {
    fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let instruction: YieldTokenizerInstruction = match try_from_slice_unchecked(data) {
            Ok(ix) => ix,
            Err(_) => return Err(ProgramError::InvalidInstructionData),
        };

        match instruction {
            YieldTokenizerInstruction::InitializeYieldTokenizer { expiry } => {
                Self::process_init_yield_tokenizer(program_id, accounts, expiry)
            }
            YieldTokenizerInstruction::TokenizeYield { amount } => {
                Self::process_tokenize_yield(program_id, accounts, amount)
            }
            YieldTokenizerInstruction::Redeem { amount } => {
                Self::process_redeem(program_id, accounts, amount)
            }
            YieldTokenizerInstruction::RedeemFromPt { amount } => {
                Self::process_redeem_from_pt(program_id, accounts, amount)
            }
            YieldTokenizerInstruction::ClaimYield => {
                Self::process_claim_yield(program_id, accounts)
            }
        }
    }

    fn process_init_yield_tokenizer(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        expiry: Expiry,
    ) -> Result<(), ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let authority = next_account_info(accounts_iter)?;
        let yield_tokenizer = next_account_info(accounts_iter)?;
        let lsu_mint = next_account_info(accounts_iter)?;
        let pt_mint = next_account_info(accounts_iter)?;
        let yt_mint = next_account_info(accounts_iter)?;
        let lsu_vault = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let atoken_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let rent = rent::Rent::get()?;
        let clock = clock::Clock::get()?;

        //TODO: Change this to be the maturity date - 0000 UTC
        let dummy_clock_expiry = clock.unix_timestamp as i64;

        // Safety Checks
        if yield_tokenizer.key != &get_yield_tokenizer_address(lsu_mint.key, dummy_clock_expiry) {
            return Err(YieldTokenizerError::InvalidYieldTokenizerAddress.into());
        }
        if pt_mint.key
            != &get_principal_token_address(yield_tokenizer.key, lsu_mint.key, dummy_clock_expiry)
        {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if yt_mint.key
            != &get_yield_token_address(yield_tokenizer.key, lsu_mint.key, dummy_clock_expiry)
        {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if lsu_vault.key != &get_associated_token_address(yield_tokenizer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::LSUTokenAccountMismatch.into());
        }
        if token_program.key != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if atoken_program.key != &spl_associated_token_account::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if system_program.key != &system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let (yield_tokenizer_addr, bump) = Pubkey::find_program_address(
            &[
                crate::LSD_SEED,
                lsu_mint.key.as_ref(),
                &dummy_clock_expiry.to_le_bytes(),
            ],
            program_id,
        );

        if yield_tokenizer.owner != program_id {
            let size = LSD_LEN;
            let required_lamports = rent
                .minimum_balance(size)
                .max(1)
                .saturating_sub(yield_tokenizer.lamports());

            invoke(
                &system_instruction::transfer(
                    authority.key,
                    &yield_tokenizer.key,
                    required_lamports,
                ),
                &[
                    authority.clone(),
                    yield_tokenizer.clone(),
                    system_program.clone(),
                ],
            )?;

            invoke_signed(
                &system_instruction::allocate(&yield_tokenizer.key, size as u64),
                &[yield_tokenizer.clone(), system_program.clone()],
                &[&[
                    crate::LSD_SEED,
                    lsu_mint.key.as_ref(),
                    &dummy_clock_expiry.to_le_bytes(),
                    &[bump],
                ]],
            )?;

            invoke_signed(
                &system_instruction::assign(&yield_tokenizer.key, program_id),
                &[yield_tokenizer.clone(), system_program.clone()],
                &[&[
                    crate::LSD_SEED,
                    lsu_mint.key.as_ref(),
                    &dummy_clock_expiry.to_le_bytes(),
                    &[bump],
                ]],
            )?;
        }

        // Check if vault exists
        if lsu_vault.owner != token_program.key {
            invoke(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    authority.key,
                    yield_tokenizer.key,
                    lsu_mint.key,
                    token_program.key,
                ),
                &[
                    authority.clone(),
                    lsu_vault.clone(),
                    yield_tokenizer.clone(),
                    lsu_mint.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    atoken_program.clone(),
                ],
            )?;
        }

        //Mint checks
        if pt_mint.owner != token_program.key {
            invoke(
                &spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    pt_mint.key,
                    yield_tokenizer.key,
                    None,
                    9,
                )?,
                &[
                    yield_tokenizer.clone(),
                    pt_mint.clone(),
                    token_program.clone(),
                ],
            )?;
        }

        if yt_mint.owner != token_program.key {
            invoke(
                &spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    yt_mint.key,
                    yield_tokenizer.key,
                    None,
                    9,
                )?,
                &[
                    yield_tokenizer.clone(),
                    yt_mint.clone(),
                    token_program.clone(),
                ],
            )?;
        }

        let yield_tokenizer_data = YieldTokenizerState {
            pt: *pt_mint.key,
            yt: *yt_mint.key,
            maturity_date: dummy_clock_expiry,
            lsu_mint: *lsu_mint.key,
            lsu_vault: *lsu_vault.key,
        };

        yield_tokenizer_data.serialize(&mut &mut yield_tokenizer.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_tokenize_yield(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> Result<(), ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let buyer = next_account_info(accounts_iter)?;
        let yield_tokenizer = next_account_info(accounts_iter)?;
        let lsu_mint = next_account_info(accounts_iter)?;
        let pt_mint = next_account_info(accounts_iter)?;
        let yt_mint = next_account_info(accounts_iter)?;
        let lsu_vault = next_account_info(accounts_iter)?;
        let buyer_lsu_ata = next_account_info(accounts_iter)?;
        let buyer_pt_ata = next_account_info(accounts_iter)?;
        let buyer_yt_ata = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let atoken_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let clock = clock::Clock::get()?;
        let now = clock.unix_timestamp as i64;

        let yield_tokenizer_data =
            YieldTokenizerState::try_from_slice(&yield_tokenizer.data.borrow())?;

        // Safety checks
        if program_id != &crate::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if yield_tokenizer.key
            != &get_yield_tokenizer_address(lsu_mint.key, yield_tokenizer_data.maturity_date)
            || lsu_vault.key != &yield_tokenizer_data.lsu_vault
        {
            return Err(YieldTokenizerError::InvalidYieldTokenizerAddress.into());
        }
        if lsu_mint.key != &yield_tokenizer_data.lsu_mint {
            return Err(YieldTokenizerError::InvalidLSU.into());
        }
        if lsu_vault.key != &get_associated_token_address(yield_tokenizer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::InvalidVault.into());
        }
        if pt_mint.key != &yield_tokenizer_data.pt {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if yt_mint.key != &yield_tokenizer_data.yt {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if buyer_lsu_ata.key != &get_associated_token_address(buyer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::LSUTokenAccountMismatch.into());
        }
        if buyer_pt_ata.key != &get_associated_token_address(buyer.key, pt_mint.key) {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if buyer_yt_ata.key != &get_associated_token_address(buyer.key, yt_mint.key) {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if token_program.key != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if atoken_program.key != &spl_associated_token_account::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if system_program.key != &system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        //Check if expiry has elapsed
        if now > yield_tokenizer_data.maturity_date {
            return Err(YieldTokenizerError::Expired.into());
        }

        // Get value of PT token at current timestamp
        // Get fixed yield of PT at current timestamp

        // Get value of YT token at current timestamp
        // Get implied yield of YT at current timestamp
        // Get average yield of underlying at current timestamp

        // Deposit LSU amount into LSU token vault

        // Mint corresponding PT

        // Mint corresponding YT

        // Transfer PT to buyer

        // Transfer YT to buyer

        // Update yield tokenizer state

        yield_tokenizer_data.serialize(&mut &mut yield_tokenizer.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_redeem(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> Result<(), ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let redeemer = next_account_info(accounts_iter)?;
        let yield_tokenizer = next_account_info(accounts_iter)?;
        let lsu_mint = next_account_info(accounts_iter)?;
        let pt_mint = next_account_info(accounts_iter)?;
        let yt_mint = next_account_info(accounts_iter)?;
        let lsu_vault = next_account_info(accounts_iter)?;
        let redeemer_lsu_ata = next_account_info(accounts_iter)?;
        let redeemer_pt_ata = next_account_info(accounts_iter)?;
        let redeemer_yt_ata = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let atoken_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let clock = clock::Clock::get()?;

        let yield_tokenizer_data =
            YieldTokenizerState::try_from_slice(&yield_tokenizer.data.borrow())?;

        if program_id != &crate::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if yield_tokenizer.key
            != &get_yield_tokenizer_address(
                &yield_tokenizer_data.lsu_mint,
                yield_tokenizer_data.maturity_date,
            )
            || lsu_vault.key != &yield_tokenizer_data.lsu_vault
        {
            return Err(YieldTokenizerError::InvalidYieldTokenizerAddress.into());
        }
        if lsu_mint.key != &yield_tokenizer_data.lsu_mint {
            return Err(YieldTokenizerError::InvalidLSU.into());
        }
        if lsu_vault.key != &get_associated_token_address(yield_tokenizer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::InvalidVault.into());
        }
        if pt_mint.key != &yield_tokenizer_data.pt {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if yt_mint.key != &yield_tokenizer_data.yt {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if redeemer_lsu_ata.key != &get_associated_token_address(redeemer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::LSUTokenAccountMismatch.into());
        }
        if redeemer_pt_ata.key != &get_associated_token_address(redeemer.key, pt_mint.key) {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if redeemer_yt_ata.key != &get_associated_token_address(redeemer.key, yt_mint.key) {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if token_program.key != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if atoken_program.key != &spl_associated_token_account::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if system_program.key != &system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        yield_tokenizer_data.serialize(&mut &mut yield_tokenizer.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_redeem_from_pt(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> Result<(), ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let redeemer = next_account_info(accounts_iter)?;
        let yield_tokenizer = next_account_info(accounts_iter)?;
        let lsu_mint = next_account_info(accounts_iter)?;
        let pt_mint = next_account_info(accounts_iter)?;
        let lsu_vault = next_account_info(accounts_iter)?;
        let redeemer_lsu_ata = next_account_info(accounts_iter)?;
        let redeemer_pt_ata = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let atoken_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let clock = clock::Clock::get()?;

        let yield_tokenizer_data =
            YieldTokenizerState::try_from_slice(&yield_tokenizer.data.borrow())?;

        if program_id != &crate::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if yield_tokenizer.key
            != &get_yield_tokenizer_address(lsu_mint.key, yield_tokenizer_data.maturity_date)
            || lsu_vault.key != &yield_tokenizer_data.lsu_vault
        {
            return Err(YieldTokenizerError::InvalidYieldTokenizerAddress.into());
        }
        if lsu_mint.key != &yield_tokenizer_data.lsu_mint {
            return Err(YieldTokenizerError::InvalidLSU.into());
        }
        if lsu_vault.key != &get_associated_token_address(yield_tokenizer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::InvalidVault.into());
        }
        if pt_mint.key != &yield_tokenizer_data.pt {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if redeemer_lsu_ata.key != &get_associated_token_address(redeemer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::LSUTokenAccountMismatch.into());
        }
        if redeemer_pt_ata.key != &get_associated_token_address(redeemer.key, pt_mint.key) {
            return Err(YieldTokenizerError::InvalidPrincipalToken.into());
        }
        if token_program.key != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if atoken_program.key != &spl_associated_token_account::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if system_program.key != &system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        // Check if expiry has elapsed

        // Send PT to program
        // Program burns PT

        // Program sends LSU to redeemer

        // Update program state
        yield_tokenizer_data.serialize(&mut &mut yield_tokenizer.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_claim_yield(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> Result<(), ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let claimer = next_account_info(accounts_iter)?;
        let yield_tokenizer = next_account_info(accounts_iter)?;
        let lsu_mint = next_account_info(accounts_iter)?;
        let yt_mint = next_account_info(accounts_iter)?;
        let lsu_vault = next_account_info(accounts_iter)?;
        let claimer_lsu_ata = next_account_info(accounts_iter)?;
        let claimer_yt_ata = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let atoken_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let clock = clock::Clock::get()?;

        let yield_tokenizer_data =
            YieldTokenizerState::try_from_slice(&yield_tokenizer.data.borrow())?;

        if program_id != &crate::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if yield_tokenizer.key
            != &get_yield_tokenizer_address(lsu_mint.key, yield_tokenizer_data.maturity_date)
            || lsu_vault.key != &yield_tokenizer_data.lsu_vault
        {
            return Err(YieldTokenizerError::InvalidYieldTokenizerAddress.into());
        }
        if lsu_mint.key != &yield_tokenizer_data.lsu_mint {
            return Err(YieldTokenizerError::InvalidLSU.into());
        }
        if lsu_vault.key != &get_associated_token_address(yield_tokenizer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::InvalidVault.into());
        }
        if yt_mint.key != &yield_tokenizer_data.yt {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if claimer_lsu_ata.key != &get_associated_token_address(claimer.key, lsu_mint.key) {
            return Err(YieldTokenizerError::LSUTokenAccountMismatch.into());
        }
        if claimer_yt_ata.key != &get_associated_token_address(claimer.key, yt_mint.key) {
            return Err(YieldTokenizerError::InvalidYieldToken.into());
        }
        if token_program.key != &spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if atoken_program.key != &spl_associated_token_account::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        if system_program.key != &system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        // Check if expiry has elapsed

        // Get accrued yield

        // Send YT to program

        // Program burns YT

        // Program sends LSU to claimer

        // Update program state

        Ok(())
    }

    fn validate_lsu() -> Result<(), ProgramError> {
        unimplemented!();
    }

    fn calc_yield_owed() -> Result<(), ProgramError> {
        unimplemented!();
    }

    fn calc_required_lsu_for_yield_owed() -> Result<(), ProgramError> {
        unimplemented!();
    }

    fn check_maturity() -> Result<(), ProgramError> {
        unimplemented!();
    }

    fn zero_coupon_pricing(
        maturity_value: u64,
        required_interest_rate: f64,
        duration_till_maturity: u64,
    ) -> Result<f64, ProgramError> {
        unimplemented!();
    }
}
