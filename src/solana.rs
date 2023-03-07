use anyhow::Result;
use futures::future;
use solana_account_decoder::UiAccountData;
use std::{str::FromStr, sync::Arc};

use solana_client::{
    rpc_client::RpcClient, rpc_request::TokenAccountsFilter, rpc_response::RpcKeyedAccount,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

#[allow(unused)]
pub enum Cluster {
    Testnet,
    Devnet,
    MainnetBeta,
}

impl Cluster {
    fn endpoint(&self) -> &str {
        match self {
            &Cluster::Devnet => "https://api.devnet.solana.com",
            &Cluster::MainnetBeta => "https://api.mainnet-beta.solana.com",
            &Cluster::Testnet => "https://api.testnet.solana.com",
        }
    }
}

#[derive(Debug)]
pub struct SolanaBalance {
    pub lamports: u64,
    pub sol: f64,
}

#[allow(unused)]
pub fn get_balance(pubkey: &Pubkey) -> Result<SolanaBalance> {
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com");

    let acc = rpc.get_account(&pubkey)?;
    Ok(SolanaBalance {
        lamports: acc.lamports,
        sol: (acc.lamports as f64) / 1000000000.0,
    })
}

pub async fn get_nfts(pubkey: &str, cluster: Cluster) -> Result<Vec<Nft>> {
    let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
    let rpc = Arc::new(RpcClient::new(String::from(cluster.endpoint())));
    let pubkey = Pubkey::from_str(pubkey)?;
    let commitment = CommitmentConfig::finalized();
    let accounts = rpc
        .get_token_accounts_by_owner_with_commitment(
            &pubkey,
            TokenAccountsFilter::ProgramId(token_program),
            commitment,
        )?
        .value;
    let mut futs = Vec::new();
    for RpcKeyedAccount { pubkey: _, account } in accounts.iter().take(5) {
        let UiAccountData::Json(parsed_account) = &account.data else { continue };
        let Some(mint) = parsed_account.parsed["info"]["mint"]
            .as_str() else {
                continue
            };
        let mint = mint.to_string();
        let rpc = rpc.clone();
        futs.push(async move {
            let metadata = metaboss::decode::decode(&rpc, &mint)?;
            // The JSON has a weird long `\0` terminator
            let uri = metadata
                .data
                .uri
                .split_once("\0")
                .map(|e| e.0)
                .ok_or(anyhow::anyhow!("Invalid NFT Uri"))?;
            let response = reqwest::get(uri).await?;
            let nft = response.json::<Nft>().await?;
            Ok(nft)
        });
    }
    let nft_results: Vec<Result<Nft>> =
        future::try_join_all(futs.into_iter().map(tokio::spawn)).await?;
    let nfts = nft_results.into_iter().filter_map(Result::ok).collect();
    Ok(nfts)
}

#[derive(serde::Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Nft {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
}
