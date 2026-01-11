// src/bin/transfer.rs
//--------------------------------------------------
// 0. 依赖
//--------------------------------------------------
use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, env, sync::Arc};

//--------------------------------------------------
// 1. 复用关卡 3 的 Gas 价查询
//--------------------------------------------------
async fn get_gas_price(provider: &Provider<Http>) -> Result<U256> {
    let gp = provider.get_gas_price().await?;
    Ok(gp)
}

//--------------------------------------------------
// 2. 转账函数
//--------------------------------------------------
async fn transfer_eth(
    provider: &Provider<Http>,
    wallet: &LocalWallet,          // 钱包（里面自带私钥与地址）
    to: &str,                       // 接收方地址（字符串）
    amount_ether: f64,              // 要转多少 ETH（单位是 ETH，好读）
) -> Result<TransactionReceipt> {
    // 2.1 解析地址
    let to: Address = to.parse()?;

    // 2.2 把 ETH 转成 Wei
    let amount = ethers::utils::parse_ether(amount_ether)?;

    // 2.3 构建 EIP-1559 交易（Arbitrum 也支持 1559）
    let tx = Eip1559TransactionRequest::new()
        .to(to)
        .value(amount)
        .chain_id(421614) // Arbitrum Sepolia
        .gas(21000)       // 固定 21 k
        .max_fee_per_gas(get_gas_price(provider).await?+U256::from(1_000_000_000)) // +1 gwei)
        .max_priority_fee_per_gas(0); // 测试网可给 0

    // 2.4 把 provider 和 wallet 绑成签名中间件
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    // 2.5 发交易并等待确认
    let pending = client.send_transaction(tx, None).await?;
    println!("⛓  交易已提交，哈希：{:?}", pending.tx_hash());

    let receipt = pending.await?.ok_or_else(|| eyre::eyre!("交易未上链"))?;
    println!("✅ 交易已确认，区块号：{}", receipt.block_number.unwrap());
    Ok(receipt)
}

//--------------------------------------------------
// 3. main
//--------------------------------------------------
#[tokio::main]
async fn main() -> Result<()> {
    // 3.1 从环境变量读私钥（禁止硬编码）
    let priv_key = env::var("PRIVATE_KEY")?;
    let wallet = priv_key
        .parse::<LocalWallet>()?
        .with_chain_id(421614u64); // Arbitrum Sepolia

    let my_address = wallet.address();
    println!("🧑  当前钱包地址：{:?}", my_address);

    // 3.2 连 Arbitrum Sepolia RPC
    let provider = Provider::<Http>::try_from(
        "https://sepolia-rollup.arbitrum.io/rpc",
    )?;

    // 3.3 收款地址（可换成任意 Arbitrum Sepolia 地址）
    let to_address = "0x07EC3cC10e2917c3C25bCD7473485f99C6205200";

    // 3.4 转 0.001 ETH 做演示
    let receipt = transfer_eth(&provider, &wallet, to_address, 0.001).await?;

    println!("🎉 转账完成，收据：{:?}", receipt);
    Ok(())
}