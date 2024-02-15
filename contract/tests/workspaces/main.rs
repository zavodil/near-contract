use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::json_types::U128;
use near_sdk::{Gas};
use near_workspaces::network::Sandbox;
use near_workspaces::{types::NearToken, Account, Contract, Worker};

use near_sdk::serde_json;

type Balance = u128;

const CONTRACT_WASM_FILEPATH: &str = "./out/local.wasm";
const FT_WASM_FILEPATH: &str = "./out/fungible_token.wasm";


const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

async fn init(worker: &Worker<Sandbox>) -> anyhow::Result<(Contract, Contract, Account, Account, Account)> {
    let owner = worker.dev_create_account().await?;
    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let _ = ft_contract
        .call("new")
        .args_json(serde_json::json!({
            "owner_id": owner.id().to_string()
        }))
        .gas(Gas::from_tgas(100))
        .transact()
        .await?;

    let contract_wasm = std::fs::read(CONTRACT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&contract_wasm).await?;

    let _ = contract
        .call("new")
        .args_json(serde_json::json!({
            "owner_id": owner.id().to_string(),
            "token_id": ft_contract.id().to_string()
        }))
        .gas(Gas::from_tgas(100))
        .transact()
        .await?;

    // register contract in the token
    let _ = owner
        .call(ft_contract.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": contract.id().to_string(),
        }))
        .gas(Gas::from_tgas(100))
        .deposit(NearToken::from_yoctonear(1250000000000000000000))
        .transact()
        .await?;

    println!("contract: {:#?}", contract.id());
    println!("owner: {:#?}", owner.id());

    return Ok((contract, ft_contract, owner, alice, bob));
}

#[tokio::test]
async fn verify_get_owner_id() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let (contract, _, owner, alice, bob) = init(&worker).await?;

    let owner_id: serde_json::Value = contract.call("get_owner").view().await?.json()?;

    assert_eq!(owner_id, owner.id().to_string());
    assert_ne!(owner_id, contract.id().to_string());
    assert_ne!(owner_id, anon.id().to_string());

    Ok(())
}