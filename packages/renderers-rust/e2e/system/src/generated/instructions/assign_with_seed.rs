//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct AssignWithSeed {
    pub account: solana_program::pubkey::Pubkey,

    pub base_account: solana_program::pubkey::Pubkey,
}

impl AssignWithSeed {
    pub fn instruction(
        &self,
        args: AssignWithSeedInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AssignWithSeedInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.base_account,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = AssignWithSeedInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SYSTEM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignWithSeedInstructionData {
    discriminator: u32,
}

impl AssignWithSeedInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 10 }
    }
}

impl Default for AssignWithSeedInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignWithSeedInstructionArgs {
    pub base: Pubkey,
    pub seed: String,
    pub program_address: Pubkey,
}

/// Instruction builder for `AssignWithSeed`.
///
/// ### Accounts:
///
///   0. `[writable]` account
///   1. `[signer]` base_account
#[derive(Clone, Debug, Default)]
pub struct AssignWithSeedBuilder {
    account: Option<solana_program::pubkey::Pubkey>,
    base_account: Option<solana_program::pubkey::Pubkey>,
    base: Option<Pubkey>,
    seed: Option<String>,
    program_address: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AssignWithSeedBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn account(&mut self, account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.account = Some(account);
        self
    }
    #[inline(always)]
    pub fn base_account(&mut self, base_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.base_account = Some(base_account);
        self
    }
    #[inline(always)]
    pub fn base(&mut self, base: Pubkey) -> &mut Self {
        self.base = Some(base);
        self
    }
    #[inline(always)]
    pub fn seed(&mut self, seed: String) -> &mut Self {
        self.seed = Some(seed);
        self
    }
    #[inline(always)]
    pub fn program_address(&mut self, program_address: Pubkey) -> &mut Self {
        self.program_address = Some(program_address);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = AssignWithSeed {
            account: self.account.expect("account is not set"),
            base_account: self.base_account.expect("base_account is not set"),
        };
        let args = AssignWithSeedInstructionArgs {
            base: self.base.clone().expect("base is not set"),
            seed: self.seed.clone().expect("seed is not set"),
            program_address: self
                .program_address
                .clone()
                .expect("program_address is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `assign_with_seed` CPI accounts.
pub struct AssignWithSeedCpiAccounts<'a, 'b> {
    pub account: &'b solana_program::account_info::AccountInfo<'a>,

    pub base_account: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `assign_with_seed` CPI instruction.
pub struct AssignWithSeedCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub account: &'b solana_program::account_info::AccountInfo<'a>,

    pub base_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: AssignWithSeedInstructionArgs,
}

impl<'a, 'b> AssignWithSeedCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AssignWithSeedCpiAccounts<'a, 'b>,
        args: AssignWithSeedInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            account: accounts.account,
            base_account: accounts.base_account,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.base_account.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = AssignWithSeedInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SYSTEM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.account.clone());
        account_infos.push(self.base_account.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `AssignWithSeed` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` account
///   1. `[signer]` base_account
#[derive(Clone, Debug)]
pub struct AssignWithSeedCpiBuilder<'a, 'b> {
    instruction: Box<AssignWithSeedCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AssignWithSeedCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AssignWithSeedCpiBuilderInstruction {
            __program: program,
            account: None,
            base_account: None,
            base: None,
            seed: None,
            program_address: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.account = Some(account);
        self
    }
    #[inline(always)]
    pub fn base_account(
        &mut self,
        base_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.base_account = Some(base_account);
        self
    }
    #[inline(always)]
    pub fn base(&mut self, base: Pubkey) -> &mut Self {
        self.instruction.base = Some(base);
        self
    }
    #[inline(always)]
    pub fn seed(&mut self, seed: String) -> &mut Self {
        self.instruction.seed = Some(seed);
        self
    }
    #[inline(always)]
    pub fn program_address(&mut self, program_address: Pubkey) -> &mut Self {
        self.instruction.program_address = Some(program_address);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = AssignWithSeedInstructionArgs {
            base: self.instruction.base.clone().expect("base is not set"),
            seed: self.instruction.seed.clone().expect("seed is not set"),
            program_address: self
                .instruction
                .program_address
                .clone()
                .expect("program_address is not set"),
        };
        let instruction = AssignWithSeedCpi {
            __program: self.instruction.__program,

            account: self.instruction.account.expect("account is not set"),

            base_account: self
                .instruction
                .base_account
                .expect("base_account is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct AssignWithSeedCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base: Option<Pubkey>,
    seed: Option<String>,
    program_address: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
