use solana_client::{
    client_error::Result as ClientResult,
    rpc_client::RpcClient,
    system_transaction::transfer,
    transaction::Transaction,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use spl_token::state::Account as TokenAccount;
use std::str::FromStr;
use solana_program::program_pack::Pack;


struct SolanaConfig {
    keypair_path: String,
    // Add more configuration parameters as needed
}

impl SolanaConfig {
    fn new(keypair_path: &str) -> Self {
        SolanaConfig {
            keypair_path: keypair_path.to_string(),
            // Initialize other configuration parameters here
        }
    }
}

fn main() -> ClientResult<()> {
    // Connect to a Solana RPC endpoint
    let rpc_url = "https://api.devnet.solana.com".to_string(); // Change to your desired network
    let rpc_client = RpcClient::new(rpc_url);

    // Replace these with your own account information
    let sender_keypair = Box::new(Keypair::new());
    let recipient_keypair = Box::new(Keypair::new());

    // Fund the sender account with SOL (you can skip this if you already have funded accounts)
    fund_account(&rpc_client, &sender_keypair)?;

    // Create and fund the sender's token account
    let sender_token_account = create_and_fund_token_account(&rpc_client, &sender_keypair)?;

    // Get the recipient's token account
    let recipient_token_account = get_or_create_token_account(&rpc_client, &recipient_keypair)?;

    // Transfer tokens from sender to recipient
    transfer_tokens(
        &rpc_client,
        &sender_keypair,
        &sender_token_account,
        &recipient_token_account,
    )?;

    Ok(())
}

fn fund_account(rpc_client: &RpcClient, keypair: &Box<Keypair>) -> ClientResult<()> {
    rpc_client.request_and_confirm_airdrop(&keypair.pubkey(), 1_000_000)?;

    Ok(())
}

fn create_and_fund_token_account(
    rpc_client: &RpcClient,
    keypair: &Box<Keypair>,
) -> ClientResult<Pubkey> {
    // Replace with the token program ID (use 'solana-tokens' on devnet)
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

    // Create a new token account for the sender
    let sender_token_account = Keypair::new();
    let create_token_account_instruction = spl_token::instruction::initialize_account(
        &spl_token::id(),
        &sender_token_account.pubkey(),
        &token_program_id,
        &keypair.pubkey(),
    )?;
    let create_token_account_tx = Transaction::new_signed_with_payer(
        &[create_token_account_instruction],
        Some(&rpc_client.get_fee_payer()?.pubkey()),
        &[&keypair],
        rpc_client.get_recent_blockhash()?,
    );

    // Send the transaction
    rpc_client.send_and_confirm_transaction(&create_token_account_tx)?;

    // Fund the new token account with some tokens
    let mint_address = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?; // Replace with the actual mint address
    let fund_token_account_instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_address,
        &sender_token_account.pubkey(),
        &keypair.pubkey(),
        &[&keypair.pubkey()],
        1_000_000, // Replace with the amount you want to fund
    )?;
    let fund_token_account_tx = Transaction::new_signed_with_payer(
        &[fund_token_account_instruction],
        Some(&rpc_client.get_fee_payer()?.pubkey()),
        &[&keypair],
        rpc_client.get_recent_blockhash()?,
    );

    // Send the transaction
    rpc_client.send_and_confirm_transaction(&fund_token_account_tx)?;

    Ok(sender_token_account.pubkey())
}

fn get_or_create_token_account(
    rpc_client: &RpcClient,
    keypair: &Box<Keypair>,
) -> ClientResult<Pubkey> {
    // Replace with the token program ID (use 'solana-tokens' on devnet)
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

    // Check if the recipient already has a token account
    let recipient_token_account_address = Keypair::new().pubkey(); // Replace with the recipient's token account address
    let recipient_token_account_info = rpc_client.get_account(&recipient_token_account_address)?;

    if recipient_token_account_info.is_none() {
        // If the recipient does not have a token account, create one
        let recipient_token_account = Keypair::new();
        let create_token_account_instruction = spl_token::instruction::initialize_account(
            &spl_token::id(),
            &recipient_token_account.pubkey(),
            &token_program_id,
            &keypair.pubkey(),
        )?;
        let create_token_account_tx = Transaction::new_signed_with_payer(
            &[create_token_account_instruction],
            Some(&rpc_client.get_fee_payer()?.pubkey()),
            &[&keypair],
            rpc_client.get_recent_blockhash()?,
        );

        // Send the transaction
        rpc_client.send_and_confirm_transaction(&create_token_account_tx)?;

        Ok(recipient_token_account.pubkey())
    } else {
        Ok(recipient_token_account_address)
    }
}

fn transfer_tokens(
    rpc_client: &RpcClient,
    sender_keypair: &Box<Keypair>,
    sender_token_account: &Pubkey,
    recipient_token_account: &Pubkey,
) -> ClientResult<()> {
    // Replace with the token program ID (use 'solana-tokens' on devnet)
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

    // Replace with the mint address of the token
    let mint_address = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

    // Get the sender's token account information
    let sender_token_account_info = rpc_client.get_account(sender_token_account)?;

    // Get the recipient's token account information
    let recipient_token_account_info = rpc_client.get_account(recipient_token_account)?;

    // Parse token account data
    let sender_token_account_data =
        TokenAccount::unpack_from_slice(sender_token_account_info.unwrap().data.as_slice())?;
    let recipient_token_account_data =
        TokenAccount::unpack_from_slice(recipient_token_account_info.unwrap().data.as_slice())?;

    // Create a transfer instruction
    let transfer_instruction = spl_token::instruction::transfer(
        &token_program_id,
        sender_token_account,
        recipient_token_account,
        &sender_keypair.pubkey(),
        &[&sender_keypair.pubkey()],
        sender_token_account_data.amount,
    )?;
    let transfer_tx = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&rpc_client.get_fee_payer()?.pubkey()),
        &[&sender_keypair],
        rpc_client.get_recent_blockhash()?,
    );

    // Send the transaction
    rpc_client.send_and_confirm_transaction(&transfer_tx)?;

    Ok(())
}
