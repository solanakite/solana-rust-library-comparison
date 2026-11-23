use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;
use spl_token::instruction::{initialize_mint, initialize_account, mint_to};
use spl_associated_token_account::instruction::create_associated_token_account;
use solana_system_interface::instruction::create_account;

let mut svm = LiteSVM::new();
let authority = Keypair::new();
svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();

let mint = Keypair::new();
let rent = svm.minimum_balance_for_rent_exemption(82);
let create_mint_ix = create_account(
    &authority.pubkey(),
    &mint.pubkey(),
    rent,
    82,
    &spl_token::id(),
);
let init_mint_ix = initialize_mint(
    &spl_token::id(),
    &mint.pubkey(),
    &authority.pubkey(),
    None,
    6,
)
.unwrap();
let mint_tx = Transaction::new(
    &[&authority, &mint],
    Message::new(&[create_mint_ix, init_mint_ix], Some(&authority.pubkey())),
    svm.latest_blockhash(),
);
svm.send_transaction(mint_tx).unwrap();

let ata = spl_associated_token_account::get_associated_token_address(
    &authority.pubkey(),
    &mint.pubkey(),
);
let create_ata_ix = create_associated_token_account(
    &authority.pubkey(),
    &authority.pubkey(),
    &mint.pubkey(),
    &spl_token::id(),
);
let ata_tx = Transaction::new(
    &[&authority],
    Message::new(&[create_ata_ix], Some(&authority.pubkey())),
    svm.latest_blockhash(),
);
svm.send_transaction(ata_tx).unwrap();

let mint_to_ix = mint_to(
    &spl_token::id(),
    &mint.pubkey(),
    &ata,
    &authority.pubkey(),
    &[],
    1_000_000_000,
)
.unwrap();
let mint_to_tx = Transaction::new(
    &[&authority],
    Message::new(&[mint_to_ix], Some(&authority.pubkey())),
    svm.latest_blockhash(),
);
svm.send_transaction(mint_to_tx).unwrap();
