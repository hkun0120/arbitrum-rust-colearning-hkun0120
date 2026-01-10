use ethers::prelude::*;
use eyre::Result;
use std::convert::TryFrom;

async fn get_eth_balance(
    provider: &Provider<Http>,
    address: &str,
) -> Result<(U256, String)> {
    let address: Address = address.parse()?;
    let balance_wei = provider.get_balance(address, None).await?;
    
    // 将 wei 转换为 ETH 格式（除以 10^18）
    let balance_eth = balance_wei.as_u128() as f64 / 1e18;
    let balance_eth_str = if balance_eth >= 1.0 {
        format!("{:.6}", balance_eth).trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        format!("{:.18}", balance_eth).trim_end_matches('0').trim_end_matches('.').to_string()
    };
    
    Ok((balance_wei, balance_eth_str))
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 查询 Arbitrum 测试网地址余额\n");
    
    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;
    let address = "0x07EC3cC10e2917c3C25bCD7473485f99C6205200";
    
    println!("查询地址: {}", address);
    match get_eth_balance(&provider, address).await {
        Ok((balance_wei, balance_eth)) => {
            println!("✅ 余额查询成功！");
            println!("📊 余额 (wei): {}", balance_wei);
            println!("💰 余额 (ETH): {} ETH", balance_eth);
        }
        Err(e) => {
            println!("❌ 查询余额失败: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

