use litesvm_utils::{LiteSVMBuilder, TestHelpers};
use solana_signer::Signer;

let mut svm = LiteSVMBuilder::build();
let authority = svm.create_funded_account(1_000_000_000).unwrap();
let mint = svm.create_token_mint(&authority, 6).unwrap();
let ata = svm
    .create_associated_token_account(&mint.pubkey(), &authority)
    .unwrap();
svm.mint_to(&mint.pubkey(), &ata, &authority, 1_000_000_000)
    .unwrap();
