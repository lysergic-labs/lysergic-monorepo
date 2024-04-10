use {
    anyhow::{anyhow, Result},
    borsh::BorshDeserialize,
    clap::{command, Parser, Subcommand},
    serde::Deserialize,
    solana_cli_config,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{read_keypair_file, Signer},
        transaction::Transaction,
    },
    std::{fs, str::FromStr},
    yield_tokenizer::state::YieldTokenizerState,
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
    #[arg(short, long)]
    rpc: Option<String>,
    #[arg(short, long)]
    payer: Option<String>,
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        lsu_mint: &Pubkey,
        expiry: yield_tokenizer::instruction::Expiry,
    },
    Tokenize {
        amount: u64,
        lsu_mint: &Pubkey,
        expiry: yield_tokenizer::instruction::Expiry,
    },
    Redeem {
        amount: u64,
        lsu_mint: &Pubkey,
        expiry: yield_tokenizer::instruction::Expiry,
    },
    RedeemPt {
        amount: u64,
        lsu_mint: &Pubkey,
        expiry: yield_tokenizer::instruction::Expiry,
    },
    Claim {
        lsu_mint: &Pubkey,
        expiry: yield_tokenizer::instruction::Expiry,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let solana_config_file = if let Some(ref config) = *solana_cli_config::CONFIG_FILE {
        solana_cli_config::Config::load(config).unwrap_or_default()
    } else {
        solana_cli_config::Config::default()
    };

    let wallet_keypair =
        read_keypair_file(solana_config_file.keypair_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let client = RpcClient::new_with_commitment(
        solana_config_file.json_rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );

    let mut ix: Instruction;

    match args.commands {
        Commands::Init { lsu_mint, expiry } => {
            ix = yield_tokenizer::instruction::init_yield_tokenizer(
                yield_tokenizer::id(),
                wallet_pubkey,
            )
        }
        Commands::Tokenize {
            amount,
            lsu_mint,
            expiry,
        } => {
            let yield_tokenizer_addr =
                yield_tokenizer::get_yield_tokenizer_address(lsu_mint, expiry);
            let pt_addr = yield_tokenizer::get_principal_token_address(yield_tokenizer_addr);
            let yt_addr = yield_tokenizer::get_yield_token_address(yield_tokenizer_addr);
            ix = yield_tokenizer::instruction::tokenize_yield(
                wallet_pubkey,
                yield_tokenizer_addr,
                lsu_mint,
                pt_addr,
                yt_addr,
                spl_associated_token_account::get_associated_token_account(
                    yield_tokenizer_addr,
                    lsu_mint,
                ),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, lsu_mint),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, pt_addr),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, yt_addr),
                amount,
            )
        }
        Commands::Redeem {
            amount,
            lsu_mint,
            expiry,
        } => {
            let yield_tokenizer_addr =
                yield_tokenizer::get_yield_tokenizer_address(lsu_mint, expiry);
            let pt_addr = yield_tokenizer::get_principal_token_address(yield_tokenizer_addr);
            let yt_addr = yield_tokenizer::get_yield_token_address(yield_tokenizer_addr);
            ix = yield_tokenizer::instruction::redeem(
                wallet_pubkey,
                yield_tokenizer_addr,
                lsu_mint,
                pt_addr,
                yt_addr,
                spl_associated_token_account::get_associated_token_account(
                    yield_tokenizer_addr,
                    lsu_mint,
                ),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, lsu_mint),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, pt_addr),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, yt_addr),
                amount,
            )
        }
        Commands::RedeemPt {
            amount,
            lsu_mint,
            expiry,
        } => {
            let yield_tokenizer_addr =
                yield_tokenizer::get_yield_tokenizer_address(lsu_mint, expiry);
            let pt_addr = yield_tokenizer::get_principal_token_address(yield_tokenizer_addr);
            ix = yield_tokenizer::instruction::redeem_from_pt(
                wallet_pubkey,
                yield_tokenizer_addr,
                pt_addr,
                spl_associated_token_account::get_associated_token_account(
                    yield_tokenizer_addr,
                    lsu_mint,
                ),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, lsu_mint),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, pt_addr),
                amount,
            )
        }
        Commands::Claim { lsu_mint, expiry } => {
            let yield_tokenizer_addr =
                yield_tokenizer::get_yield_tokenizer_address(lsu_mint, expiry);
            let yt_addr = yield_tokenizer::get_yield_token_address(yield_tokenizer_addr);
            ix = yield_tokenizer::instruction::redeem(
                wallet_pubkey,
                yield_tokenizer_addr,
                yt_addr,
                spl_associated_token_account::get_associated_token_account(
                    yield_tokenizer_addr,
                    lsu_mint,
                ),
                spl_associated_token_account::get_associated_token_account(
                    yield_tokenizer_addr,
                    yt_addr,
                ),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, lsu_mint),
                spl_associated_token_account::get_associated_token_account(wallet_pubkey, yt_addr),
            )
        }
    }

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&wallet_pubkey));
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Cannot retrieve latest blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);

    let id = client
        .send_and_confirm_transaction(&tx)
        .expect("Transaction failed");

    println!("{:?}", id);

    Ok(())
}
