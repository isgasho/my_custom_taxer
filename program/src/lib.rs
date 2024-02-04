use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{self, rent::Rent, Sysvar},
    clock::Clock,
    instruction::{AccountMeta, Instruction},
    program_pack::{IsInitialized, Pack, Sealed},
    system_instruction,
};
use spl_token::state::Account as TokenAccount;
use solana_program::program::invoke_signed;
use std::convert::TryInto;

entrypoint!(process_instruction);

const TAX_PERCENTAGE: u8 = 5;

#[derive(Debug, Default, PartialEq)]
pub struct Taxes {
    marketing: u8,
    liquidity: u8,
    dev: u8,
}


impl Taxes {
    fn calculate_fee(&self, amount: u64, is_sell: bool) -> u64 {
        // Implement ...
        if is_sell {
            (amount * u64::from(self.marketing + self.dev)) / 100
        } else {
            (amount * u64::from(self.marketing + self.liquidity + self.dev)) / 100
        }
    }

}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    match instruction_data[0] {
        0 => initialize_token(program_id, accounts),
        1 => transfer(program_id, accounts, instruction_data),
        2 => set_buy_taxes(program_id, accounts, instruction_data),
        3 => set_sell_taxes(program_id, accounts, instruction_data),
        4 => swap_for_fees(program_id, accounts, instruction_data),
        5 => add_liquidity(program_id, accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initialize_token(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Implement ....
    Ok(())
}

fn transfer(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Implement ....

    //TODO
    let accounts_iter = &mut accounts.iter();
    let token_account_info = next_account_info(accounts_iter)?;
    let sender_account_info = next_account_info(accounts_iter)?;
    let recipient_account_info = next_account_info(accounts_iter)?;
    let token_program_account_info = next_account_info(accounts_iter)?;
    let system_program_account_info = next_account_info(accounts_iter)?;
    let rent_sysvar_account_info = next_account_info(accounts_iter)?;

    // enough balance?
    let sender_token_account = TokenAccount::unpack(&sender_account_info.data.borrow())?;
    let transfer_amount = sender_token_account.amount;
    let tax_amount = (transfer_amount as u64 * TAX_PERCENTAGE as u64) / 100;

    // recipient account exist?
    let recipient_token_account = TokenAccount::unpack(&recipient_account_info.data.borrow())?;

    // transfer(
    //     token_program_account_info.key,
    //     sender_account_info.key,
    //     recipient_account_info.key,
    //     program_id,
    //     &[&sender_account_info.key],
    //     transfer_amount,
    // )?;

    // // Transfer the tax amount
    // transfer(
    //     token_program_account_info.key,
    //     sender_account_info.key,
    //     recipient_account_info.key,
    //     program_id,
    //     &[&sender_account_info.key],
    //     tax_amount.try_into().unwrap(),
    // )?;

    Ok(())
}

fn set_buy_taxes(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Implement ....
    Ok(())
}

fn set_sell_taxes(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Implement ....
    Ok(())
}

fn swap_for_fees(program_id: &Pubkey, accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let token_account = next_account_info(account_iter)?;

    // Extracting authority account
    let authority_account = next_account_info(account_iter)?;

    // Check if the authority is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let contract_balance = token_account.lamports();
    let swap_threshold = 10_000 * 10u64.pow(9); // in lamports

    if contract_balance >= swap_threshold {
        // Implement ...

        // swap_tokens_for_sol(token_account, to_swap)?;
        // add_liquidity(tokens_to_add_liquidity_with, sol_to_add_liquidity_with)?;

        // Implement sending to marketing and dev wallets
        // send_to_marketing(marketing_amt)?;
        // send_to_dev(dev_amt)?;

        msg!("Swapping - ok");
    }

    Ok(())
}

fn manual_swap(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let token_account = next_account_info(account_iter)?;

    let marketing_account = next_account_info(account_iter)?;
    let dev_account = next_account_info(account_iter)?;

    // Implement the logic of manual swap
    let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
    let dev_percentage = u64::from_le_bytes(instruction_data[9..17].try_into().unwrap());
    let marketing_percentage = u64::from_le_bytes(instruction_data[17..25].try_into().unwrap());

    let init_balance = token_account.lamports();
    // Implement your swap logic here
    // swap_tokens_for_sol(token_account, amount)?;

    let new_balance = token_account.lamports() - init_balance;
    if marketing_percentage > 0 {
        // send_to_marketing wallet
        let marketing_amt = new_balance * marketing_percentage / (dev_percentage + marketing_percentage);
        send_to_wallet(marketing_account, marketing_amt)?;
    }

    if dev_percentage > 0 {
        // send_to_dev wallet
        let dev_amt = new_balance * dev_percentage / (dev_percentage + marketing_percentage);
        send_to_wallet(dev_account, dev_amt)?;
    }

    Ok(())
}

fn send_to_wallet(wallet_account: &AccountInfo, amount: u64) -> ProgramResult {
    let lamports = amount;

    // Create a system transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &wallet_account.key,
        &solana_program::system_program::ID,
        lamports,
    );

    solana_program::program::invoke(
        &transfer_instruction,
        &[wallet_account.clone()],
    )?;

    msg!("Sent to wallet {}: {}", wallet_account.key, lamports);
    Ok(())
}

fn swap_tokens_for_sol(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let token_account = next_account_info(account_iter)?;

    // NOTE: amount is passed as an argument in instruction_data
    let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());

    // NOTE: DEX account is passed as an argument in instruction_data
    let dex_account = next_account_info(account_iter)?;

    // NOTE: user account is passed as an argument in instruction_data
    let user_account = next_account_info(account_iter)?;

    let approve_ix = spl_token::instruction::approve(
        &spl_token::id(),
        &token_account.key,
        &dex_account.key,
        &program_id,
        &[],
        amount,
    )?;

    // Swap tokens for SOL
    let swap_ix = solana_program::instruction::Instruction {
        program_id: *dex_account.owner,
        accounts: vec![
            AccountMeta::new_readonly(*token_account.key, false),
            AccountMeta::new_readonly(*dex_account.key, false),
            AccountMeta::new(*user_account.key, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            // ...other accounts...
        ],
        data: Vec::new(),
    };

    // Invoke the token approval and swap instructions
    solana_program::program::invoke_signed(
        &approve_ix,
        &[token_account.clone()],
        &[],
    )?;

    solana_program::program::invoke_signed(
        &swap_ix,
        &[token_account.clone(), dex_account.clone(), user_account.clone()],
        &[],
    )?;

    // For example, print a message indicating that tokens are swapped for SOL
    msg!("Tokens swapped for SOL: {}", amount);

    Ok(())
}

fn add_liquidity(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let token_account = next_account_info(account_iter)?;

    // Assuming token_amount and sol_amount are passed as arguments in instruction_data
    let token_amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
    let sol_amount = u64::from_le_bytes(instruction_data[9..17].try_into().unwrap());

    // Assuming dex_program account is passed as an argument in instruction_data
    let dex_program_account = next_account_info(account_iter)?;

    // Assuming dev_wallet account is passed as an argument in instruction_data
    let dev_wallet_account = next_account_info(account_iter)?;

    // Approve the DEX to spend the tokens
    let approve_ix = spl_token::instruction::approve(
        &spl_token::id(),
        &token_account.key,
        &dex_program_account.key,
        &program_id,
        &[],
        token_amount,
    )?;

    // Add liquidity to the DEX
    let add_liquidity_ix = solana_program::instruction::Instruction {
        program_id: *dex_program_account.owner,
        accounts: vec![
            AccountMeta::new(*token_account.key, false),
            AccountMeta::new(*dex_program_account.key, false),
            AccountMeta::new_readonly(*dev_wallet_account.key, false),
            // ... Add other accounts as needed
        ],
        data: Vec::new(),
    };

    // Invoke the token approval and add liquidity instructions
    solana_program::program::invoke_signed(
        &approve_ix,
        &[token_account.clone(), dex_program_account.clone()],
        &[],
    )?;

    solana_program::program::invoke_signed(
        &add_liquidity_ix,
        &[token_account.clone(), dex_program_account.clone(), dev_wallet_account.clone()],
        &[],
    )?;

    msg!("Liquidity added: Token Amount: {}, SOL Amount: {}", token_amount, sol_amount);

    Ok(())
}