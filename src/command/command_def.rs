//! TXT命令语法规则定义
//! 命令格式：指令 关键词 参数1=值1 参数2=值2

/// 核心指令枚举
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType {
    /// 初始化命令
    INIT,
    /// 插件管理命令
    PLUGIN,
    /// 路由管理命令
    ROUTE,
    /// 规则配置命令
    RULE,
    /// 配置管理命令
    CONFIG,
    /// 迭代接口命令
    ITERATE,
    /// 服务管理命令
    SERVICE,
    /// 脚本命令
    SCRIPT,
    /// 函数命令
    FUNCTION,
    /// 条件命令
    CONDITION,
    /// 循环命令
    LOOP,
    /// 数据库操作命令
    DATABASE,
    /// API定义命令
    API,
    /// 变量命令
    VARIABLE,
    /// 事务命令
    TRANSACTION,
    /// 未知命令
    Unknown(String),
}

/// 命令参数结构
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CommandParam {
    pub key: String,
    pub value: String,
}

/// TXT命令结构
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TxtCommand {
    pub cmd_type: CommandType,
    pub keyword: String,
    pub params: Vec<CommandParam>,
}

/// 条件判断类型
#[derive(Debug, PartialEq, Clone)]
pub enum ConditionType {
    /// 数值比较
    NumericComparison {
        left: String,
        operator: ComparisonOperator,
        right: String,
    },
    /// 插件状态判断
    PluginStatus {
        plugin_name: String,
        status: PluginStatus,
    },
    /// 配置存在判断
    ConfigExists(String),
}

/// 比较运算符
#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOperator {
    Eq,  // 等于
    Ne,  // 不等于
    Lt,  // 小于
    Lte, // 小于等于
    Gt,  // 大于
    Gte, // 大于等于
}

/// 插件状态
#[derive(Debug, PartialEq, Clone)]
pub enum PluginStatus {
    Enabled,
    Disabled,
    Installed,
    Uninstalled,
}

/// 条件判断结构
#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub then_commands: Vec<TxtCommand>,
    pub else_commands: Vec<TxtCommand>,
}

/// 脚本结构
#[derive(Debug, PartialEq, Clone)]
pub struct Script {
    pub name: String,
    pub commands: Vec<TxtCommand>,
    pub variables: Vec<Variable>,
}

/// 函数结构
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub commands: Vec<TxtCommand>,
    pub return_type: Option<String>,
}

/// 循环结构
#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub loop_type: LoopType,
    pub commands: Vec<TxtCommand>,
    pub condition: Option<ConditionType>,
    pub iterations: Option<u32>,
}

/// 循环类型
#[derive(Debug, PartialEq, Clone)]
pub enum LoopType {
    For,
    While,
    DoWhile,
}

/// 变量结构
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub var_type: VariableType,
}

/// 变量类型
#[derive(Debug, PartialEq, Clone)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

/// API定义结构
#[derive(Debug, PartialEq, Clone)]
pub struct ApiDefinition {
    pub path: String,
    pub method: HttpMethod,
    pub handler: String,
    pub parameters: Vec<ApiParameter>,
    pub response: ApiResponse,
}

/// HTTP方法
#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// API参数
#[derive(Debug, PartialEq, Clone)]
pub struct ApiParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default_value: Option<String>,
}

/// 参数类型
#[derive(Debug, PartialEq, Clone)]
pub enum ParameterType {
    Path,
    Query,
    Body,
    Header,
}

/// API响应
#[derive(Debug, PartialEq, Clone)]
pub struct ApiResponse {
    pub status_code: u16,
    pub content_type: String,
    pub body: Option<String>,
}

/// 数据库操作结构
#[derive(Debug, PartialEq, Clone)]
pub struct DatabaseOperation {
    pub operation_type: DatabaseOperationType,
    pub table: String,
    pub fields: Option<Vec<String>>,
    pub values: Option<Vec<String>>,
    pub condition: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
}

/// 数据库操作类型
#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseOperationType {
    Select,
    Insert,
    Update,
    Delete,
    CreateTable,
    DropTable,
    AlterTable,
}

/// 从字符串转换为命令类型
pub fn command_type_from_str(s: &str) -> CommandType {
    match s.to_uppercase().as_str() {
        "INIT" => CommandType::INIT,
        "PLUGIN" => CommandType::PLUGIN,
        "ROUTE" => CommandType::ROUTE,
        "RULE" => CommandType::RULE,
        "CONFIG" => CommandType::CONFIG,
        "ITERATE" => CommandType::ITERATE,
        "SERVICE" => CommandType::SERVICE,
        "SCRIPT" => CommandType::SCRIPT,
        "FUNCTION" => CommandType::FUNCTION,
        "CONDITION" => CommandType::CONDITION,
        "LOOP" => CommandType::LOOP,
        "DATABASE" => CommandType::DATABASE,
        "API" => CommandType::API,
        "VARIABLE" => CommandType::VARIABLE,
        "TRANSACTION" => CommandType::TRANSACTION,
        unknown => CommandType::Unknown(unknown.to_string()),
    }
}

/// 从字符串转换为比较运算符
pub fn comparison_operator_from_str(s: &str) -> Option<ComparisonOperator> {
    match s {
        "==" => Some(ComparisonOperator::Eq),
        "!=" => Some(ComparisonOperator::Ne),
        "<" => Some(ComparisonOperator::Lt),
        "<=" => Some(ComparisonOperator::Lte),
        ">" => Some(ComparisonOperator::Gt),
        ">=" => Some(ComparisonOperator::Gte),
        _ => None,
    }
}

/// 从字符串转换为插件状态
pub fn plugin_status_from_str(s: &str) -> Option<PluginStatus> {
    match s.to_lowercase().as_str() {
        "enabled" => Some(PluginStatus::Enabled),
        "disabled" => Some(PluginStatus::Disabled),
        "installed" => Some(PluginStatus::Installed),
        "uninstalled" => Some(PluginStatus::Uninstalled),
        _ => None,
    }
}

/// 从字符串转换为循环类型
pub fn loop_type_from_str(s: &str) -> Option<LoopType> {
    match s.to_lowercase().as_str() {
        "for" => Some(LoopType::For),
        "while" => Some(LoopType::While),
        "dowhile" => Some(LoopType::DoWhile),
        _ => None,
    }
}

/// 从字符串转换为变量类型
pub fn variable_type_from_str(s: &str) -> Option<VariableType> {
    match s.to_lowercase().as_str() {
        "string" => Some(VariableType::String),
        "number" => Some(VariableType::Number),
        "boolean" => Some(VariableType::Boolean),
        "array" => Some(VariableType::Array),
        "object" => Some(VariableType::Object),
        _ => None,
    }
}

/// 从字符串转换为HTTP方法
pub fn http_method_from_str(s: &str) -> Option<HttpMethod> {
    match s.to_uppercase().as_str() {
        "GET" => Some(HttpMethod::GET),
        "POST" => Some(HttpMethod::POST),
        "PUT" => Some(HttpMethod::PUT),
        "DELETE" => Some(HttpMethod::DELETE),
        "PATCH" => Some(HttpMethod::PATCH),
        _ => None,
    }
}

/// 从字符串转换为参数类型
pub fn parameter_type_from_str(s: &str) -> Option<ParameterType> {
    match s.to_lowercase().as_str() {
        "path" => Some(ParameterType::Path),
        "query" => Some(ParameterType::Query),
        "body" => Some(ParameterType::Body),
        "header" => Some(ParameterType::Header),
        _ => None,
    }
}

/// 从字符串转换为数据库操作类型
pub fn database_operation_type_from_str(s: &str) -> Option<DatabaseOperationType> {
    match s.to_uppercase().as_str() {
        "SELECT" => Some(DatabaseOperationType::Select),
        "INSERT" => Some(DatabaseOperationType::Insert),
        "UPDATE" => Some(DatabaseOperationType::Update),
        "DELETE" => Some(DatabaseOperationType::Delete),
        "CREATETABLE" => Some(DatabaseOperationType::CreateTable),
        "DROPTABLE" => Some(DatabaseOperationType::DropTable),
        "ALTERTABLE" => Some(DatabaseOperationType::AlterTable),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_type_from_str() {
        assert_eq!(command_type_from_str("init"), CommandType::INIT);
        assert_eq!(command_type_from_str("PLUGIN"), CommandType::PLUGIN);
        assert_eq!(
            command_type_from_str("unknown"),
            CommandType::Unknown("UNKNOWN".to_string())
        );
    }

    #[test]
    fn test_comparison_operator_from_str() {
        assert_eq!(
            comparison_operator_from_str("=="),
            Some(ComparisonOperator::Eq)
        );
        assert_eq!(
            comparison_operator_from_str(">"),
            Some(ComparisonOperator::Gt)
        );
        assert_eq!(
            comparison_operator_from_str("!="),
            Some(ComparisonOperator::Ne)
        );
        assert_eq!(comparison_operator_from_str("invalid"), None);
    }

    #[test]
    fn test_plugin_status_from_str() {
        assert_eq!(
            plugin_status_from_str("enabled"),
            Some(PluginStatus::Enabled)
        );
        assert_eq!(
            plugin_status_from_str("DISABLED"),
            Some(PluginStatus::Disabled)
        );
        assert_eq!(plugin_status_from_str("invalid"), None);
    }
}
