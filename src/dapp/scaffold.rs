use log::info;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// DAPP 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DappProjectConfig {
    /// 项目名称
    pub name: String,
    /// 项目描述
    pub description: String,
    /// 作者
    pub author: String,
    /// 版本
    pub version: String,
    /// DAPP 类型
    pub dapp_type: DappType,
    /// 区块链网络
    pub blockchain: BlockchainNetwork,
    /// 前端框架
    pub frontend_framework: FrontendFramework,
    /// 是否包含智能合约
    pub include_smart_contracts: bool,
    /// 是否包含测试
    pub include_tests: bool,
}

/// DAPP 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DappType {
    /// 去中心化金融
    DeFi,
    /// 非同质化代币
    NFT,
    /// 去中心化自治组织
    DAO,
    /// 游戏
    Game,
    /// 社交
    Social,
    /// 其他
    Other,
}

/// 区块链网络
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockchainNetwork {
    /// Ethereum
    Ethereum,
    /// Binance Smart Chain
    BSC,
    /// Polygon
    Polygon,
    /// Solana
    Solana,
    /// Near
    Near,
    /// Cosmos
    Cosmos,
}

/// 前端框架
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrontendFramework {
    /// React
    React,
    /// Vue
    Vue,
    /// Angular
    Angular,
    /// Svelte
    Svelte,
    /// Godot (GUF)
    Godot,
}

/// DAPP 脚手架
pub struct DappScaffold {
    /// 配置
    config: DappProjectConfig,
}

impl DappScaffold {
    /// 创建新的 DAPP 脚手架
    pub fn new(config: DappProjectConfig) -> Self {
        Self { config }
    }

    /// 生成 DAPP 项目
    pub fn generate(&self, output_dir: &str) -> Result<(), String> {
        info!("Generating DAPP project: {}", self.config.name);
        info!("Output directory: {}", output_dir);

        // 创建项目根目录
        let project_path = Path::new(output_dir).join(&self.config.name);
        fs::create_dir_all(&project_path)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;

        // 生成项目文件
        self.generate_package_json(&project_path)?;
        self.generate_readme(&project_path)?;
        self.generate_gitignore(&project_path)?;

        // 生成目录结构
        self.generate_directory_structure(&project_path)?;

        // 生成前端代码
        self.generate_frontend(&project_path)?;

        // 生成智能合约（如果需要）
        if self.config.include_smart_contracts {
            self.generate_smart_contracts(&project_path)?;
        }

        // 生成测试（如果需要）
        if self.config.include_tests {
            self.generate_tests(&project_path)?;
        }

        info!("DAPP project generated successfully!");
        Ok(())
    }

    /// 生成 package.json 文件
    fn generate_package_json(&self, project_path: &Path) -> Result<(), String> {
        let package_json = serde_json::json!({
            "name": self.config.name,
            "version": self.config.version,
            "description": self.config.description,
            "author": self.config.author,
            "private": true,
            "scripts": {
                "dev": "echo \"Starting development server...\"",
                "build": "echo \"Building for production...\"",
                "test": "echo \"Running tests...\"",
                "deploy": "echo \"Deploying to blockchain...\""
            },
            "dependencies": {
                "web3": "^4.0.0",
                "ethers": "^6.0.0"
            },
            "devDependencies": {
                "hardhat": "^2.19.0",
                "@nomicfoundation/hardhat-toolbox": "^3.0.0"
            }
        });

        let package_json_path = project_path.join("package.json");
        let mut file = File::create(&package_json_path)
            .map_err(|e| format!("Failed to create package.json: {}", e))?;
        file.write_all(
            serde_json::to_string_pretty(&package_json)
                .unwrap()
                .as_bytes(),
        )
        .map_err(|e| format!("Failed to write package.json: {}", e))?;

        Ok(())
    }

    /// 生成 README.md 文件
    fn generate_readme(&self, project_path: &Path) -> Result<(), String> {
        let readme_content = format!(
            "# {}

{}

## 项目信息
- **Author**: {}
- **Version**: {}
- **DAPP Type**: {:?}
- **Blockchain**: {:?}
- **Frontend Framework**: {:?}

## 快速开始

### 安装依赖
```bash
npm install
```

### 开发模式
```bash
npm run dev
```

### 构建生产版本
```bash
npm run build
```

### 运行测试
```bash
npm run test
```

### 部署到区块链
```bash
npm run deploy
```

## 项目结构
- `src/` - 源代码
- `contracts/` - 智能合约
- `tests/` - 测试代码
- `scripts/` - 部署脚本
",
            self.config.name,
            self.config.description,
            self.config.author,
            self.config.version,
            self.config.dapp_type,
            self.config.blockchain,
            self.config.frontend_framework
        );

        let readme_path = project_path.join("README.md");
        let mut file =
            File::create(&readme_path).map_err(|e| format!("Failed to create README.md: {}", e))?;
        file.write_all(readme_content.as_bytes())
            .map_err(|e| format!("Failed to write README.md: {}", e))?;

        Ok(())
    }

    /// 生成 .gitignore 文件
    fn generate_gitignore(&self, project_path: &Path) -> Result<(), String> {
        let gitignore_content = r#"# Dependencies
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

# Build outputs
build/
dist/
out/

# Environment variables
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# IDE and editor files
.vscode/
.idea/
*.swp
*.swo
*~

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Hardhat
cache/
artifacts/
"#;

        let gitignore_path = project_path.join(".gitignore");
        let mut file = File::create(&gitignore_path)
            .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
        file.write_all(gitignore_content.as_bytes())
            .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

        Ok(())
    }

    /// 生成目录结构
    fn generate_directory_structure(&self, project_path: &Path) -> Result<(), String> {
        // 源代码目录
        let src_path = project_path.join("src");
        fs::create_dir_all(&src_path)
            .map_err(|e| format!("Failed to create src directory: {}", e))?;

        // 组件目录
        let components_path = src_path.join("components");
        fs::create_dir_all(&components_path)
            .map_err(|e| format!("Failed to create components directory: {}", e))?;

        // 页面目录
        let pages_path = src_path.join("pages");
        fs::create_dir_all(&pages_path)
            .map_err(|e| format!("Failed to create pages directory: {}", e))?;

        // 服务目录
        let services_path = src_path.join("services");
        fs::create_dir_all(&services_path)
            .map_err(|e| format!("Failed to create services directory: {}", e))?;

        // 工具目录
        let utils_path = src_path.join("utils");
        fs::create_dir_all(&utils_path)
            .map_err(|e| format!("Failed to create utils directory: {}", e))?;

        // 智能合约目录（如果需要）
        if self.config.include_smart_contracts {
            let contracts_path = project_path.join("contracts");
            fs::create_dir_all(&contracts_path)
                .map_err(|e| format!("Failed to create contracts directory: {}", e))?;

            // 部署脚本目录
            let scripts_path = project_path.join("scripts");
            fs::create_dir_all(&scripts_path)
                .map_err(|e| format!("Failed to create scripts directory: {}", e))?;
        }

        // 测试目录（如果需要）
        if self.config.include_tests {
            let tests_path = project_path.join("tests");
            fs::create_dir_all(&tests_path)
                .map_err(|e| format!("Failed to create tests directory: {}", e))?;
        }

        Ok(())
    }

    /// 生成前端代码
    fn generate_frontend(&self, project_path: &Path) -> Result<(), String> {
        let src_path = project_path.join("src");

        // 生成主入口文件
        let main_file = match self.config.frontend_framework {
            FrontendFramework::React => "index.jsx",
            FrontendFramework::Vue => "main.js",
            FrontendFramework::Angular => "main.ts",
            FrontendFramework::Svelte => "main.js",
            FrontendFramework::Godot => "main.gd",
        };

        let main_content = match self.config.frontend_framework {
            FrontendFramework::React => {
                r#"import React from \"react\"
import ReactDOM from \"react-dom/client\"
import App from './App'
import './index.css'

const root = ReactDOM.createRoot(document.getElementById('root'))
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
"#
            }
            FrontendFramework::Vue => {
                r#"import { createApp } from \"vue\" 
import App from './App.vue'
import './style.css'

createApp(App).mount('#app')
"#
            }
            FrontendFramework::Angular => {
                r#"import { platformBrowserDynamic } from '@angular/platform-browser-dynamic'
import { AppModule } from './app/app.module'

platformBrowserDynamic().bootstrapModule(AppModule)
  .catch(err => console.error(err))
"#
            }
            FrontendFramework::Svelte => {
                r#"import App from './App.svelte'

const app = new App({
  target: document.getElementById('app')
})

export default app
"#
            }
            FrontendFramework::Godot => {
                r#"extends Node

# Main entry point for Godot DAPP

func _ready():
    print("Godot DAPP initialized")
    # Initialize blockchain connection
    # Setup UI components
"#
            }
        };

        let main_path = src_path.join(main_file);
        let mut file =
            File::create(&main_path).map_err(|e| format!("Failed to create main file: {}", e))?;
        file.write_all(main_content.as_bytes())
            .map_err(|e| format!("Failed to write main file: {}", e))?;

        Ok(())
    }

    /// 生成智能合约
    fn generate_smart_contracts(&self, project_path: &Path) -> Result<(), String> {
        let contracts_path = project_path.join("contracts");

        // 生成示例智能合约
        let contract_content = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ExampleContract {
    string public name;
    uint256 public version;
    address public owner;

    event NameUpdated(string newName);

    constructor(string memory _name, uint256 _version) {
        name = _name;
        version = _version;
        owner = msg.sender;
    }

    function updateName(string memory _newName) public onlyOwner {
        name = _newName;
        emit NameUpdated(_newName);
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }
}
"#;

        let contract_path = contracts_path.join("ExampleContract.sol");
        let mut file = File::create(&contract_path)
            .map_err(|e| format!("Failed to create smart contract: {}", e))?;
        file.write_all(contract_content.as_bytes())
            .map_err(|e| format!("Failed to write smart contract: {}", e))?;

        // 生成部署脚本
        let scripts_path = project_path.join("scripts");
        let deploy_content = format!(
            r#"async function main() {{
  const [deployer] = await ethers.getSigners();

  console.log("Deploying contracts with the account:", deployer.address);
  console.log("Account balance:", (await deployer.getBalance()).toString());

  const ExampleContract = await ethers.getContractFactory("ExampleContract");
  const contract = await ExampleContract.deploy("{}", 1);

  await contract.deployed();

  console.log("Contract deployed to:", contract.address);
}}

main()
  .then(() => process.exit(0))
  .catch((error) => {{
    console.error(error);
    process.exit(1);
  }});
"#,
            self.config.name
        );

        let deploy_path = scripts_path.join("deploy.js");
        let mut file = File::create(&deploy_path)
            .map_err(|e| format!("Failed to create deploy script: {}", e))?;
        file.write_all(deploy_content.as_bytes())
            .map_err(|e| format!("Failed to write deploy script: {}", e))?;

        Ok(())
    }

    /// 生成测试
    fn generate_tests(&self, project_path: &Path) -> Result<(), String> {
        let tests_path = project_path.join("tests");

        // 生成示例测试
        let test_content = format!(
            r#"const {{ expect }} = require("chai");

describe("ExampleContract", function () {{
  let ExampleContract;
  let exampleContract;
  let owner;
  let addr1;

  beforeEach(async function () {{
    ExampleContract = await ethers.getContractFactory("ExampleContract");
    [owner, addr1] = await ethers.getSigners();
    exampleContract = await ExampleContract.deploy("{}", 1);
    await exampleContract.deployed();
  }});

  describe("Deployment", function () {{
    it("Should set the right name", async function () {{
      expect(await exampleContract.name()).to.equal("{}");
    }});

    it("Should set the right version", async function () {{
      expect(await exampleContract.version()).to.equal(1);
    }});

    it("Should set the right owner", async function () {{
      expect(await exampleContract.owner()).to.equal(owner.address);
    }});
  }});

  describe("Update Name", function () {{
    it("Should allow owner to update name", async function () {{
      const newName = "Updated Name";
      await exampleContract.updateName(newName);
      expect(await exampleContract.name()).to.equal(newName);
    }});

    it("Should not allow non-owner to update name", async function () {{
      const newName = "Updated Name";
      await expect(exampleContract.connect(addr1).updateName(newName))
        .to.be.revertedWith("Only owner can call this function");
    }});
  }});
}});
"#,
            self.config.name, self.config.name
        );

        let test_path = tests_path.join("example-test.js");
        let mut file =
            File::create(&test_path).map_err(|e| format!("Failed to create test file: {}", e))?;
        file.write_all(test_content.as_bytes())
            .map_err(|e| format!("Failed to write test file: {}", e))?;

        Ok(())
    }
}

/// 生成默认 DAPP 项目配置
pub fn default_dapp_config() -> DappProjectConfig {
    DappProjectConfig {
        name: "my-dapp".to_string(),
        description: "A decentralized application".to_string(),
        author: "Developer".to_string(),
        version: "1.0.0".to_string(),
        dapp_type: DappType::Other,
        blockchain: BlockchainNetwork::Ethereum,
        frontend_framework: FrontendFramework::React,
        include_smart_contracts: true,
        include_tests: true,
    }
}

/// 示例使用
pub fn example_usage() -> Result<(), String> {
    let config = default_dapp_config();
    let scaffold = DappScaffold::new(config);
    scaffold.generate(".")
}
