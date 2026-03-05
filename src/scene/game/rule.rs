// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct GameRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<RuleAction>,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleType {
    PlayerJoin,
    PlayerLeave,
    PlayerChat,
    PlayerTrade,
    PlayerLevelUp,
    Custom(String),
}

#[derive(Clone, Debug)]
pub struct RuleCondition {
    pub condition_type: ConditionType,
    pub operator: ComparisonOperator,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionType {
    PlayerLevel,
    PlayerCoins,
    PlayerStatus,
    PlayerItemCount,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparisonOperator {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Clone, Debug)]
pub struct RuleAction {
    pub action_type: ActionType,
    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionType {
    SendMessage,
    GiveItem,
    GiveCoins,
    KickPlayer,
    BanPlayer,
    Custom(String),
}

impl GameRule {
    pub fn new(id: &str, name: &str, rule_type: RuleType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            rule_type,
            conditions: Vec::new(),
            actions: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_condition(&mut self, condition: RuleCondition) {
        self.conditions.push(condition);
    }

    pub fn add_action(&mut self, action: RuleAction) {
        self.actions.push(action);
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn matches(&self, context: &RuleContext) -> bool {
        for condition in &self.conditions {
            if !self.evaluate_condition(condition, context) {
                return false;
            }
        }
        true
    }

    fn evaluate_condition(&self, condition: &RuleCondition, context: &RuleContext) -> bool {
        match &condition.condition_type {
            ConditionType::PlayerLevel => {
                if let Some(level) = context.player_level {
                    self.compare_values(level as i64, &condition.value, &condition.operator)
                } else {
                    false
                }
            }
            ConditionType::PlayerCoins => {
                if let Some(coins) = context.player_coins {
                    self.compare_values(coins as i64, &condition.value, &condition.operator)
                } else {
                    false
                }
            }
            ConditionType::PlayerStatus => {
                if let Some(status) = &context.player_status {
                    self.compare_values_string(status, &condition.value, &condition.operator)
                } else {
                    false
                }
            }
            ConditionType::PlayerItemCount => {
                if let Some(count) = context.player_item_count {
                    self.compare_values(count as i64, &condition.value, &condition.operator)
                } else {
                    false
                }
            }
            ConditionType::Custom(_) => false,
        }
    }

    fn compare_values(&self, actual: i64, expected: &str, operator: &ComparisonOperator) -> bool {
        let expected_value = match expected.parse::<i64>() {
            Ok(v) => v,
            Err(_) => return false,
        };

        match operator {
            ComparisonOperator::Eq => actual == expected_value,
            ComparisonOperator::Ne => actual != expected_value,
            ComparisonOperator::Lt => actual < expected_value,
            ComparisonOperator::Lte => actual <= expected_value,
            ComparisonOperator::Gt => actual > expected_value,
            ComparisonOperator::Gte => actual >= expected_value,
        }
    }

    fn compare_values_string(
        &self,
        actual: &str,
        expected: &str,
        operator: &ComparisonOperator,
    ) -> bool {
        match operator {
            ComparisonOperator::Eq => actual == expected,
            ComparisonOperator::Ne => actual != expected,
            ComparisonOperator::Lt => actual < expected,
            ComparisonOperator::Lte => actual <= expected,
            ComparisonOperator::Gt => actual > expected,
            ComparisonOperator::Gte => actual >= expected,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RuleContext {
    pub player_id: Option<String>,
    pub player_level: Option<u32>,
    pub player_coins: Option<u64>,
    pub player_status: Option<String>,
    pub player_item_count: Option<usize>,
    pub event_type: Option<String>,
}

impl Default for RuleContext {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleContext {
    pub fn new() -> Self {
        Self {
            player_id: None,
            player_level: None,
            player_coins: None,
            player_status: None,
            player_item_count: None,
            event_type: None,
        }
    }

    pub fn with_player_id(mut self, player_id: &str) -> Self {
        self.player_id = Some(player_id.to_string());
        self
    }

    pub fn with_player_level(mut self, level: u32) -> Self {
        self.player_level = Some(level);
        self
    }

    pub fn with_player_coins(mut self, coins: u64) -> Self {
        self.player_coins = Some(coins);
        self
    }

    pub fn with_player_status(mut self, status: &str) -> Self {
        self.player_status = Some(status.to_string());
        self
    }

    pub fn with_player_item_count(mut self, count: usize) -> Self {
        self.player_item_count = Some(count);
        self
    }

    pub fn with_event_type(mut self, event_type: &str) -> Self {
        self.event_type = Some(event_type.to_string());
        self
    }
}

pub struct RuleManager {
    rules: Arc<RwLock<HashMap<String, GameRule>>>,
}

impl Default for RuleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleManager {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_rule(&self, rule: GameRule) -> Result<(), Box<dyn std::error::Error>> {
        let mut rules = self.rules.write().await;

        if rules.contains_key(&rule.id) {
            return Err("Rule already exists".into());
        }

        rules.insert(rule.id.clone(), rule);
        Ok(())
    }

    pub async fn remove_rule(&self, rule_id: &str) -> bool {
        let mut rules = self.rules.write().await;
        rules.remove(rule_id).is_some()
    }

    pub async fn get_rule(&self, rule_id: &str) -> Option<GameRule> {
        let rules = self.rules.read().await;
        rules.get(rule_id).cloned()
    }

    pub async fn get_all_rules(&self) -> Vec<GameRule> {
        let rules = self.rules.read().await;
        rules.values().cloned().collect()
    }

    pub async fn get_enabled_rules(&self) -> Vec<GameRule> {
        let rules = self.rules.read().await;
        rules
            .values()
            .filter(|rule| rule.is_enabled())
            .cloned()
            .collect()
    }

    pub async fn evaluate_rules(&self, context: &RuleContext) -> Vec<RuleAction> {
        let rules = self.rules.read().await;
        let mut actions = Vec::new();
        for rule in rules.values() {
            if rule.is_enabled() && rule.matches(context) {
                actions.extend(rule.actions.clone());
            }
        }
        actions
    }

    pub async fn enable_rule(&self, rule_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.get_mut(rule_id) {
            rule.enable();
            Ok(())
        } else {
            Err("Rule not found".into())
        }
    }

    pub async fn disable_rule(&self, rule_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.get_mut(rule_id) {
            rule.disable();
            Ok(())
        } else {
            Err("Rule not found".into())
        }
    }

    pub async fn parse_rules_from_txt(
        &self,
        txt_content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for line in txt_content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 3 {
                continue;
            }

            let rule_id = parts[0];
            let rule_name = parts[1];
            let rule_type = match parts[2].to_uppercase().as_str() {
                "PLAYER_JOIN" => RuleType::PlayerJoin,
                "PLAYER_LEAVE" => RuleType::PlayerLeave,
                "PLAYER_CHAT" => RuleType::PlayerChat,
                "PLAYER_TRADE" => RuleType::PlayerTrade,
                "PLAYER_LEVEL_UP" => RuleType::PlayerLevelUp,
                custom => RuleType::Custom(custom.to_string()),
            };

            let mut rule = GameRule::new(rule_id, rule_name, rule_type);

            for i in (3..parts.len()).step_by(2) {
                if i + 1 >= parts.len() {
                    break;
                }

                let condition = RuleCondition {
                    condition_type: match parts[i].to_uppercase().as_str() {
                        "PLAYER_LEVEL" => ConditionType::PlayerLevel,
                        "PLAYER_COINS" => ConditionType::PlayerCoins,
                        "PLAYER_STATUS" => ConditionType::PlayerStatus,
                        "PLAYER_ITEM_COUNT" => ConditionType::PlayerItemCount,
                        custom => ConditionType::Custom(custom.to_string()),
                    },
                    operator: match parts[i + 1] {
                        "==" => ComparisonOperator::Eq,
                        "!=" => ComparisonOperator::Ne,
                        "<" => ComparisonOperator::Lt,
                        "<=" => ComparisonOperator::Lte,
                        ">" => ComparisonOperator::Gt,
                        ">=" => ComparisonOperator::Gte,
                        _ => ComparisonOperator::Eq,
                    },
                    value: "0".to_string(),
                };

                rule.add_condition(condition);
            }

            self.add_rule(rule).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_rule_creation() {
        let rule = GameRule::new("rule1", "Test Rule", RuleType::PlayerJoin);
        assert_eq!(rule.id, "rule1");
        assert_eq!(rule.name, "Test Rule");
        assert_eq!(rule.rule_type, RuleType::PlayerJoin);
        assert!(rule.is_enabled());
    }

    #[test]
    fn test_game_rule_conditions() {
        let mut rule = GameRule::new("rule1", "Test Rule", RuleType::PlayerJoin);
        let condition = RuleCondition {
            condition_type: ConditionType::PlayerLevel,
            operator: ComparisonOperator::Gte,
            value: "10".to_string(),
        };
        rule.add_condition(condition);
        assert_eq!(rule.conditions.len(), 1);
    }

    #[test]
    fn test_game_rule_actions() {
        let mut rule = GameRule::new("rule1", "Test Rule", RuleType::PlayerJoin);
        let action = RuleAction {
            action_type: ActionType::SendMessage,
            parameters: {
                let mut params = HashMap::new();
                params.insert("message".to_string(), "Welcome!".to_string());
                params
            },
        };
        rule.add_action(action);
        assert_eq!(rule.actions.len(), 1);
    }

    #[test]
    fn test_rule_context() {
        let context = RuleContext::new()
            .with_player_id("player1")
            .with_player_level(10)
            .with_player_coins(1000)
            .with_player_status("online")
            .with_player_item_count(5)
            .with_event_type("player_join");

        assert_eq!(context.player_id, Some("player1".to_string()));
        assert_eq!(context.player_level, Some(10));
        assert_eq!(context.player_coins, Some(1000));
        assert_eq!(context.player_status, Some("online".to_string()));
        assert_eq!(context.player_item_count, Some(5));
        assert_eq!(context.event_type, Some("player_join".to_string()));
    }

    #[tokio::test]
    async fn test_rule_manager() {
        let manager = RuleManager::new();
        let rule = GameRule::new("rule1", "Test Rule", RuleType::PlayerJoin);

        let result = manager.add_rule(rule).await;
        assert!(result.is_ok());

        let retrieved_rule = manager.get_rule("rule1").await;
        assert!(retrieved_rule.is_some());
    }

    #[tokio::test]
    async fn test_rule_evaluation() {
        let manager = RuleManager::new();
        let mut rule = GameRule::new("rule1", "Test Rule", RuleType::PlayerJoin);

        let condition = RuleCondition {
            condition_type: ConditionType::PlayerLevel,
            operator: ComparisonOperator::Gte,
            value: "10".to_string(),
        };
        rule.add_condition(condition);

        let action = RuleAction {
            action_type: ActionType::SendMessage,
            parameters: {
                let mut params = HashMap::new();
                params.insert("message".to_string(), "Welcome!".to_string());
                params
            },
        };
        rule.add_action(action);

        let _ = manager.add_rule(rule).await;

        let context = RuleContext::new()
            .with_player_level(15)
            .with_event_type("player_join");

        let actions = manager.evaluate_rules(&context).await;
        assert_eq!(actions.len(), 1);
    }

    #[tokio::test]
    async fn test_parse_rules_from_txt() {
        let manager = RuleManager::new();
        let txt_content = r#"
            # This is a comment
            rule1 WelcomeRule PLAYER_JOIN PLAYER_LEVEL >= 10
            rule2 LeaveRule PLAYER_LEAVE PLAYER_LEVEL < 5
        "#;

        let result = manager.parse_rules_from_txt(txt_content).await;
        assert!(result.is_ok());

        let rules = manager.get_all_rules().await;
        assert_eq!(rules.len(), 2);
    }
}

