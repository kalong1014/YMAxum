// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 插件格式处理模块
//! 负责解析和处理插件格式（.axpl、目录）

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

/// 插件格式
#[derive(Debug, PartialEq)]
pub enum PluginFormat {
    /// .axpl压缩包格式
    Axpl,
    /// 目录格式
    Directory,
    /// 未知格式
    Unknown,
}

/// 插件清单
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    /// 插件名称
    pub name: String,
    /// 插件版本
    pub version: String,
    /// 插件作者
    pub author: String,
    /// 插件描述
    pub description: String,
    /// 插件类型
    pub plugin_type: String,
    /// 依赖列表
    pub dependencies: Vec<PluginDependency>,
    /// 核心版本
    pub core_version: String,
    /// 入口文件
    pub entry_file: String,
    /// 配置文件
    pub config_file: Option<String>,
    /// 签名文件
    pub signature_file: String,
    /// 许可证
    pub license: String,
    /// 路由配置
    pub routes: Option<Vec<PluginRoute>>,
}

/// 插件路由配置
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PluginRoute {
    /// 路由路径
    pub path: String,
    /// HTTP方法
    pub method: String,
    /// 处理器函数名
    pub handler: String,
    /// 路由描述
    pub description: Option<String>,
    /// 是否需要认证
    pub require_auth: Option<bool>,
}

/// 插件路由类型别名，用于简化类型定义
pub type PluginRoutes = Vec<PluginRoute>;

/// 插件依赖
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PluginDependency {
    /// 依赖名称
    pub name: String,
    /// 依赖版本
    pub version: String,
    /// 依赖类型
    pub dep_type: String,
    /// 是否可选
    pub optional: bool,
}

/// 插件格式处理器
#[derive(Debug, Clone)]
pub struct PluginFormatHandler;

impl PluginFormatHandler {
    /// 创建新的格式处理器
    pub fn new() -> Self {
        Self {}
    }

    /// 检测插件格式
    pub fn detect_format<P: AsRef<Path>>(&self, path: P) -> PluginFormat {
        let path = path.as_ref();

        if path.is_dir() {
            return PluginFormat::Directory;
        }

        if let Some(ext) = path.extension()
            && ext == "axpl"
        {
            return PluginFormat::Axpl;
        }

        PluginFormat::Unknown
    }

    /// 解析插件清单
    pub fn parse_manifest<P: AsRef<Path>>(&self, plugin_path: P) -> io::Result<PluginManifest> {
        let format = self.detect_format(&plugin_path);

        match format {
            PluginFormat::Axpl => self.parse_manifest_from_axpl(&plugin_path),
            PluginFormat::Directory => self.parse_manifest_from_dir(&plugin_path),
            PluginFormat::Unknown => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "不支持的插件格式",
            )),
        }
    }

    /// 从.axpl文件解析清单
    fn parse_manifest_from_axpl<P: AsRef<Path>>(&self, axpl_path: P) -> io::Result<PluginManifest> {
        let file = File::open(axpl_path.as_ref())?;
        let mut archive = ZipArchive::new(file)?;

        // 查找plugin_metadata.json文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name() == "plugin_metadata.json" {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let manifest: PluginManifest = serde_json::from_str(&contents)?;
                return Ok(manifest);
            }
        }

        // 如果没有找到plugin_metadata.json，尝试manifest.json
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name() == "manifest.json" {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let manifest: PluginManifest = serde_json::from_str(&contents)?;
                return Ok(manifest);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "未找到插件元数据文件",
        ))
    }

    /// 从目录解析清单
    fn parse_manifest_from_dir<P: AsRef<Path>>(&self, dir_path: P) -> io::Result<PluginManifest> {
        let manifest_path = dir_path.as_ref().join("manifest.json");
        let mut file = File::open(manifest_path)?;

        let mut manifest_content = String::new();
        file.read_to_string(&mut manifest_content)?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_content)?;
        Ok(manifest)
    }

    /// 打包插件为.axpl格式
    pub fn package_plugin<P: AsRef<Path>>(&self, plugin_dir: P, output_path: P) -> io::Result<()> {
        let plugin_dir = plugin_dir.as_ref();
        let output_path = output_path.as_ref();

        // 创建输出文件
        let file = File::create(output_path)?;
        let mut zip = ZipWriter::new(file);

        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // 读取并添加manifest.json
        let manifest_path = plugin_dir.join("manifest.json");
        if manifest_path.exists() {
            let mut manifest_file = File::open(&manifest_path)?;
            let mut manifest_content = Vec::new();
            manifest_file.read_to_end(&mut manifest_content)?;
            zip.start_file("manifest.json", options)?;
            zip.write_all(&manifest_content)?;
        }

        // 读取并添加plugin_metadata.json
        let metadata_path = plugin_dir.join("plugin_metadata.json");
        if metadata_path.exists() {
            let mut metadata_file = File::open(&metadata_path)?;
            let mut metadata_content = Vec::new();
            metadata_file.read_to_end(&mut metadata_content)?;
            zip.start_file("plugin_metadata.json", options)?;
            zip.write_all(&metadata_content)?;
        }

        // 递归添加目录中的所有文件
        self.add_directory_to_zip(&mut zip, plugin_dir, "", &options)?;

        zip.finish()?;
        Ok(())
    }

    /// 递归添加目录到ZIP
    fn add_directory_to_zip(
        &self,
        zip: &mut ZipWriter<File>,
        dir_path: &Path,
        base_path: &str,
        options: &FileOptions,
    ) -> io::Result<()> {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path
                    .file_name()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "文件名为空"))?
                    .to_string_lossy()
                    .to_string();

                // 跳过manifest.json和plugin_metadata.json，因为已经添加过了
                if file_name == "manifest.json" || file_name == "plugin_metadata.json" {
                    continue;
                }

                let relative_path = if base_path.is_empty() {
                    file_name.clone()
                } else {
                    format!("{}/{}", base_path, file_name)
                };

                let mut file = File::open(&path)?;
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;

                zip.start_file(&relative_path, *options)?;
                zip.write_all(&content)?;
            } else if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "目录名为空"))?
                    .to_string_lossy()
                    .to_string();

                let relative_path = if base_path.is_empty() {
                    dir_name.clone()
                } else {
                    format!("{}/{}", base_path, dir_name)
                };

                // 添加目录条目
                zip.add_directory(&relative_path, *options)?;

                // 递归添加目录内容
                self.add_directory_to_zip(zip, &path, &relative_path, options)?;
            }
        }

        Ok(())
    }

    /// 从.axpl文件提取文件
    pub fn extract_plugin<P: AsRef<Path>>(&self, axpl_path: P, output_dir: P) -> io::Result<()> {
        let output_dir = output_dir.as_ref();
        let file = File::open(axpl_path.as_ref())?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_path = output_dir.join(file.name());

            if file.name().ends_with('/') {
                // 创建目录
                std::fs::create_dir_all(&file_path)?;
            } else {
                // 创建父目录
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // 提取文件
                let mut output_file = File::create(&file_path)?;
                std::io::copy(&mut file, &mut output_file)?;
            }
        }

        Ok(())
    }
}

impl Default for PluginFormatHandler {
    fn default() -> Self {
        Self::new()
    }
}

