// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::merchant::MerchantManager;
use super::profit::ProfitManager;
use super::settlement::SettlementManager;
use crate::scene::SceneAdapter;
use log::info;

/// 商城场景适配器
pub struct MallSceneAdapter {
    /// 商户管理器
    merchant_manager: Option<MerchantManager>,
    /// 利润管理器
    profit_manager: Option<ProfitManager>,
    /// 结算管理器
    settlement_manager: Option<SettlementManager>,
    /// 场景名称
    scene_name: &'static str,
    /// 是否已初始化
    initialized: bool,
    /// 是否已启动
    started: bool,
}

impl MallSceneAdapter {
    /// 创建新的商城场景适配器实例
    pub fn new() -> Self {
        Self {
            merchant_manager: None,
            profit_manager: None,
            settlement_manager: None,
            scene_name: "mall",
            initialized: false,
            started: false,
        }
    }
}

impl SceneAdapter for MallSceneAdapter {
    /// 获取场景名称
    fn name(&self) -> &'static str {
        self.scene_name
    }

    /// 初始化场景
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            info!("商城场景已初始化");
            return Ok(());
        }

        // 初始化商户管理器，最大支持1000个商户
        self.merchant_manager = Some(MerchantManager::new(1000));
        info!("商户管理器已初始化，最大商户数：1000");

        // 初始化利润管理器
        self.profit_manager = Some(ProfitManager::new());
        info!("利润管理器已初始化");

        // 初始化结算管理器
        self.settlement_manager = Some(SettlementManager::new());
        info!("结算管理器已初始化");

        self.initialized = true;
        info!("商城场景初始化完成");
        Ok(())
    }

    /// 启动场景
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("商城场景未初始化，请先调用init()".into());
        }

        if self.started {
            info!("商城场景已启动");
            return Ok(());
        }

        // 启动定时结算任务
        tokio::spawn(async move {
            loop {
                // 每小时执行一次结算任务
                tokio::time::sleep(std::time::Duration::from_hours(1)).await;
                info!("执行商城场景定时结算任务");
            }
        });

        self.started = true;
        info!("商城场景已启动");
        Ok(())
    }

    /// 停止场景
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.started {
            info!("商城场景已停止");
            return Ok(());
        }

        // 清理资源
        self.merchant_manager = None;
        self.profit_manager = None;
        self.settlement_manager = None;

        self.started = false;
        self.initialized = false;
        info!("商城场景已停止");
        Ok(())
    }
}

impl Default for MallSceneAdapter {
    fn default() -> Self {
        Self::new()
    }
}

