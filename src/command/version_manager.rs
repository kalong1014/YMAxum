//! 版本管理命令
//! 用于自动化版本管理、发布管理和变更日志生成

use clap::Parser;
use log::info;
use semver::Version;
use std::fs;
use std::path::Path;

/// 版本管理命令参数
#[derive(Parser, Debug)]
pub struct VersionManagerCommand {
    /// 命令操作: bump, release, changelog, show
    #[arg(short, long, default_value = "show")]
    pub command: String,

    /// 版本号部分: major, minor, patch, prerelease
    #[arg(short, long, default_value = "patch")]
    pub part: String,

    /// 版本号
    #[arg(short, long)]
    pub version: Option<String>,

    /// 发布说明
    #[arg(short, long, default_value = "")]
    pub message: String,

    /// 变更日志文件路径
    #[arg(short, long, default_value = "./RELEASE_NOTES.md")]
    pub changelog: String,

    ///  Cargo.toml 文件路径
    #[arg(short, long, default_value = "./Cargo.toml")]
    pub cargo_toml: String,

    /// 自动提交变更
    #[arg(short, long, default_value = "false")]
    pub auto_commit: bool,

    /// 预发布标识符
    #[arg(short, long, default_value = "")]
    pub prerelease: String,
}

impl VersionManagerCommand {
    /// 执行版本管理命令
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始执行版本管理命令: {}", self.command);

        match self.command.as_str() {
            "show" => {
                self.show_version().await?;
            }
            "bump" => {
                self.bump_version().await?;
            }
            "release" => {
                self.release_version().await?;
            }
            "changelog" => {
                self.generate_changelog().await?;
            }
            _ => {
                return Err(format!("未知命令: {}", self.command).into());
            }
        }

        Ok(())
    }

    /// 显示当前版本
    async fn show_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("显示当前版本");

        // 从Cargo.toml读取版本号
        let version = self.get_current_version()?;
        info!("当前版本: {}", version);
        println!("当前版本: {}", version);

        Ok(())
    }

    /// 升级版本号
    async fn bump_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("升级版本号");

        // 获取当前版本
        let current_version = self.get_current_version()?;
        info!("当前版本: {}", current_version);

        // 计算新版本
        let new_version = self.calculate_new_version(&current_version)?;
        info!("新版本: {}", new_version);

        // 更新Cargo.toml
        self.update_cargo_toml(&new_version)?;

        // 更新发布说明
        self.update_release_notes(&new_version)?;

        info!("版本升级完成: {} -> {}", current_version, new_version);
        println!("版本升级完成: {} -> {}", current_version, new_version);

        Ok(())
    }

    /// 发布版本
    async fn release_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("发布版本");

        let version = if let Some(v) = &self.version {
            v.clone()
        } else {
            self.get_current_version()?.to_string()
        };

        info!("发布版本: {}", version);

        // 生成发布说明
        self.generate_release_notes(&version)?;

        // 验证版本号
        let parsed_version = Version::parse(&version)?;
        info!("验证版本号: {}", parsed_version);

        info!("版本发布完成: {}", version);
        println!("版本发布完成: {}", version);

        Ok(())
    }

    /// 生成变更日志
    async fn generate_changelog(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("生成变更日志");

        // 读取当前版本
        let version = self.get_current_version()?;
        info!("当前版本: {}", version);

        // 生成变更日志
        self.generate_changelog_content(&version)?;

        info!("变更日志生成完成: {}", self.changelog);
        println!("变更日志生成完成: {}", self.changelog);

        Ok(())
    }

    /// 获取当前版本号
    fn get_current_version(&self) -> Result<Version, Box<dyn std::error::Error>> {
        let cargo_toml_path = Path::new(&self.cargo_toml);
        if !cargo_toml_path.exists() {
            return Err(format!("Cargo.toml 文件不存在: {}", self.cargo_toml).into());
        }

        let content = fs::read_to_string(cargo_toml_path)?;

        // 解析Cargo.toml获取版本号
        for line in content.lines() {
            if line.starts_with("version = ") {
                let version_str = line.split('"').nth(1).ok_or("无法解析版本号")?;
                return Ok(Version::parse(version_str)?);
            }
        }

        Err("未找到版本号".into())
    }

    /// 计算新版本号
    fn calculate_new_version(
        &self,
        current_version: &Version,
    ) -> Result<Version, Box<dyn std::error::Error>> {
        let (major, minor, patch) = match self.part.as_str() {
            "major" => (current_version.major + 1, 0, 0),
            "minor" => (current_version.major, current_version.minor + 1, 0),
            "patch" => (
                current_version.major,
                current_version.minor,
                current_version.patch + 1,
            ),
            _ => return Err(format!("未知版本部分: {}", self.part).into()),
        };

        // 构建版本字符串
        let version_str = if !self.prerelease.is_empty() {
            format!("{}.{}.{}-{}", major, minor, patch, self.prerelease)
        } else {
            format!("{}.{}.{}", major, minor, patch)
        };

        // 解析版本字符串
        let version = Version::parse(&version_str)?;

        Ok(version)
    }

    /// 更新Cargo.toml文件
    fn update_cargo_toml(&self, version: &Version) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_toml_path = Path::new(&self.cargo_toml);
        let content = fs::read_to_string(cargo_toml_path)?;

        let mut new_content = String::new();
        for line in content.lines() {
            if line.starts_with("version = ") {
                new_content.push_str(&format!("version = \"{}\"\n", version));
            } else {
                new_content.push_str(&format!("{}\n", line));
            }
        }

        fs::write(cargo_toml_path, new_content)?;
        info!("更新Cargo.toml版本: {}", version);

        Ok(())
    }

    /// 更新发布说明文件
    fn update_release_notes(&self, version: &Version) -> Result<(), Box<dyn std::error::Error>> {
        let changelog_path = Path::new(&self.changelog);

        let mut content = if changelog_path.exists() {
            fs::read_to_string(changelog_path)?
        } else {
            String::new()
        };

        // 添加新版本条目
        let new_entry = format!(
            "## {}\n\n{}\n\n",
            version,
            if self.message.is_empty() {
                "- 版本升级"
            } else {
                &self.message
            }
        );

        content = new_entry + &content;
        fs::write(changelog_path, content)?;
        info!("更新发布说明: {}", version);

        Ok(())
    }

    /// 生成发布说明
    fn generate_release_notes(&self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        let changelog_path = Path::new(&self.changelog);

        let content = if changelog_path.exists() {
            fs::read_to_string(changelog_path)?
        } else {
            String::new()
        };

        // 检查版本是否已存在
        if content.contains(&format!("## {}", version)) {
            info!("版本 {} 已存在于发布说明中", version);
            return Ok(());
        }

        // 添加新版本条目
        let new_entry = format!(
            "## {}\n\n{}\n\n",
            version,
            if self.message.is_empty() {
                "- 正式发布"
            } else {
                &self.message
            }
        );

        let new_content = new_entry + &content;
        fs::write(changelog_path, new_content)?;
        info!("生成发布说明: {}", version);

        Ok(())
    }

    /// 生成变更日志内容
    fn generate_changelog_content(
        &self,
        version: &Version,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let changelog_path = Path::new(&self.changelog);

        let content = format!(
            "# 变更日志\n\n## {}\n\n### 功能变更\n- 版本升级到 {}\n\n### 修复\n- 修复已知问题\n\n### 其他\n- 更新依赖\n- 优化性能\n\n",
            version, version
        );

        fs::write(changelog_path, content)?;
        info!("生成变更日志内容: {}", version);

        Ok(())
    }
}
