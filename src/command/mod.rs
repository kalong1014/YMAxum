//! TXT命令解析引擎模块
//! 支持TXT命令的解析、执行、校验等功能

pub mod check;
pub mod command_def;
pub mod doc_generator;
pub mod executor;
pub mod parser;
pub mod enhanced_parser;
pub mod parser_cache;
pub mod error_handling;
pub mod security_enhancer;
pub mod performance_optimizer;
pub mod security_scanner;
pub mod test_generator;
pub mod transaction;
pub mod version_manager;

// 重新导出常用组件
pub use check::{KeywordCompleter, SyntaxCheckResult, SyntaxChecker};
pub use command_def::*;
pub use doc_generator::DocGeneratorCommand;
pub use executor::{CommandExecutor, ExecuteResult, ExecutionContext, ExecutionMode};
pub use parser::CommandParser;
pub use enhanced_parser::EnhancedCommandParser;
pub use parser_cache::{CommandPrecompiler, ParserCache};
pub use error_handling::{CommandError, ErrorHandler, ExecutionError, ParseError};
pub use security_enhancer::{SecurityEnhancer, SecurityIssue, SecurityIssueType, SecurityScanResult, SecuritySeverity};
pub use performance_optimizer::PerformanceOptimizerCommand;
pub use security_scanner::SecurityScannerCommand;
pub use test_generator::TestGeneratorCommand;
pub use transaction::{Transaction, TransactionManager, TransactionOperation, TransactionStatus};
pub use version_manager::VersionManagerCommand;
