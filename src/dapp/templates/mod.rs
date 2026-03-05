// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DAPP 模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DappTemplate {
    /// 模板 ID
    pub id: String,
    /// 模板名称
    pub name: String,
    /// 模板描述
    pub description: String,
    /// DAPP 类型
    pub dapp_type: String,
    /// 区块链网络
    pub blockchain: String,
    /// 前端框架
    pub frontend_framework: String,
    /// 智能合约模板
    pub smart_contract_templates: Vec<SmartContractTemplate>,
    /// 前端组件模板
    pub frontend_templates: Vec<FrontendTemplate>,
    /// 配置模板
    pub config_templates: HashMap<String, String>,
    /// 部署脚本模板
    pub deployment_templates: Vec<String>,
}

/// 智能合约模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractTemplate {
    /// 合约名称
    pub name: String,
    /// 合约文件路径
    pub path: String,
    /// 合约内容
    pub content: String,
    /// 部署参数
    pub deployment_params: Vec<String>,
}

/// 前端组件模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendTemplate {
    /// 组件名称
    pub name: String,
    /// 组件文件路径
    pub path: String,
    /// 组件内容
    pub content: String,
    /// 组件类型
    pub component_type: String,
}

/// DAPP 模板库
pub struct DappTemplateLibrary {
    /// 模板映射
    templates: HashMap<String, DappTemplate>,
}

impl DappTemplateLibrary {
    /// 创建新的 DAPP 模板库
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
        };

        // 初始化内置模板
        library.initialize_templates();
        library
    }

    /// 初始化内置模板
    fn initialize_templates(&mut self) {
        // DeFi 模板
        self.templates.insert(
            "defi-lending".to_string(),
            crate::dapp::templates::defi_lending::create_defi_lending_template(),
        );

        // NFT 模板
        self.templates.insert(
            "nft-marketplace".to_string(),
            crate::dapp::templates::nft_marketplace::create_nft_marketplace_template(),
        );

        // DAO 模板
        self.templates.insert(
            "dao-governance".to_string(),
            crate::dapp::templates::dao_governance::create_dao_governance_template(),
        );

        // 游戏模板
        self.templates
            .insert("game-items".to_string(), crate::dapp::templates::game_items::create_game_items_template());

        // 社交模板
        self.templates.insert(
            "social-profile".to_string(),
            crate::dapp::templates::social_profile::create_social_profile_template(),
        );

        // 供应链模板
        self.templates.insert(
            "supply-chain".to_string(),
            crate::dapp::templates::supply_chain::create_supply_chain_template(),
        );

        // 医疗健康模板
        self.templates
            .insert("healthcare".to_string(), crate::dapp::templates::healthcare::create_healthcare_template());

        // 教育模板
        self.templates
            .insert("education".to_string(), crate::dapp::templates::education::create_education_template());

        // 房地产模板
        self.templates.insert(
            "real-estate".to_string(),
            crate::dapp::templates::real_estate::create_real_estate_template(),
        );

        // 能源管理模板
        self.templates
            .insert("energy".to_string(), crate::dapp::templates::energy::create_energy_template());
    }

    /// 获取模板
    pub fn get_template(&self, template_id: &str) -> Option<&DappTemplate> {
        self.templates.get(template_id)
    }

    /// 获取所有模板
    pub fn get_all_templates(&self) -> Vec<&DappTemplate> {
        self.templates.values().collect()
    }

    /// 按类型获取模板
    pub fn get_templates_by_type(&self, dapp_type: &str) -> Vec<&DappTemplate> {
        self.templates
            .values()
            .filter(|template| template.dapp_type == dapp_type)
            .collect()
    }
}

// 模板模块
pub mod defi_lending;
pub mod nft_marketplace;
pub mod dao_governance;
pub mod game_items;
pub mod social_profile;
pub mod supply_chain;
pub mod healthcare;
pub mod education;
pub mod real_estate;
pub mod energy;

/// 获取默认模板库
pub fn default_template_library() -> DappTemplateLibrary {
    DappTemplateLibrary::new()
}
