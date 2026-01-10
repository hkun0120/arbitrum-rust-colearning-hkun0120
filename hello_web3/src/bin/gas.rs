    // - 核心目标：用 ethers-rs 获取 Arbitrum 测试网的 Gas 价格 + 基础转账 Gas 限额，编写函数计算预估转账 Gas 费；
    // - 关键提示：
    //     1. Gas 费计算公式：Gas 费 = Gas 价格 × Gas 限额（基础转账 Gas 限额可参考行业通用值）；
    //     2. 需通过 ethers-rs 动态获取实时 Gas 价格，而非硬编码。
use ethers::prelude::*;
use eyre::Result;
use std::convert::TryFrom;
use ethers::types::U256;

async fn get_gas_price(
    provider: &Provider<Http>,
) -> Result<U256> {
    let gas_price = provider.get_gas_price().await?;
    Ok(gas_price)
}

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;
    let gas_price = get_gas_price(&provider).await?;
    println!("Gas price: {:?}", gas_price);
    // 添加GAS限额: 21000以及计算gas费公式: gas费 = gas价格 * gas限额 以及预估转账Gas费:  
    let gas_limit = 21000;
    let gas_price = get_gas_price(&provider).await?;
    let gas_fee: U256 = gas_price * gas_limit;
    println!("Gas fee: {:?}", gas_fee);
    // 添加gwei 和ether的转换
    let gas_fee_gwei = gas_fee.as_u128() as f64 / 1e9;
    let gas_fee_ether = gas_fee.as_u128() as f64 / 1e18;
    println!("Gas fee in Gwei: {:?}", gas_fee_gwei);
    println!("Gas fee in Ether: {:?}", gas_fee_ether);
    Ok(())
}