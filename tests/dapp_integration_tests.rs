//! DAPP 集成测试
//! 测试 DAPP 开发支持功能，包括智能合约模板、前端组件和部署流程

use ymaxum::dapp::scaffold::{DappProjectConfig, DappScaffold};
use ymaxum::dapp::templates::default_template_library;

#[tokio::test]
async fn test_dapp_templates_loading() {
    // 测试 DAPP 模板加载
    let library = default_template_library();
    let all_templates = library.get_all_templates();
    assert!(!all_templates.is_empty());

    // 测试按类型获取模板
    let defi_templates = library.get_templates_by_type("DeFi");
    assert!(!defi_templates.is_empty());

    let nft_templates = library.get_templates_by_type("NFT");
    assert!(!nft_templates.is_empty());

    let social_templates = library.get_templates_by_type("Social");
    assert!(!social_templates.is_empty());
}

#[tokio::test]
async fn test_dapp_template_by_id() {
    // 测试按 ID 获取模板
    let library = default_template_library();

    let defi_template = library.get_template("defi-lending");
    assert!(defi_template.is_some());

    let nft_template = library.get_template("nft-marketplace");
    assert!(nft_template.is_some());

    let social_template = library.get_template("social-profile");
    assert!(social_template.is_some());

    let supply_chain_template = library.get_template("supply-chain");
    assert!(supply_chain_template.is_some());
}

#[tokio::test]
async fn test_dapp_scaffold() {
    // 测试 DAPP 脚手架
    let config = DappProjectConfig {
        name: "test_dapp".to_string(),
        description: "A test DAPP".to_string(),
        author: "Test Author".to_string(),
        version: "1.0.0".to_string(),
        dapp_type: ymaxum::dapp::scaffold::DappType::DeFi,
        blockchain: ymaxum::dapp::scaffold::BlockchainNetwork::Ethereum,
        frontend_framework: ymaxum::dapp::scaffold::FrontendFramework::React,
        include_smart_contracts: true,
        include_tests: true,
    };

    let scaffold = DappScaffold::new(config);

    // 测试创建 DAPP 脚手架
    let result = scaffold.generate("./test_output");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_dapp_template_content() {
    // 测试 DAPP 模板内容完整性
    let library = default_template_library();

    let defi_template = library.get_template("defi-lending").unwrap();
    assert!(!defi_template.smart_contract_templates.is_empty());
    assert!(!defi_template.frontend_templates.is_empty());

    let supply_chain_template = library.get_template("supply-chain").unwrap();
    assert!(!supply_chain_template.smart_contract_templates.is_empty());
    assert!(!supply_chain_template.frontend_templates.is_empty());
}
