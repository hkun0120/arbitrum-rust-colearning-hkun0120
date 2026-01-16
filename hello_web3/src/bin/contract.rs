// Arbitrum Sepolia 测试网合约交互示例
// 合约地址: ARB Token on Arbitrum Sepolia (示例 ERC20 代币)

use ethers::prelude::*;
use std::sync::Arc;

// ERC20 标准 ABI (只包含我们需要的只读方法)
abigen!(
    IERC20,
    r#"[
        function name() public view returns (string)
        function symbol() public view returns (string)
        function decimals() public view returns (uint8)
        function totalSupply() public view returns (uint256)
        function balanceOf(address account) public view returns (uint256)
    ]"#,
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Arbitrum Sepolia 测试网 RPC 端点
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 连接到 Arbitrum Sepolia 测试网
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    
    // 使用一个真实的 ERC20 合约地址 (Arbitrum Sepolia 测试网上的 USDC 测试代币)
    // 地址: 0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d
    // 其他可用的测试合约示例:
    // - DAI: 0xfe045beB14e92C6Eaeb067D42bB6D7AE1F609104
    // - WETH: 0x980B62Da83eFf3D4576C647993b0c1D7faf17c73
    let contract_address: Address = "0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d".parse()?;
    
    // 创建合约实例
    let contract = IERC20::new(contract_address, provider.clone());
    
    println!("🔗 正在连接到 Arbitrum Sepolia 测试网...");
    println!("📝 合约地址: {:?}\n", contract_address);
    
    // 1. 查询代币名称
    println!("📌 查询合约信息:");
    match contract.name().call().await {
        Ok(name) => println!("   Name: {}", name),
        Err(e) => println!("   ❌ 无法获取 name(): {}", e),
    }
    
    // 2. 查询代币符号
    match contract.symbol().call().await {
        Ok(symbol) => println!("   Symbol: {}", symbol),
        Err(e) => println!("   ❌ 无法获取 symbol(): {}", e),
    }
    
    // 3. 查询小数位数
    match contract.decimals().call().await {
        Ok(decimals) => println!("   Decimals: {}", decimals),
        Err(e) => println!("   ❌ 无法获取 decimals(): {}", e),
    }
    
    // 4. 查询总供应量
    match contract.total_supply().call().await {
        Ok(total_supply) => {
            println!("   Total Supply: {}", total_supply);
            // 格式化显示（假设 18 位小数）
            let decimals = contract.decimals().call().await.unwrap_or(18);
            let divisor = U256::from(10).pow(U256::from(decimals));
            let formatted = total_supply.checked_div(divisor).unwrap_or(U256::zero());
            println!("   Total Supply (formatted): {} tokens", formatted);
        },
        Err(e) => println!("   ❌ 无法获取 totalSupply(): {}", e),
    }
    
    // 5. 查询特定地址余额（示例地址）
    let sample_address: Address = "0x0000000000000000000000000000000000000000".parse()?;
    match contract.balance_of(sample_address).call().await {
        Ok(balance) => {
            println!("\n💰 查询地址余额:");
            println!("   地址: {:?}", sample_address);
            println!("   余额: {}", balance);
        },
        Err(e) => println!("   ❌ 无法获取 balanceOf(): {}", e),
    }
    
    println!("\n✅ 合约交互完成!");
    
    Ok(())
}