use solana_client::nonblocking::websocket::WebSocketRpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::{
    utils::{get_treasury, treasury_tokens_pubkey},
    Miner,
};

impl Miner {
    pub async fn treasury(&self) {
        let client =
            WebSocketRpcClient::new_with_commitment(self.cluster.clone(), CommitmentConfig::confirmed());
        if let Ok(Some(treasury_tokens)) = client.get_token_account(&treasury_tokens_pubkey()).await
        {
            let treasury = get_treasury(self.cluster.clone()).await;
            let balance = treasury_tokens.token_amount.ui_amount_string;
            println!("{:} ORE", balance);
            println!("Admin: {}", treasury.admin);
            println!("Difficulty: {}", treasury.difficulty.to_string());
            println!("Last reset at: {}", treasury.last_reset_at);
            println!(
                "Reward rate: {} ORE",
                (treasury.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
            );
            println!(
                "Total claimed rewards: {} ORE",
                (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
            );
        }
    }
}
