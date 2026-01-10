use ethers::prelude::*;
use eyre::Result;
use std::convert::TryFrom;

/// 查询指定地址的 ETH 余额
/// 
/// # 参数
/// * `provider` - Ethereum provider 实例
/// * `address` - 要查询的以太坊地址（字符串格式）
/// 
/// # 返回
/// * `Result<(U256, String)>` - 返回 (wei 格式余额, ETH 格式余额字符串)
async fn get_eth_balance(
    provider: &Provider<Http>,
    address: &str,
) -> Result<(U256, String)> {
    // 解析地址字符串为 Address 类型
    let address: Address = address.parse()?;
    
    // 查询余额（返回 U256 类型，单位：wei）
    let balance_wei = provider.get_balance(address, None).await?;
    
    // 将 wei 转换为 ETH 格式（除以 10^18）
    // 手动计算以保留精度：将 U256 转换为浮点数进行除法
    let balance_eth = balance_wei.as_u128() as f64 / 1e18;
    
    // 如果余额很大，使用整数格式，否则使用小数格式
    let balance_eth_str = if balance_eth >= 1.0 {
        format!("{:.6}", balance_eth).trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        format!("{:.18}", balance_eth).trim_end_matches('0').trim_end_matches('.').to_string()
    };
    
    Ok((balance_wei, balance_eth_str))
}

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

    println!("\n{}", "=".repeat(50));
    println!("💰 查询地址 ETH 余额");
    println!("{}", "=".repeat(50));
    
    // 示例：查询一个 Arbitrum 测试网地址的余额
    // 这里使用一个示例地址，您可以替换为任何有效的以太坊地址
    // let test_address = "0x0000000000000000000000000000000000000000"; // 零地址示例
    let test_address = "0x07EC3cC10e2917c3C25bCD7473485f99C6205200"; // 地址示例
    
    println!("\n查询地址: {}", test_address);
    match get_eth_balance(&provider, test_address).await {
        Ok((balance_wei, balance_eth)) => {
            println!("✅ 余额查询成功！");
            println!("📊 余额 (wei): {}", balance_wei);
            println!("💰 余额 (ETH): {} ETH", balance_eth);
        }
        Err(e) => {
            println!("❌ 查询余额失败: {}", e);
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("🎉 Arbitrum 测试网连接成功！");
    
    Ok(())
    
}
