use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{EnhancedCommandParser, TxtCommand};

/// 命令解析器缓存
pub struct ParserCache {
    // 命令缓存，key为命令字符串，value为解析结果
    command_cache: Arc<RwLock<HashMap<String, TxtCommand>>>,
    // 解析器实例
    parser: EnhancedCommandParser,
}

impl ParserCache {
    /// 创建新的缓存实例
    pub fn new() -> Self {
        Self {
            command_cache: Arc::new(RwLock::new(HashMap::new())),
            parser: EnhancedCommandParser::new(),
        }
    }

    /// 从字符串解析命令，使用缓存
    pub async fn parse_line(&self, line: &str, line_num: usize) -> Option<TxtCommand> {
        // 检查缓存
        let cached = {
            let cache = self.command_cache.read().await;
            cache.get(line).cloned()
        };

        if let Some(cmd) = cached {
            return Some(cmd);
        }

        // 解析命令
        let cmd = self.parser.parse_line(line, line_num);

        // 缓存结果
        if let Some(ref cmd) = cmd {
            let mut cache = self.command_cache.write().await;
            cache.insert(line.to_string(), cmd.clone());
        }

        cmd
    }

    /// 从文件解析命令，使用缓存
    pub async fn parse_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<Vec<TxtCommand>> {
        // 这里可以实现文件级别的缓存
        // 目前直接调用解析器的方法
        self.parser.parse_file(path)
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.command_cache.write().await;
        cache.clear();
    }

    /// 获取缓存大小
    pub async fn cache_size(&self) -> usize {
        let cache = self.command_cache.read().await;
        cache.len()
    }
}

impl Default for ParserCache {
    fn default() -> Self {
        Self::new()
    }
}

/// 预编译的命令模板
pub struct CommandTemplate {
    pub template: String,
    pub parameter_names: Vec<String>,
}

/// 命令预编译器
pub struct CommandPrecompiler {
    templates: HashMap<String, CommandTemplate>,
}

impl CommandPrecompiler {
    /// 创建新的预编译器实例
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// 注册命令模板
    pub fn register_template(&mut self, name: &str, template: &str) {
        // 提取参数名，格式为 ${param}
        let mut parameter_names = Vec::new();
        let mut in_param = false;
        let mut param_name = String::new();

        for c in template.chars() {
            match c {
                '$' => {
                    in_param = true;
                    param_name.clear();
                }
                '}' => {
                    if in_param {
                        in_param = false;
                        if !param_name.is_empty() {
                            parameter_names.push(param_name.clone());
                        }
                    }
                }
                _ => {
                    if in_param && c != '{' {
                        param_name.push(c);
                    }
                }
            }
        }

        self.templates.insert(name.to_string(), CommandTemplate {
            template: template.to_string(),
            parameter_names,
        });
    }

    /// 使用模板生成命令
    pub fn generate_command(&self, name: &str, parameters: &HashMap<&str, &str>) -> Option<String> {
        if let Some(template) = self.templates.get(name) {
            let mut command = template.template.clone();

            // 替换参数
            for param_name in &template.parameter_names {
                let placeholder = format!("${{{}}}", param_name);
                if let Some(value) = parameters.get(param_name.as_str()) {
                    command = command.replace(&placeholder, value);
                }
            }

            Some(command)
        } else {
            None
        }
    }

    /// 获取模板列表
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }
}

impl Default for CommandPrecompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parser_cache() {
        use crate::command::CommandType;
        let cache = ParserCache::new();

        // 测试第一次解析
        let line = "PLUGIN INSTALL NAME=test VERSION=1.0";
        let cmd1 = cache.parse_line(line, 1).await.unwrap();
        assert_eq!(cmd1.cmd_type, CommandType::PLUGIN);
        assert_eq!(cmd1.keyword, "INSTALL");

        // 测试缓存命中
        let cmd2 = cache.parse_line(line, 2).await.unwrap();
        assert_eq!(cmd1, cmd2);

        // 测试缓存大小
        assert_eq!(cache.cache_size().await, 1);

        // 测试清除缓存
        cache.clear_cache().await;
        assert_eq!(cache.cache_size().await, 0);
    }

    #[test]
    fn test_command_precompiler() {
        let mut compiler = CommandPrecompiler::new();

        // 注册模板
        compiler.register_template(
            "install_plugin",
            "PLUGIN INSTALL NAME=${name} VERSION=${version}"
        );

        // 生成命令
        let mut params = HashMap::new();
        params.insert("name", "test_plugin");
        params.insert("version", "1.0.0");

        let command = compiler.generate_command("install_plugin", &params).unwrap();
        assert_eq!(command, "PLUGIN INSTALL NAME=test_plugin VERSION=1.0.0");

        // 测试模板列表
        let templates = compiler.list_templates();
        assert!(templates.contains(&"install_plugin".to_string()));
    }
}
