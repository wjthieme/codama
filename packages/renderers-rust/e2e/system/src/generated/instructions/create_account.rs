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
pub struct CreateAccount {
    pub payer: solana_program::pubkey::Pubkey,

    pub new_account: solana_program::pubkey::Pubkey,
}

impl CreateAccount {
    pub fn instruction(
        &self,
        args: CreateAccountInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateAccountInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.new_account,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateAccountInstructionData::new().try_to_vec().unwrap();
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
pub struct CreateAccountInstructionData {
    discriminator: u32,
}

impl CreateAccountInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 0 }
    }
}

impl Default for CreateAccountInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAccountInstructionArgs {
    pub lamports: u64,
    pub space: u64,
    pub program_address: Pubkey,
}

/// Instruction builder for `CreateAccount`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[writable, signer]` new_account
#[derive(Clone, Debug, Default)]
pub struct CreateAccountBuilder {
    payer: Option<solana_program::pubkey::Pubkey>,
    new_account: Option<solana_program::pubkey::Pubkey>,
    lamports: Option<u64>,
    space: Option<u64>,
    program_address: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    #[inline(always)]
    pub fn new_account(&mut self, new_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.new_account = Some(new_account);
        self
    }
    #[inline(always)]
    pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.lamports = Some(lamports);
        self
    }
    #[inline(always)]
    pub fn space(&mut self, space: u64) -> &mut Self {
        self.space = Some(space);
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
        let accounts = CreateAccount {
            payer: self.payer.expect("payer is not set"),
            new_account: self.new_account.expect("new_account is not set"),
        };
        let args = CreateAccountInstructionArgs {
            lamports: self.lamports.clone().expect("lamports is not set"),
            space: self.space.clone().expect("space is not set"),
            program_address: self
                .program_address
                .clone()
                .expect("program_address is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_account` CPI accounts.
pub struct CreateAccountCpiAccounts<'a, 'b> {
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_account: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_account` CPI instruction.
pub struct CreateAccountCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateAccountInstructionArgs,
}

impl<'a, 'b> CreateAccountCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateAccountCpiAccounts<'a, 'b>,
        args: CreateAccountInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            new_account: accounts.new_account,
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
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.new_account.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateAccountInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SYSTEM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.new_account.clone());
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

/// Instruction builder for `CreateAccount` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[writable, signer]` new_account
#[derive(Clone, Debug)]
pub struct CreateAccountCpiBuilder<'a, 'b> {
    instruction: Box<CreateAccountCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateAccountCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateAccountCpiBuilderInstruction {
            __program: program,
            payer: None,
            new_account: None,
            lamports: None,
            space: None,
            program_address: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    #[inline(always)]
    pub fn new_account(
        &mut self,
        new_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.new_account = Some(new_account);
        self
    }
    #[inline(always)]
    pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.instruction.lamports = Some(lamports);
        self
    }
    #[inline(always)]
    pub fn space(&mut self, space: u64) -> &mut Self {
        self.instruction.space = Some(space);
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
        let args = CreateAccountInstructionArgs {
            lamports: self
                .instruction
                .lamports
                .clone()
                .expect("lamports is not set"),
            space: self.instruction.space.clone().expect("space is not set"),
            program_address: self
                .instruction
                .program_address
                .clone()
                .expect("program_address is not set"),
        };
        let instruction = CreateAccountCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer.expect("payer is not set"),

            new_account: self
                .instruction
                .new_account
                .expect("new_account is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateAccountCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lamports: Option<u64>,
    space: Option<u64>,
    program_address: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
