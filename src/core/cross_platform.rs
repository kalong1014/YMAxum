// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// 跨平台工具函数
pub struct CrossPlatformUtils;

impl CrossPlatformUtils {
    /// 获取当前操作系统类型
    pub fn get_os_type() -> OsType {
        if cfg!(target_os = "windows") {
            OsType::Windows
        } else if cfg!(target_os = "linux") {
            OsType::Linux
        } else if cfg!(target_os = "macos") {
            OsType::MacOS
        } else {
            OsType::Unknown
        }
    }

    /// 获取平台特定的文件路径分隔符
    pub fn path_separator() -> char {
        if cfg!(target_os = "windows") {
            '\\'
        } else {
            '/'
        }
    }

    /// 获取平台特定的环境变量分隔符
    pub fn env_separator() -> char {
        if cfg!(target_os = "windows") {
            ';'
        } else {
            ':'
        }
    }

    /// 获取平台特定的行结束符
    pub fn line_ending() -> &'static str {
        if cfg!(target_os = "windows") {
            "\r\n"
        } else {
            "\n"
        }
    }

    /// 将路径转换为平台特定格式
    pub fn to_platform_path(path: &str) -> String {
        let separator = Self::path_separator();
        path.replace('/', &separator.to_string())
    }

    /// 组合路径
    pub fn join_paths<I, S>(paths: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let separator = Self::path_separator();
        paths
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(&separator.to_string())
    }

    /// 获取应用数据目录
    pub fn app_data_dir() -> String {
        match Self::get_os_type() {
            OsType::Windows => env::var("APPDATA").unwrap_or_else(|_| {
                format!(
                    r"{}\AppData\Roaming",
                    env::var("USERPROFILE").unwrap_or_else(|_| String::from(r"C:\"))
                )
            }),
            OsType::Linux => env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
                format!(
                    "{}/.local/share",
                    env::var("HOME").unwrap_or_else(|_| String::from("/"))
                )
            }),
            OsType::MacOS => {
                format!(
                    "{}/Library/Application Support",
                    env::var("HOME").unwrap_or_else(|_| String::from("/"))
                )
            }
            OsType::Unknown => env::current_dir().unwrap().to_string_lossy().into_owned(),
        }
    }

    /// 获取应用配置目录
    pub fn app_config_dir() -> String {
        match Self::get_os_type() {
            OsType::Windows => env::var("APPDATA").unwrap_or_else(|_| {
                format!(
                    r"{}\AppData\Roaming",
                    env::var("USERPROFILE").unwrap_or_else(|_| String::from(r"C:\"))
                )
            }),
            OsType::Linux => env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
                format!(
                    "{}/.config",
                    env::var("HOME").unwrap_or_else(|_| String::from("/"))
                )
            }),
            OsType::MacOS => {
                format!(
                    "{}/Library/Preferences",
                    env::var("HOME").unwrap_or_else(|_| String::from("/"))
                )
            }
            OsType::Unknown => env::current_dir().unwrap().to_string_lossy().into_owned(),
        }
    }

    /// 获取应用日志目录
    pub fn app_log_dir() -> String {
        match Self::get_os_type() {
            OsType::Windows => env::var("APPDATA").unwrap_or_else(|_| {
                format!(
                    r"{}\AppData\Roaming\logs",
                    env::var("USERPROFILE").unwrap_or_else(|_| String::from(r"C:\"))
                )
            }),
            OsType::Linux => env::var("XDG_STATE_HOME").unwrap_or_else(|_| {
                format!(
                    "{}/.local/state",
                    env::var("HOME").unwrap_or_else(|_| String::from("/"))
                )
            }),
            OsType::MacOS => {
                format!(
                    "{}/Library/Logs",
                    env::var("HOME").unwrap_or_else(|_| String::from("/"))
                )
            }
            OsType::Unknown => env::current_dir().unwrap().to_string_lossy().into_owned(),
        }
    }

    /// 获取临时目录
    pub fn temp_dir() -> String {
        env::temp_dir().to_string_lossy().into_owned()
    }

    /// 检查文件是否存在
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// 检查目录是否存在
    pub fn dir_exists(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    /// 创建目录（包括父目录）
    pub fn create_dir_all(path: &str) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| format!("Failed to create directory: {}", e))
    }

    /// 获取可执行文件后缀
    pub fn exe_suffix() -> &'static str {
        if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        }
    }

    /// 执行系统命令
    pub fn execute_command(cmd: &str, args: &[&str]) -> Result<String, String> {
        let output = Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            String::from_utf8(output.stdout)
                .map_err(|e| format!("Failed to parse command output: {}", e))
        } else {
            let stderr =
                String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Command failed: {}", stderr))
        }
    }

    /// 获取环境变量，带默认值
    pub fn get_env(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// 设置环境变量
    pub fn set_env(key: &str, value: &str) -> Result<(), String> {
        unsafe {
            env::set_var(key, value);
        }
        Ok(())
    }

    /// 生成跨平台打包脚本
    pub fn generate_build_script(output_dir: &str) -> Result<(), String> {
        let os_type = Self::get_os_type();

        match os_type {
            OsType::Windows => {
                // 生成Windows构建脚本
                let script_path = Self::join_paths([output_dir, "build.bat"]);
                let content = "@echo off
setlocal enabledelayedexpansion
echo Building YMAxum Framework for Windows...
echo ===================================
REM 设置构建目标
echo Setting build target...
set RUSTFLAGS=-C opt-level=3

REM 构建项目
echo Building project...
cargo build --release

if %errorlevel% neq 0 (
    echo Build failed!
    exit /b 1
)

echo Build completed successfully!
echo ===================================
echo Executable: target\\release\\YMAxum.exe
endlocal

echo Done!";
                fs::write(script_path, content)
                    .map_err(|e| format!("Failed to write Windows build script: {}", e))?;
            }
            OsType::Linux | OsType::MacOS => {
                // 生成Linux/macOS构建脚本
                let script_path = Self::join_paths([output_dir, "build.sh"]);
                let os_name = if os_type == OsType::Linux {
                    "Linux"
                } else {
                    "macOS"
                };
                let content = format!(
                    "#!/bin/bash

echo \"Building YMAxum Framework for {}\"
echo \"==================================\"

# 设置构建目标
echo \"Setting build target...\"
export RUSTFLAGS=-C opt-level=3

# 构建项目
echo \"Building project...\"
cargo build --release

if [ $? -ne 0 ]; then
    echo \"Build failed!\"
    exit 1
fi

echo \"Build completed successfully!\"
echo \"==================================\"
echo \"Executable: target/release/YMAxum\"
echo \"Done!\"",
                    os_name
                );
                fs::write(&script_path, content)
                    .map_err(|e| format!("Failed to write Unix build script: {}", e))?;

                // 添加执行权限
                if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                    Command::new("chmod")
                        .args(["+x", &script_path])
                        .output()
                        .map_err(|e| format!("Failed to set executable permission: {}", e))?;
                }
            }
            OsType::Unknown => {
                return Err("Unsupported OS type for build script generation".to_string());
            }
        }

        Ok(())
    }

    /// 执行跨平台打包
    pub fn execute_build() -> Result<String, String> {
        let os_type = Self::get_os_type();

        match os_type {
            OsType::Windows => {
                // 在Windows上执行构建
                Self::execute_command("cargo", &["build", "--release"])
            }
            OsType::Linux | OsType::MacOS => {
                // 在Linux/macOS上执行构建
                Self::execute_command("cargo", &["build", "--release"])
            }
            OsType::Unknown => Err("Unsupported OS type for build execution".to_string()),
        }
    }

    /// 清理构建输出
    pub fn clean_build_output() -> Result<String, String> {
        Self::execute_command("cargo", &["clean"])
    }

    /// 获取构建输出路径
    pub fn get_build_output_path() -> String {
        let os_type = Self::get_os_type();
        let exe_suffix = Self::exe_suffix();

        match os_type {
            OsType::Windows => {
                Self::join_paths(["target", "release", &format!("YMAxum{}", exe_suffix)])
            }
            OsType::Linux | OsType::MacOS => {
                Self::join_paths(["target", "release", &format!("YMAxum{}", exe_suffix)])
            }
            OsType::Unknown => {
                Self::join_paths(["target", "release", &format!("YMAxum{}", exe_suffix)])
            }
        }
    }
}

/// 操作系统类型枚举
#[derive(Debug, PartialEq)]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

/// 跨平台路径处理
pub trait PlatformPath {
    /// 转换为平台特定路径
    fn to_platform(&self) -> String;
}

impl PlatformPath for &str {
    fn to_platform(&self) -> String {
        CrossPlatformUtils::to_platform_path(self)
    }
}

impl PlatformPath for String {
    fn to_platform(&self) -> String {
        CrossPlatformUtils::to_platform_path(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_type() {
        let os_type = CrossPlatformUtils::get_os_type();
        assert!(matches!(
            os_type,
            OsType::Windows | OsType::Linux | OsType::MacOS | OsType::Unknown
        ));
    }

    #[test]
    fn test_path_separator() {
        let separator = CrossPlatformUtils::path_separator();
        if cfg!(target_os = "windows") {
            assert_eq!(separator, '\\');
        } else {
            assert_eq!(separator, '/');
        }
    }

    #[test]
    fn test_to_platform_path() {
        let path = "/path/to/file";
        let platform_path = CrossPlatformUtils::to_platform_path(path);
        if cfg!(target_os = "windows") {
            assert_eq!(platform_path, "\\path\\to\\file");
        } else {
            assert_eq!(platform_path, path);
        }
    }
}

