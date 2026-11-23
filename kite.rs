use solana_kite::{
    create_wallet, create_token_mint, create_associated_token_account,
    mint_tokens_to_token_account,
};
use litesvm::LiteSVM;
use solana_signer::Signer;

let mut svm = LiteSVM::new();
let authority = create_wallet(&mut svm, 1_000_000_000).unwrap();
let mint = create_token_mint(&mut svm, &authority, 6, None, None).unwrap();
let ata = create_associated_token_account(
    &mut svm,
    &authority.pubkey(),
    &mint,
    &authority,
    None,
)
.unwrap();
mint_tokens_to_token_account(&mut svm, &mint, &ata, 1_000_000_000, &authority, None).unwrap();
