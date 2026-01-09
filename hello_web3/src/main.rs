use ethers::prelude::*;
use eyre::Result;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Hello Web3!");
    println!("正在连接到 Arbitrum 测试网...\n");

    // Arbitrum Sepolia 测试网 RPC URL
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 创建 provider
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    println!("✅ 成功连接到 Arbitrum 测试网!");
    println!("RPC URL: {}\n", rpc_url);

    // 获取链 ID
    let chain_id = provider.get_chainid().await?;
    println!("📡 链 ID: {:?}", chain_id);

    // 获取最新区块号
    let block_number = provider.get_block_number().await?;
    println!("📦 最新区块号: {:?}", block_number);

    // 获取最新区块信息
    if let Some(block) = provider.get_block(block_number).await? {
        println!("⏰ 区块时间戳: {:?}", block.timestamp);
        println!("🔢 区块中的交易数: {:?}", block.transactions.len());
    }

    // 获取 gas 价格
    if let Ok(gas_price) = provider.get_gas_price().await {
        println!("⛽ Gas 价格: {} wei", gas_price);
        println!("⛽ Gas 价格: {:.2} Gwei", gas_price.as_u64() as f64 / 1e9);
    }

    println!("\n🎉 Arbitrum 测试网连接成功！");
    
    Ok(())
}
