//! 结算/璧勯噾安全防护模块
//! 提供鍏ㄩ摼璺祫閲戞棩蹇椼€佸紓甯歌祫閲戝憡璀︺€佸垎娑﹁绠椾笁閲嶆牎楠屽姛鑳?
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use log::{info, debug, error};
use chrono::Utc;

// 瀵煎叆鍦烘櫙模块涓殑结算目标稿叧绫诲瀷
use crate::scene::mall::settlement::SettlementConfig;

/// 退款剧粺算＄被鍨嬪埆鍚嶏紝商户ID -> 鎸夊皬鏃剁殑退款捐褰曪紙小时 -> 金额锛?type RefundStats = Arc<RwLock<std::collections::HashMap<String, Vec<(u64, f64)>>>>;

/// 璧勯噾操作绫诲瀷
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FundOperationType {
    /// 算㈠崟鏀粯
    OrderPayment,
    /// 算㈠崟退款?    OrderRefund,
    /// 分润算＄畻
    ProfitCalculation,
    /// 结算申请
    SettlementApply,
    /// 结算处理
    SettlementProcess,
    /// 结算完成
    SettlementComplete,
    /// 结算取消
    SettlementCancel,
    /// 手动调整
    ManualAdjust,
}

/// 璧勯噾操作日志
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundLog {
    /// 日志ID
    pub id: String,
    /// 操作绫诲瀷
    pub operation_type: FundOperationType,
    /// 商户ID
    pub merchant_id: String,
    /// 鍏宠仈算㈠崟ID
    pub order_id: Option<String>,
    /// 鍏宠仈结算鍗旾D
    pub settlement_id: Option<String>,
    /// 操作金额
    pub amount: f64,
    /// 操作鍓嶄綑棰?    pub before_balance: f64,
    /// 操作鍚庝綑棰?    pub after_balance: f64,
    /// 操作浜?    pub operator: String,
    /// 操作时间
    pub operation_time: u64,
    /// 操作描述
    pub description: String,
    /// 操作鐘舵€?    pub status: bool,
    /// 澶囨敞
    pub remark: Option<String>,
}

/// 寮傚父璧勯噾浜嬩欢绫诲瀷
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FundAlarmEventType {
    /// 澶ч分润
    LargeProfit,
    /// 楂橀退款?    HighFrequencyRefund,
    /// 寮傚父结算
    AbnormalSettlement,
    /// 璧勯噾寮傚父娴佸姩
    AbnormalFundFlow,
    /// 分润姣斾緥寮傚父
    AbnormalProfitRate,
}

/// 寮傚父璧勯噾鍛婅
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundAlarm {
    /// 鍛婅ID
    pub id: String,
    /// 浜嬩欢绫诲瀷
    pub event_type: FundAlarmEventType,
    /// 商户ID
    pub merchant_id: String,
    /// 鍏宠仈算㈠崟ID
    pub order_id: Option<String>,
    /// 鍏宠仈结算鍗旾D
    pub settlement_id: Option<String>,
    /// 鍛婅金额
    pub amount: f64,
    /// 鍛婅时间
    pub alarm_time: u64,
    /// 鍛婅描述
    pub description: String,
    /// 是否宸插鐞?    pub is_processed: bool,
    /// 处理浜?    pub processed_by: Option<String>,
    /// 处理时间
    pub processed_time: Option<u64>,
    /// 处理结果
    pub processed_result: Option<String>,
}

/// 璧勯噾安全配置
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundSecurityConfig {
    /// 澶ч分润鍛婅闃堝€?    pub large_profit_threshold: f64,
    /// 楂橀退款炬鏁伴槇鍊硷紙小时锛?    pub high_refund_count_threshold: u32,
    /// 楂橀退款鹃噾棰濋槇鍊硷紙小时锛?    pub high_refund_amount_threshold: f64,
    /// 寮傚父结算金额娉㈠姩闃堝€硷紙鐧惧垎姣旓級
    pub abnormal_settlement_threshold: f64,
    /// 是否启用鍛婅
    pub enable_alarm: bool,
    /// 鍛婅妫€鏌ラ棿闅旓紙绉掞級
    pub alarm_check_interval: u64,
    /// 日志保留天数
    pub log_retention_days: u32,
}

impl Default for FundSecurityConfig {
    fn default() -> Self {
        Self {
            large_profit_threshold: 10000.0, // 10000鍏?            high_refund_count_threshold: 10,  // 10娆?小时
            high_refund_amount_threshold: 5000.0, // 5000鍏?小时
            abnormal_settlement_threshold: 50.0, // 50%
            enable_alarm: true,
            alarm_check_interval: 3600, // 1小时
            log_retention_days: 90, // 90澶?        }
    }
}

/// 分润鏍￠獙结果
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfitCheckResult {
    /// 是否閫氳繃
    pub passed: bool,
    /// 瑙勫垯鏍￠獙结果
    pub rule_check_passed: bool,
    /// 鍗曞晢鎴锋牎楠岀粨鏋?    pub merchant_check_passed: bool,
    /// 鎬绘暟鏍￠獙结果
    pub total_check_passed: bool,
    /// 閿欒信息
    pub error_message: Option<String>,
    /// 鏍￠獙时间
    pub check_time: u64,
}

/// 璧勯噾操作日志璇锋眰
#[derive(Clone)]
pub struct LogFundOperationRequest {
    /// 操作绫诲瀷
    pub operation_type: FundOperationType,
    /// 商户ID
    pub merchant_id: String,
    /// 鍏宠仈算㈠崟ID
    pub order_id: Option<String>,
    /// 鍏宠仈结算鍗旾D
    pub settlement_id: Option<String>,
    /// 操作金额
    pub amount: f64,
    /// 操作鍓嶄綑棰?    pub before_balance: f64,
    /// 操作鍚庝綑棰?    pub after_balance: f64,
    /// 操作浜?    pub operator: String,
    /// 操作描述
    pub description: String,
    /// 操作鐘舵€?    pub status: bool,
    /// 澶囨敞
    pub remark: Option<String>,
}

/// 璧勯噾安全服务
#[derive(Clone)]
pub struct FundSecurityService {
    /// 配置
    config: Arc<FundSecurityConfig>,
    /// 璧勯噾操作日志
    fund_logs: Arc<RwLock<Vec<FundLog>>>,
    /// 璧勯噾鍛婅
    fund_alarms: Arc<RwLock<Vec<FundAlarm>>>,
    /// 商户璧勯噾浣欓
    merchant_balances: Arc<RwLock<std::collections::HashMap<String, f64>>>,
    /// 退款剧粺算★紙鎸夊皬鏃讹級
    refund_stats: RefundStats, // 商户ID -> 鎸夊皬鏃剁殑退款捐褰?}

impl FundSecurityService {
    /// 创建鏂扮殑璧勯噾安全服务实例
    pub fn new(config: FundSecurityConfig) -> Self {
        Self {
            config: Arc::new(config),
            fund_logs: Arc::new(RwLock::new(Vec::new())),
            fund_alarms: Arc::new(RwLock::new(Vec::new())),
            merchant_balances: Arc::new(RwLock::new(std::collections::HashMap::new())),
            refund_stats: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 算板綍璧勯噾操作日志
    pub async fn log_fund_operation(
        &self,
        request: LogFundOperationRequest,
    ) -> Result<FundLog, String> {
        let log = FundLog {
            id: format!("fund_log_{}", uuid::Uuid::new_v4()),
            operation_type: request.operation_type.clone(),
            merchant_id: request.merchant_id.clone(),
            order_id: request.order_id.clone(),
            settlement_id: request.settlement_id.clone(),
            amount: request.amount,
            before_balance: request.before_balance,
            after_balance: request.after_balance,
            operator: request.operator.clone(),
            operation_time: Utc::now().timestamp_secs() as u64,
            description: request.description.clone(),
            status: request.status,
            remark: request.remark.clone(),
        };

        let mut logs = self.fund_logs.write().await;
        logs.push(log.clone());

        // 妫€鏌ユ槸鍚﹁Е鍙戝紓甯歌祫閲戝憡璀?        self.check_abnormal_fund_event(&log).await;

        info!("璧勯噾操作日志宸茶褰? {:?}, 商户ID: {}, 金额: {:.2}", request.operation_type, request.merchant_id, request.amount);
        Ok(log)
    }

    /// 妫€鏌ュ紓甯歌祫閲戜簨浠?    async fn check_abnormal_fund_event(&self, log: &FundLog) {
        if !self.config.enable_alarm {
            return;
        }

        // 妫€鏌ュぇ棰濆垎娑?        if log.operation_type == FundOperationType::ProfitCalculation && log.amount > self.config.large_profit_threshold {
            self.create_fund_alarm(
                FundAlarmEventType::LargeProfit,
                &log.merchant_id,
                log.order_id.clone(),
                log.settlement_id.clone(),
                log.amount,
                &format!("商户 {} 鍙戠敓澶ч分润锛岄噾棰? {:.2}", log.merchant_id, log.amount),
            ).await;
        }

        // 妫€鏌ラ珮棰戦€€娆?        if log.operation_type == FundOperationType::OrderRefund {
            self.check_high_frequency_refund(log).await;
        }

        // 妫€鏌ュ紓甯哥粨绠?        if let Some(settlement_id) = &log.settlement_id {
            self.check_abnormal_settlement(log, settlement_id).await;
        }
    }

    /// 妫€鏌ラ珮棰戦€€娆?    async fn check_high_frequency_refund(&self, log: &FundLog) {
        let now = Utc::now().timestamp_secs() as u64;
        let current_hour = now / 3600;
        
        let mut stats = self.refund_stats.write().await;
        let merchant_refunds = stats.entry(log.merchant_id.clone()).or_default();
        
        // 娓呯悊鏃х殑退款捐褰曪紙瓒呰繃1小时鐨勮褰曪級
        merchant_refunds.retain(|(hour, _)| *hour == current_hour);
        
        // 娣诲姞褰撳墠退款捐褰?        merchant_refunds.push((current_hour, log.amount));
        
        // 妫€鏌ラ€€娆炬鏁板拰金额
        let refund_count = merchant_refunds.len();
        let total_refund_amount: f64 = merchant_refunds.iter().map(|(_, amount)| amount).sum();
        
        if refund_count >= self.config.high_refund_count_threshold as usize || total_refund_amount > self.config.high_refund_amount_threshold {
            self.create_fund_alarm(
                FundAlarmEventType::HighFrequencyRefund,
                &log.merchant_id,
                log.order_id.clone(),
                log.settlement_id.clone(),
                log.amount,
                &format!("商户 {} 鍙戠敓楂橀退款撅紝娆℃暟: {}, 金额: {:.2}", log.merchant_id, refund_count, total_refund_amount),
            ).await;
        }
    }

    /// 妫€鏌ュ紓甯哥粨绠?    async fn check_abnormal_settlement(&self, log: &FundLog, _settlement_id: &str) {
        // 杩欓噷鍙互实现寮傚父结算妫€鏌ラ€昏緫
        // 渚嬪锛氱粨绠楅噾棰濅笌寰呯粨绠楅噾棰濆亸宸繃澶?        // 鏆傛椂鍙仛绠€鍗曞疄鐜?        if log.amount > 100000.0 { // 10涓囧厓浠ヤ笂鐨勭粨绠?            self.create_fund_alarm(
                FundAlarmEventType::AbnormalSettlement,
                &log.merchant_id,
                log.order_id.clone(),
                log.settlement_id.clone(),
                log.amount,
                &format!("商户 {} 鍙戠敓澶ч结算锛岄噾棰? {:.2}", log.merchant_id, log.amount),
            ).await;
        }
    }

    /// 创建璧勯噾鍛婅
    async fn create_fund_alarm(
        &self,
        event_type: FundAlarmEventType,
        merchant_id: &str,
        order_id: Option<String>,
        settlement_id: Option<String>,
        amount: f64,
        description: &str,
    ) {
        let alarm = FundAlarm {
            id: format!("fund_alarm_{}", uuid::Uuid::new_v4()),
            event_type: event_type.clone(),
            merchant_id: merchant_id.to_string(),
            order_id,
            settlement_id,
            amount,
            alarm_time: Utc::now().timestamp_secs() as u64,
            description: description.to_string(),
            is_processed: false,
            processed_by: None,
            processed_time: None,
            processed_result: None,
        };

        let mut alarms = self.fund_alarms.write().await;
        alarms.push(alarm.clone());

        // 瑙﹀彂鍛婅閫氱煡
        self.trigger_alarm_notification(&alarm).await;

        error!("璧勯噾寮傚父鍛婅宸茬敓鎴? {:?}, 商户ID: {}, 金额: {:.2}, 描述: {}", event_type, merchant_id, amount, description);
    }

    /// 瑙﹀彂鍛婅閫氱煡
    async fn trigger_alarm_notification(&self, alarm: &FundAlarm) {
        // 杩欓噷鍙互实现鍛婅閫氱煡核心緫
        // 渚嬪锛氬彂閫侀偖浠躲€佺煭淇°€佷紒涓氬井淇￠€氱煡绛?        // 鏆傛椂鍙褰曟棩蹇?        debug!("鍛婅閫氱煡宸茶Е鍙? {:?}, 商户ID: {}, 金额: {:.2}", alarm.event_type, alarm.merchant_id, alarm.amount);
    }

    /// 分润算＄畻涓夐噸鏍￠獙
    pub async fn profit_calculation_check(
        &self,
        merchant_id: &str,
        order_amount: f64,
        profit_amount: f64,
        fee_amount: f64,
        actual_amount: f64,
        order_id: &str,
        config: &SettlementConfig,
    ) -> ProfitCheckResult {
        let mut passed = true;
        let mut rule_check_passed = true;
        let mut merchant_check_passed = true;
        let mut total_check_passed = true;
        let mut error_message = None;

        // 1. 瑙勫垯鏍￠獙锛氭鏌ユ墜缁垂算＄畻是否姝ｇ‘
        let calculated_fee = order_amount * config.fee_rate;
        let max_fee = config.max_fee_amount.unwrap_or(f64::INFINITY);
        let actual_calculated_fee = calculated_fee.min(max_fee);
        
        if (fee_amount - actual_calculated_fee).abs() > 0.01 { // 鍏佽0.01鍏冭宸?            rule_check_passed = false;
            passed = false;
            error_message = Some(format!("鎵嬬画璐硅绠楅敊璇紝棰勬湡: {:.2}, 实际: {:.2}", actual_calculated_fee, fee_amount));
        }

        // 2. 鍗曞晢鎴锋牎楠岋細妫€鏌ラ噾棰濇槸鍚﹀尮閰?        let expected_actual_amount = order_amount - profit_amount - fee_amount;
        if (actual_amount - expected_actual_amount).abs() > 0.01 { // 鍏佽0.01鍏冭宸?            merchant_check_passed = false;
            passed = false;
            error_message = Some(format!("鍗曞晢鎴烽噾棰濇牎楠屽け璐ワ紝棰勬湡: {:.2}, 实际: {:.2}", expected_actual_amount, actual_amount));
        }

        // 3. 鎬绘暟鏍￠獙锛氭鏌ユ墍鏈夐噾棰濅箣鍜屾槸鍚︾瓑浜庤鍗曢噾棰?        let total = profit_amount + fee_amount + actual_amount;
        if (total - order_amount).abs() > 0.01 { // 鍏佽0.01鍏冭宸?            total_check_passed = false;
            passed = false;
            error_message = Some(format!("鎬绘暟鏍￠獙澶辫触锛岄鏈? {:.2}, 实际: {:.2}", order_amount, total));
        }

        let result = ProfitCheckResult {
            passed,
            rule_check_passed,
            merchant_check_passed,
            total_check_passed,
            error_message,
            check_time: Utc::now().timestamp_secs() as u64,
        };

        // 算板綍分润鏍￠獙结果
        if !passed {
            self.log_profit_check_failure(merchant_id, order_id, &result).await;
        }

        debug!("分润算＄畻涓夐噸鏍￠獙结果: {:?}, 商户ID: {}, 算㈠崟金额: {:.2}", passed, merchant_id, order_amount);
        result
    }

    /// 算板綍分润鏍￠獙澶辫触日志
    async fn log_profit_check_failure(&self, merchant_id: &str, order_id: &str, result: &ProfitCheckResult) {
        let error_msg = result.error_message.clone().unwrap_or("鏈煡閿欒".to_string());
        self.create_fund_alarm(
            FundAlarmEventType::AbnormalProfitRate,
            merchant_id,
            Some(order_id.to_string()),
            None,
            0.0,
            &format!("商户 {} 分润算＄畻鏍￠獙澶辫触: {}", merchant_id, error_msg),
        ).await;
    }

    /// 鑾峰彇璧勯噾操作日志
    pub async fn get_fund_logs(&self, merchant_id: Option<&str>, start_time: Option<u64>, end_time: Option<u64>) -> Vec<FundLog> {
        let logs = self.fund_logs.read().await;
        let mut result = Vec::new();

        for log in logs.iter() {
            // 鎸夊晢鎴稩D绛涢€?            if let Some(mid) = merchant_id
                && log.merchant_id != mid {
                    continue;
                }

            // 鎸夋椂闂磋寖鍥寸瓫閫?            if let Some(st) = start_time
                && log.operation_time < st {
                    continue;
                }

            if let Some(et) = end_time
                && log.operation_time > et {
                    continue;
                }

            result.push(log.clone());
        }

        result
    }

    /// 鑾峰彇璧勯噾鍛婅
    pub async fn get_fund_alarms(&self, is_processed: Option<bool>) -> Vec<FundAlarm> {
        let alarms = self.fund_alarms.read().await;
        let mut result = Vec::new();

        for alarm in alarms.iter() {
            // 鎸夊鐞嗙姸鎬佺瓫閫?            if let Some(processed) = is_processed
                && alarm.is_processed != processed {
                    continue;
                }

            result.push(alarm.clone());
        }

        result
    }

    /// 处理璧勯噾鍛婅
    pub async fn process_fund_alarm(&self, alarm_id: &str, processed_by: &str, processed_result: &str) -> Result<(), String> {
        let mut alarms = self.fund_alarms.write().await;
        
        for alarm in alarms.iter_mut() {
            if alarm.id == alarm_id {
                alarm.is_processed = true;
                alarm.processed_by = Some(processed_by.to_string());
                alarm.processed_time = Some(Utc::now().timestamp_secs() as u64);
                alarm.processed_result = Some(processed_result.to_string());
                
                info!("璧勯噾鍛婅宸插鐞? {}, 处理浜? {}, 结果: {}", alarm_id, processed_by, processed_result);
                return Ok(());
            }
        }

        Err(format!("璧勯噾鍛婅涓嶅瓨鍦? {}", alarm_id))
    }

    /// 算剧疆商户璧勯噾浣欓
    pub async fn set_merchant_balance(&self, merchant_id: &str, balance: f64) {
        let mut balances = self.merchant_balances.write().await;
        balances.insert(merchant_id.to_string(), balance);
        info!("商户璧勯噾浣欓宸茶缃? {}, 浣欓: {:.2}", merchant_id, balance);
    }

    /// 鑾峰彇商户璧勯噾浣欓
    pub async fn get_merchant_balance(&self, merchant_id: &str) -> f64 {
        let balances = self.merchant_balances.read().await;
        *balances.get(merchant_id).unwrap_or(&0.0)
    }

    /// 鏇存柊商户璧勯噾浣欓
    pub async fn update_merchant_balance(&self, merchant_id: &str, amount: f64, operation_type: &FundOperationType, operator: &str, description: &str) -> Result<f64, String> {
        let mut balances = self.merchant_balances.write().await;
        let current_balance = *balances.get(merchant_id).unwrap_or(&0.0);
        let new_balance = current_balance + amount;

        // 算板綍璧勯噾操作日志
        let _log_result = self.log_fund_operation(LogFundOperationRequest {
            operation_type: operation_type.clone(),
            merchant_id: merchant_id.to_string(),
            order_id: None,
            settlement_id: None,
            amount,
            before_balance: current_balance,
            after_balance: new_balance,
            operator: operator.to_string(),
            description: description.to_string(),
            status: true,
            remark: None,
        }).await?;

        balances.insert(merchant_id.to_string(), new_balance);
        Ok(new_balance)
    }
}
