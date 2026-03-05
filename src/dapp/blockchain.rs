use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 区块链网络
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
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
    /// Avalanche
    Avalanche,
    /// Arbitrum
    Arbitrum,
    /// Optimism
    Optimism,
    /// Polkadot
    Polkadot,
}

/// 区块链网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// 网络名称
    pub name: String,
    /// RPC URL
    pub rpc_url: String,
    /// 链 ID
    pub chain_id: u64,
    /// 原生代币符号
    pub native_token: String,
    /// 区块时间（秒）
    pub block_time: u64,
    /// 交易费用估算
    pub gas_estimate: u64,
}

/// 智能合约模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractTemplate {
    /// 模板 ID
    pub id: String,
    /// 模板名称
    pub name: String,
    /// 合约类型
    pub contract_type: String,
    /// 区块链网络
    pub blockchain: BlockchainNetwork,
    /// 合约代码
    pub code: String,
    /// 编译配置
    pub compile_config: CompileConfig,
    /// 部署参数
    pub deployment_params: Vec<String>,
    /// 接口定义
    pub interface: ContractInterface,
}

/// 编译配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileConfig {
    /// 编译器版本
    pub compiler_version: String,
    /// 优化级别
    pub optimization: u8,
    /// 输出格式
    pub output_format: String,
}

/// 合约接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInterface {
    /// 函数列表
    pub functions: Vec<ContractFunction>,
    /// 事件列表
    pub events: Vec<ContractEvent>,
    /// ABI 定义
    pub abi: serde_json::Value,
}

/// 合约函数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFunction {
    /// 函数名称
    pub name: String,
    /// 输入参数
    pub inputs: Vec<FunctionParam>,
    /// 输出参数
    pub outputs: Vec<FunctionParam>,
    /// 函数类型
    pub function_type: String,
    /// 常量函数
    pub constant: bool,
}

/// 函数参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParam {
    /// 参数名称
    pub name: String,
    /// 参数类型
    pub param_type: String,
    /// 索引（事件参数）
    pub indexed: bool,
}

/// 合约事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    /// 事件名称
    pub name: String,
    /// 事件参数
    pub inputs: Vec<FunctionParam>,
    /// 匿名事件
    pub anonymous: bool,
}

/// 区块链事件监听器
#[derive(Clone)]
#[allow(dead_code)]
pub struct BlockchainEventListener {
    /// 网络
    network: BlockchainNetwork,
    /// 合约地址
    contract_address: String,
    /// 事件名称
    event_name: String,
    /// 过滤器
    filter: serde_json::Value,
    /// 回调函数
    callback: Arc<dyn Fn(serde_json::Value) -> bool + Send + Sync>,
    /// 状态
    status: Arc<RwLock<EventListenerStatus>>,
}

/// 事件监听器状态
#[derive(Debug, Clone)]
pub enum EventListenerStatus {
    /// 运行中
    Running,
    /// 暂停
    Paused,
    /// 停止
    Stopped,
    /// 错误
    Error(String),
}

/// 区块链适配器
#[async_trait::async_trait]
pub trait BlockchainAdapter: Send + Sync {
    /// 获取网络配置
    fn get_network_config(&self) -> &NetworkConfig;

    /// 连接到区块链
    async fn connect(&self) -> Result<(), String>;

    /// 断开连接
    async fn disconnect(&self) -> Result<(), String>;

    /// 发送交易
    async fn send_transaction(&self, tx: TransactionRequest) -> Result<TransactionReceipt, String>;

    /// 调用合约
    async fn call_contract(&self, call: ContractCall) -> Result<serde_json::Value, String>;

    /// 部署合约
    async fn deploy_contract(&self, deploy: ContractDeploy) -> Result<ContractDeployment, String>;

    /// 监听事件
    async fn listen_for_event(&self, listener: BlockchainEventListener) -> Result<(), String>;

    /// 获取区块
    async fn get_block(&self, block_number: u64) -> Result<BlockInfo, String>;

    /// 获取交易
    async fn get_transaction(&self, tx_hash: &str) -> Result<TransactionInfo, String>;

    /// 获取账户余额
    async fn get_balance(&self, address: &str) -> Result<String, String>;
}

/// 交易请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    /// 发送方
    pub from: String,
    /// 接收方
    pub to: String,
    /// 价值
    pub value: String,
    /// 数据
    pub data: String,
    ///  gas 价格
    pub gas_price: Option<String>,
    ///  gas 限制
    pub gas_limit: Option<String>,
    ///  nonce
    pub nonce: Option<u64>,
}

/// 交易收据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    /// 交易哈希
    pub transaction_hash: String,
    /// 区块哈希
    pub block_hash: String,
    /// 区块号
    pub block_number: u64,
    /// 交易索引
    pub transaction_index: u64,
    /// 来自
    pub from: String,
    /// 到
    pub to: String,
    /// 合约地址
    pub contract_address: Option<String>,
    /// 日志
    pub logs: Vec<Log>,
    /// 状态
    pub status: bool,
    /// 有效 gas 使用
    pub gas_used: String,
}

/// 日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// 地址
    pub address: String,
    /// 主题
    pub topics: Vec<String>,
    /// 数据
    pub data: String,
    /// 区块哈希
    pub block_hash: String,
    /// 区块号
    pub block_number: u64,
    /// 交易哈希
    pub transaction_hash: String,
    /// 交易索引
    pub transaction_index: u64,
    /// 日志索引
    pub log_index: u64,
    ///  removed
    pub removed: bool,
}

/// 合约调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCall {
    /// 合约地址
    pub to: String,
    /// 数据
    pub data: String,
    /// 从
    pub from: Option<String>,
    /// 价值
    pub value: Option<String>,
}

/// 合约部署
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeploy {
    /// 字节码
    pub bytecode: String,
    /// 构造函数参数
    pub constructor_args: String,
    /// 从
    pub from: String,
    /// 价值
    pub value: Option<String>,
}

/// 合约部署结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeployment {
    /// 合约地址
    pub contract_address: String,
    /// 交易哈希
    pub transaction_hash: String,
    /// 部署时间
    pub deployed_at: u64,
}

/// 区块信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// 区块号
    pub number: u64,
    /// 哈希
    pub hash: String,
    /// 父哈希
    pub parent_hash: String,
    /// 时间戳
    pub timestamp: u64,
    /// 交易
    pub transactions: Vec<String>,
    /// gas 限制
    pub gas_limit: String,
    /// gas 使用
    pub gas_used: String,
    /// 难度
    pub difficulty: String,
    /// 矿工
    pub miner: String,
}

/// 交易信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// 哈希
    pub hash: String,
    /// 区块哈希
    pub block_hash: String,
    /// 区块号
    pub block_number: u64,
    /// 交易索引
    pub transaction_index: u64,
    /// 从
    pub from: String,
    /// 到
    pub to: String,
    /// 价值
    pub value: String,
    /// 非ce
    pub nonce: u64,
    /// 数据
    pub input: String,
    /// gas 价格
    pub gas_price: String,
    /// gas 限制
    pub gas: String,
}

/// 智能合约模板库
pub struct SmartContractTemplateLibrary {
    /// 模板映射
    templates: HashMap<String, SmartContractTemplate>,
    /// 按类型分组的模板
    templates_by_type: HashMap<String, Vec<String>>,
    /// 按区块链分组的模板
    templates_by_blockchain: HashMap<String, Vec<String>>,
}

impl SmartContractTemplateLibrary {
    /// 创建新的智能合约模板库
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
            templates_by_type: HashMap::new(),
            templates_by_blockchain: HashMap::new(),
        };

        // 初始化内置模板
        library.initialize_templates();
        library
    }

    /// 初始化内置模板
    fn initialize_templates(&mut self) {
        // ERC20 模板
        let erc20_template = self.create_erc20_template();
        self.add_template(erc20_template);

        // ERC721 模板
        let erc721_template = self.create_erc721_template();
        self.add_template(erc721_template);

        // ERC1155 模板
        let erc1155_template = self.create_erc1155_template();
        self.add_template(erc1155_template);

        // 众筹模板
        let crowdfunding_template = self.create_crowdfunding_template();
        self.add_template(crowdfunding_template);

        // 投票模板
        let voting_template = self.create_voting_template();
        self.add_template(voting_template);
    }

    /// 添加模板
    fn add_template(&mut self, template: SmartContractTemplate) {
        self.templates.insert(template.id.clone(), template.clone());

        // 按类型分组
        self.templates_by_type
            .entry(template.contract_type.clone())
            .or_default()
            .push(template.id.clone());

        // 按区块链分组
        let blockchain_key = format!("{:?}", template.blockchain);
        self.templates_by_blockchain
            .entry(blockchain_key)
            .or_default()
            .push(template.id.clone());
    }

    /// 获取模板
    pub fn get_template(&self, template_id: &str) -> Option<&SmartContractTemplate> {
        self.templates.get(template_id)
    }

    /// 获取所有模板
    pub fn get_all_templates(&self) -> Vec<&SmartContractTemplate> {
        self.templates.values().collect()
    }

    /// 按类型获取模板
    pub fn get_templates_by_type(&self, contract_type: &str) -> Vec<&SmartContractTemplate> {
        if let Some(template_ids) = self.templates_by_type.get(contract_type) {
            template_ids
                .iter()
                .filter_map(|id| self.templates.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 按区块链获取模板
    pub fn get_templates_by_blockchain(
        &self,
        blockchain: &BlockchainNetwork,
    ) -> Vec<&SmartContractTemplate> {
        let blockchain_key = format!("{:?}", blockchain);
        if let Some(template_ids) = self.templates_by_blockchain.get(&blockchain_key) {
            template_ids
                .iter()
                .filter_map(|id| self.templates.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 创建 ERC20 模板
    fn create_erc20_template(&self) -> SmartContractTemplate {
        let code = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MyToken is ERC20, Ownable {
    constructor(string memory name, string memory symbol, uint256 initialSupply) ERC20(name, symbol) {
        _mint(msg.sender, initialSupply);
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function burn(uint256 amount) public {
        _burn(msg.sender, amount);
    }
}
"#;

        let abi = serde_json::json!([
            {
                "constant": true,
                "inputs": [],
                "name": "name",
                "outputs": [{"name": "", "type": "string"}],
                "payable": false,
                "stateMutability": "view",
                "type": "function"
            },
            {
                "constant": false,
                "inputs": [
                    {"name": "to", "type": "address"},
                    {"name": "amount", "type": "uint256"}
                ],
                "name": "mint",
                "outputs": [],
                "payable": false,
                "stateMutability": "nonpayable",
                "type": "function"
            },
            {
                "constant": true,
                "inputs": [],
                "name": "symbol",
                "outputs": [{"name": "", "type": "string"}],
                "payable": false,
                "stateMutability": "view",
                "type": "function"
            },
            {
                "constant": false,
                "inputs": [{"name": "amount", "type": "uint256"}],
                "name": "burn",
                "outputs": [],
                "payable": false,
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]);

        SmartContractTemplate {
            id: "erc20-standard".to_string(),
            name: "Standard ERC20 Token".to_string(),
            contract_type: "token".to_string(),
            blockchain: BlockchainNetwork::Ethereum,
            code: code.to_string(),
            compile_config: CompileConfig {
                compiler_version: "0.8.0".to_string(),
                optimization: 200,
                output_format: "json".to_string(),
            },
            deployment_params: vec![
                "MyToken".to_string(),
                "MTK".to_string(),
                "1000000000000000000000".to_string(),
            ],
            interface: ContractInterface {
                functions: vec![
                    ContractFunction {
                        name: "name".to_string(),
                        inputs: vec![],
                        outputs: vec![FunctionParam {
                            name: "".to_string(),
                            param_type: "string".to_string(),
                            indexed: false,
                        }],
                        function_type: "view".to_string(),
                        constant: true,
                    },
                    ContractFunction {
                        name: "mint".to_string(),
                        inputs: vec![
                            FunctionParam {
                                name: "to".to_string(),
                                param_type: "address".to_string(),
                                indexed: false,
                            },
                            FunctionParam {
                                name: "amount".to_string(),
                                param_type: "uint256".to_string(),
                                indexed: false,
                            },
                        ],
                        outputs: vec![],
                        function_type: "nonpayable".to_string(),
                        constant: false,
                    },
                ],
                events: vec![],
                abi,
            },
        }
    }

    /// 创建 ERC721 模板
    fn create_erc721_template(&self) -> SmartContractTemplate {
        let code = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract MyNFT is ERC721, Ownable {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}

    function mintNFT(address recipient, string memory tokenURI) public onlyOwner returns (uint256) {
        _tokenIds.increment();
        uint256 newItemId = _tokenIds.current();
        _mint(recipient, newItemId);
        _setTokenURI(newItemId, tokenURI);
        return newItemId;
    }
}
"#;

        let abi = serde_json::json!([
            {
                "constant": true,
                "inputs": [],
                "name": "name",
                "outputs": [{"name": "", "type": "string"}],
                "payable": false,
                "stateMutability": "view",
                "type": "function"
            },
            {
                "constant": false,
                "inputs": [
                    {"name": "recipient", "type": "address"},
                    {"name": "tokenURI", "type": "string"}
                ],
                "name": "mintNFT",
                "outputs": [{"name": "", "type": "uint256"}],
                "payable": false,
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]);

        SmartContractTemplate {
            id: "erc721-standard".to_string(),
            name: "Standard ERC721 NFT".to_string(),
            contract_type: "nft".to_string(),
            blockchain: BlockchainNetwork::Ethereum,
            code: code.to_string(),
            compile_config: CompileConfig {
                compiler_version: "0.8.0".to_string(),
                optimization: 200,
                output_format: "json".to_string(),
            },
            deployment_params: vec!["MyNFT".to_string(), "MNFT".to_string()],
            interface: ContractInterface {
                functions: vec![
                    ContractFunction {
                        name: "name".to_string(),
                        inputs: vec![],
                        outputs: vec![FunctionParam {
                            name: "".to_string(),
                            param_type: "string".to_string(),
                            indexed: false,
                        }],
                        function_type: "view".to_string(),
                        constant: true,
                    },
                    ContractFunction {
                        name: "mintNFT".to_string(),
                        inputs: vec![
                            FunctionParam {
                                name: "recipient".to_string(),
                                param_type: "address".to_string(),
                                indexed: false,
                            },
                            FunctionParam {
                                name: "tokenURI".to_string(),
                                param_type: "string".to_string(),
                                indexed: false,
                            },
                        ],
                        outputs: vec![FunctionParam {
                            name: "".to_string(),
                            param_type: "uint256".to_string(),
                            indexed: false,
                        }],
                        function_type: "nonpayable".to_string(),
                        constant: false,
                    },
                ],
                events: vec![],
                abi,
            },
        }
    }

    /// 创建 ERC1155 模板
    fn create_erc1155_template(&self) -> SmartContractTemplate {
        let code = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MyMultiToken is ERC1155, Ownable {
    constructor(string memory uri) ERC1155(uri) {}

    function mint(address account, uint256 id, uint256 amount, bytes memory data) public onlyOwner {
        _mint(account, id, amount, data);
    }

    function mintBatch(address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data) public onlyOwner {
        _mintBatch(to, ids, amounts, data);
    }

    function burn(address account, uint256 id, uint256 amount) public {
        require(msg.sender == account, "Only account can burn its tokens");
        _burn(account, id, amount);
    }
}
"#;

        let abi = serde_json::json!([
            {
                "constant": false,
                "inputs": [
                    {"name": "account", "type": "address"},
                    {"name": "id", "type": "uint256"},
                    {"name": "amount", "type": "uint256"},
                    {"name": "data", "type": "bytes"}
                ],
                "name": "mint",
                "outputs": [],
                "payable": false,
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]);

        SmartContractTemplate {
            id: "erc1155-standard".to_string(),
            name: "Standard ERC1155 Multi-Token".to_string(),
            contract_type: "multi-token".to_string(),
            blockchain: BlockchainNetwork::Ethereum,
            code: code.to_string(),
            compile_config: CompileConfig {
                compiler_version: "0.8.0".to_string(),
                optimization: 200,
                output_format: "json".to_string(),
            },
            deployment_params: vec!["https://game.example/api/item/{id}.json".to_string()],
            interface: ContractInterface {
                functions: vec![ContractFunction {
                    name: "mint".to_string(),
                    inputs: vec![
                        FunctionParam {
                            name: "account".to_string(),
                            param_type: "address".to_string(),
                            indexed: false,
                        },
                        FunctionParam {
                            name: "id".to_string(),
                            param_type: "uint256".to_string(),
                            indexed: false,
                        },
                        FunctionParam {
                            name: "amount".to_string(),
                            param_type: "uint256".to_string(),
                            indexed: false,
                        },
                        FunctionParam {
                            name: "data".to_string(),
                            param_type: "bytes".to_string(),
                            indexed: false,
                        },
                    ],
                    outputs: vec![],
                    function_type: "nonpayable".to_string(),
                    constant: false,
                }],
                events: vec![],
                abi,
            },
        }
    }

    /// 创建众筹模板
    fn create_crowdfunding_template(&self) -> SmartContractTemplate {
        let code = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Crowdfunding {
    address public owner;
    uint256 public fundingGoal;
    uint256 public deadline;
    uint256 public totalFunds;
    bool public funded;
    bool public ended;

    mapping(address => uint256) public contributions;

    event Contributed(address indexed contributor, uint256 amount);
    event GoalReached(uint256 totalFunds);
    event FundsWithdrawn(address indexed owner, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner");
        _;
    }

    modifier notEnded() {
        require(!ended, "Campaign ended");
        _;
    }

    constructor(uint256 _fundingGoal, uint256 _durationDays) {
        owner = msg.sender;
        fundingGoal = _fundingGoal;
        deadline = block.timestamp + (_durationDays * 1 days);
        funded = false;
        ended = false;
        totalFunds = 0;
    }

    function contribute() public payable notEnded {
        require(msg.value > 0, "Must contribute something");
        contributions[msg.sender] += msg.value;
        totalFunds += msg.value;
        emit Contributed(msg.sender, msg.value);

        if (totalFunds >= fundingGoal && !funded) {
            funded = true;
            emit GoalReached(totalFunds);
        }
    }

    function endCampaign() public {
        require(block.timestamp >= deadline || funded, "Campaign not ended");
        require(!ended, "Campaign already ended");
        ended = true;
    }

    function withdrawFunds() public onlyOwner {
        require(ended, "Campaign not ended");
        require(funded, "Goal not reached");
        uint256 amount = totalFunds;
        totalFunds = 0;
        payable(owner).transfer(amount);
        emit FundsWithdrawn(owner, amount);
    }

    function getRefund() public {
        require(ended, "Campaign not ended");
        require(!funded, "Goal reached");
        require(contributions[msg.sender] > 0, "No contribution");
        uint256 amount = contributions[msg.sender];
        contributions[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
    }
}
"#;

        let abi = serde_json::json!([
            {
                "constant": false,
                "inputs": [],
                "name": "contribute",
                "outputs": [],
                "payable": true,
                "stateMutability": "payable",
                "type": "function"
            }
        ]);

        SmartContractTemplate {
            id: "crowdfunding-standard".to_string(),
            name: "Standard Crowdfunding Campaign".to_string(),
            contract_type: "crowdfunding".to_string(),
            blockchain: BlockchainNetwork::Ethereum,
            code: code.to_string(),
            compile_config: CompileConfig {
                compiler_version: "0.8.0".to_string(),
                optimization: 200,
                output_format: "json".to_string(),
            },
            deployment_params: vec!["1000000000000000000".to_string(), "30".to_string()],
            interface: ContractInterface {
                functions: vec![ContractFunction {
                    name: "contribute".to_string(),
                    inputs: vec![],
                    outputs: vec![],
                    function_type: "payable".to_string(),
                    constant: false,
                }],
                events: vec![ContractEvent {
                    name: "Contributed".to_string(),
                    inputs: vec![
                        FunctionParam {
                            name: "contributor".to_string(),
                            param_type: "address".to_string(),
                            indexed: true,
                        },
                        FunctionParam {
                            name: "amount".to_string(),
                            param_type: "uint256".to_string(),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                }],
                abi,
            },
        }
    }

    /// 创建投票模板
    fn create_voting_template(&self) -> SmartContractTemplate {
        let code = r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Voting {
    address public owner;
    string public proposal;
    uint256 public startTimestamp;
    uint256 public endTimestamp;
    bool public ended;

    mapping(address => bool) public hasVoted;
    mapping(uint256 => uint256) public votes;
    uint256 public totalVotes;

    event Voted(address indexed voter, uint256 indexed option);
    event VoteEnded(uint256[] results);

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner");
        _;
    }

    modifier votingActive() {
        require(!ended, "Voting ended");
        require(block.timestamp >= startTimestamp, "Voting not started");
        require(block.timestamp <= endTimestamp, "Voting ended");
        _;
    }

    constructor(string memory _proposal, uint256 _durationHours) {
        owner = msg.sender;
        proposal = _proposal;
        startTimestamp = block.timestamp;
        endTimestamp = block.timestamp + (_durationHours * 1 hours);
        ended = false;
        totalVotes = 0;
    }

    function vote(uint256 option) public votingActive {
        require(!hasVoted[msg.sender], "Already voted");
        require(option < 2, "Invalid option"); // 0 or 1 for yes/no
        hasVoted[msg.sender] = true;
        votes[option]++;
        totalVotes++;
        emit Voted(msg.sender, option);
    }

    function endVoting() public onlyOwner {
        require(!ended, "Voting already ended");
        ended = true;
        uint256[] memory results = new uint256[](2);
        results[0] = votes[0];
        results[1] = votes[1];
        emit VoteEnded(results);
    }

    function getResults() public view returns (uint256[] memory) {
        require(ended, "Voting not ended");
        uint256[] memory results = new uint256[](2);
        results[0] = votes[0];
        results[1] = votes[1];
        return results;
    }
}
"#;

        let abi = serde_json::json!([
            {
                "constant": false,
                "inputs": [{"name": "option", "type": "uint256"}],
                "name": "vote",
                "outputs": [],
                "payable": false,
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]);

        SmartContractTemplate {
            id: "voting-standard".to_string(),
            name: "Standard Voting Contract".to_string(),
            contract_type: "governance".to_string(),
            blockchain: BlockchainNetwork::Ethereum,
            code: code.to_string(),
            compile_config: CompileConfig {
                compiler_version: "0.8.0".to_string(),
                optimization: 200,
                output_format: "json".to_string(),
            },
            deployment_params: vec![
                "Should we upgrade the contract?".to_string(),
                "24".to_string(),
            ],
            interface: ContractInterface {
                functions: vec![ContractFunction {
                    name: "vote".to_string(),
                    inputs: vec![FunctionParam {
                        name: "option".to_string(),
                        param_type: "uint256".to_string(),
                        indexed: false,
                    }],
                    outputs: vec![],
                    function_type: "nonpayable".to_string(),
                    constant: false,
                }],
                events: vec![ContractEvent {
                    name: "Voted".to_string(),
                    inputs: vec![
                        FunctionParam {
                            name: "voter".to_string(),
                            param_type: "address".to_string(),
                            indexed: true,
                        },
                        FunctionParam {
                            name: "option".to_string(),
                            param_type: "uint256".to_string(),
                            indexed: true,
                        },
                    ],
                    anonymous: false,
                }],
                abi,
            },
        }
    }
}

/// 区块链管理器
pub struct BlockchainManager {
    /// 网络配置
    network_configs: HashMap<BlockchainNetwork, NetworkConfig>,
    /// 适配器
    adapters: HashMap<BlockchainNetwork, Arc<dyn BlockchainAdapter>>,
    /// 智能合约模板库
    contract_templates: SmartContractTemplateLibrary,
    /// 事件监听器
    event_listeners: Arc<RwLock<Vec<BlockchainEventListener>>>,
}

impl BlockchainManager {
    /// 创建新的区块链管理器
    pub fn new() -> Self {
        let mut manager = Self {
            network_configs: HashMap::new(),
            adapters: HashMap::new(),
            contract_templates: SmartContractTemplateLibrary::new(),
            event_listeners: Arc::new(RwLock::new(Vec::new())),
        };

        // 初始化网络配置
        manager.initialize_network_configs();
        manager
    }

    /// 初始化网络配置
    fn initialize_network_configs(&mut self) {
        // Ethereum 主网
        self.network_configs.insert(
            BlockchainNetwork::Ethereum,
            NetworkConfig {
                name: "Ethereum Mainnet".to_string(),
                rpc_url: "https://mainnet.infura.io/v3/YOUR_API_KEY".to_string(),
                chain_id: 1,
                native_token: "ETH".to_string(),
                block_time: 15,
                gas_estimate: 21000,
            },
        );

        // BSC 主网
        self.network_configs.insert(
            BlockchainNetwork::BSC,
            NetworkConfig {
                name: "Binance Smart Chain Mainnet".to_string(),
                rpc_url: "https://bsc-dataseed.binance.org/".to_string(),
                chain_id: 56,
                native_token: "BNB".to_string(),
                block_time: 3,
                gas_estimate: 21000,
            },
        );

        // Polygon 主网
        self.network_configs.insert(
            BlockchainNetwork::Polygon,
            NetworkConfig {
                name: "Polygon Mainnet".to_string(),
                rpc_url: "https://polygon-rpc.com".to_string(),
                chain_id: 137,
                native_token: "MATIC".to_string(),
                block_time: 2,
                gas_estimate: 21000,
            },
        );

        // 其他网络配置...
    }

    /// 添加网络配置
    pub fn add_network_config(&mut self, network: BlockchainNetwork, config: NetworkConfig) {
        self.network_configs.insert(network, config);
    }

    /// 获取网络配置
    pub fn get_network_config(&self, network: &BlockchainNetwork) -> Option<&NetworkConfig> {
        self.network_configs.get(network)
    }

    /// 添加适配器
    pub fn add_adapter(&mut self, network: BlockchainNetwork, adapter: Arc<dyn BlockchainAdapter>) {
        self.adapters.insert(network, adapter);
    }

    /// 获取适配器
    pub fn get_adapter(&self, network: &BlockchainNetwork) -> Option<Arc<dyn BlockchainAdapter>> {
        self.adapters.get(network).cloned()
    }

    /// 获取智能合约模板库
    pub fn contract_templates(&self) -> &SmartContractTemplateLibrary {
        &self.contract_templates
    }

    /// 添加事件监听器
    pub async fn add_event_listener(&self, listener: BlockchainEventListener) {
        let mut listeners = self.event_listeners.write().await;
        listeners.push(listener);
    }

    /// 启动所有事件监听器
    pub async fn start_event_listeners(&self) {
        let listeners = self.event_listeners.read().await;
        for listener in listeners.iter() {
            // 启动监听器逻辑
            info!(
                "Starting event listener for {} on {:?}",
                listener.event_name, listener.network
            );
        }
    }

    /// 停止所有事件监听器
    pub async fn stop_event_listeners(&self) {
        let listeners = self.event_listeners.read().await;
        for listener in listeners.iter() {
            // 停止监听器逻辑
            info!(
                "Stopping event listener for {} on {:?}",
                listener.event_name, listener.network
            );
        }
    }
}

/// 创建默认的区块链管理器
pub fn default_blockchain_manager() -> BlockchainManager {
    BlockchainManager::new()
}

/// 示例使用
pub async fn example_usage() {
    let manager = default_blockchain_manager();

    // Get network config
    if let Some(config) = manager.get_network_config(&BlockchainNetwork::Ethereum) {
        println!("Ethereum network config: {:?}", config);
    }

    // Get contract templates
    let templates = manager.contract_templates().get_all_templates();
    println!("\nAvailable contract templates:");
    for template in templates {
        println!("- {}: {}", template.id, template.name);
    }

    // Get templates by type
    let token_templates = manager.contract_templates().get_templates_by_type("token");
    println!("\nToken templates:");
    for template in token_templates {
        println!("- {}", template.name);
    }
}
