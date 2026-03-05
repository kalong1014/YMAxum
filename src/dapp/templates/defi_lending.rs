// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建 DeFi 借贷模板
pub fn create_defi_lending_template() -> DappTemplate {
    let lending_contract = SmartContractTemplate {
        name: "LendingProtocol".to_string(),
        path: "contracts/LendingProtocol.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract LendingProtocol {
    mapping(address => uint256) public deposits;
    mapping(address => uint256) public borrows;
    uint256 public totalDeposits;
    uint256 public totalBorrows;
    uint256 public interestRate;

    event Deposited(address indexed user, uint256 amount);
    event Borrowed(address indexed user, uint256 amount);
    event Repaid(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount);

    constructor(uint256 _interestRate) {
        interestRate = _interestRate;
    }

    function deposit() external payable {
        deposits[msg.sender] += msg.value;
        totalDeposits += msg.value;
        emit Deposited(msg.sender, msg.value);
    }

    function borrow(uint256 amount) external {
        require(amount <= totalDeposits * 80 / 100, "Insufficient liquidity");
        borrows[msg.sender] += amount;
        totalBorrows += amount;
        payable(msg.sender).transfer(amount);
        emit Borrowed(msg.sender, amount);
    }

    function repay() external payable {
        uint256 amount = msg.value;
        require(amount <= borrows[msg.sender], "Amount exceeds borrow balance");
        borrows[msg.sender] -= amount;
        totalBorrows -= amount;
        emit Repaid(msg.sender, amount);
    }

    function withdraw(uint256 amount) external {
        require(amount <= deposits[msg.sender], "Amount exceeds deposit balance");
        require(totalDeposits - amount >= totalBorrows, "Insufficient liquidity");
        deposits[msg.sender] -= amount;
        totalDeposits -= amount;
        payable(msg.sender).transfer(amount);
        emit Withdrawn(msg.sender, amount);
    }
}
"#
        .to_string(),
        deployment_params: vec!["100".to_string()],
    };

    let frontend_template = FrontendTemplate {
        name: "LendingDashboard".to_string(),
        path: "src/components/LendingDashboard.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './LendingDashboard.css';

const LendingDashboard = ({ web3, contract }) => {
  const [depositAmount, setDepositAmount] = useState('');
  const [borrowAmount, setBorrowAmount] = useState('');
  const [userDeposit, setUserDeposit] = useState('0');
  const [userBorrow, setUserBorrow] = useState('0');
  const [totalDeposits, setTotalDeposits] = useState('0');
  const [totalBorrows, setTotalBorrows] = useState('0');
  const [interestRate, setInterestRate] = useState('0');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    if (!web3 || !contract) return;

    try {
      const accounts = await web3.eth.getAccounts();
      const userAddress = accounts[0];

      // Get user data
      const deposit = await contract.methods.deposits(userAddress).call();
      const borrow = await contract.methods.borrows(userAddress).call();
      
      // Get global data
      const totalDep = await contract.methods.totalDeposits().call();
      const totalBor = await contract.methods.totalBorrows().call();
      const rate = await contract.methods.interestRate().call();

      setUserDeposit(web3.utils.fromWei(deposit, 'ether'));
      setUserBorrow(web3.utils.fromWei(borrow, 'ether'));
      setTotalDeposits(web3.utils.fromWei(totalDep, 'ether'));
      setTotalBorrows(web3.utils.fromWei(totalBor, 'ether'));
      setInterestRate(rate);
    } catch (error) {
      console.error('Error loading data:', error);
    }
  };

  const handleDeposit = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !depositAmount) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const amount = web3.utils.toWei(depositAmount, 'ether');

      await contract.methods.deposit().send({
        from: accounts[0],
        value: amount
      });

      alert('Deposit successful!');
      setDepositAmount('');
      loadData();
    } catch (error) {
      console.error('Error depositing:', error);
      alert('Deposit failed!');
    } finally {
      setLoading(false);
    }
  };

  const handleBorrow = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !borrowAmount) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const amount = web3.utils.toWei(borrowAmount, 'ether');

      await contract.methods.borrow(amount).send({
        from: accounts[0]
      });

      alert('Borrow successful!');
      setBorrowAmount('');
      loadData();
    } catch (error) {
      console.error('Error borrowing:', error);
      alert('Borrow failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="lending-dashboard">
      <h2>Lending Protocol</h2>
      
      <div className="stats">
        <div className="stat-item">
          <h3>Your Deposit</h3>
          <p>{userDeposit} ETH</p>
        </div>
        <div className="stat-item">
          <h3>Your Borrow</h3>
          <p>{userBorrow} ETH</p>
        </div>
        <div className="stat-item">
          <h3>Total Deposits</h3>
          <p>{totalDeposits} ETH</p>
        </div>
        <div className="stat-item">
          <h3>Total Borrows</h3>
          <p>{totalBorrows} ETH</p>
        </div>
        <div className="stat-item">
          <h3>Interest Rate</h3>
          <p>{interestRate}%</p>
        </div>
      </div>

      <div className="actions">
        <form onSubmit={handleDeposit} className="action-form">
          <h3>Deposit ETH</h3>
          <input
            type="number"
            step="0.01"
            placeholder="Amount (ETH)"
            value={depositAmount}
            onChange={(e) => setDepositAmount(e.target.value)}
            required
          />
          <button type="submit" disabled={loading}>
            {loading ? 'Depositing...' : 'Deposit'}
          </button>
        </form>

        <form onSubmit={handleBorrow} className="action-form">
          <h3>Borrow ETH</h3>
          <input
            type="number"
            step="0.01"
            placeholder="Amount (ETH)"
            value={borrowAmount}
            onChange={(e) => setBorrowAmount(e.target.value)}
            required
          />
          <button type="submit" disabled={loading}>
            {loading ? 'Borrowing...' : 'Borrow'}
          </button>
        </form>
      </div>
    </div>
  );
};

export default LendingDashboard;
"#
        .to_string(),
        component_type: "dashboard".to_string(),
    };

    let mut config_templates = HashMap::new();
    config_templates.insert(
        "hardhat.config.js".to_string(),
        r#"require('@nomicfoundation/hardhat-toolbox');

module.exports = {
  solidity: '0.8.0',
  networks: {
    hardhat: {},
    localhost: {
      url: 'http://127.0.0.1:8545'
    }
  }
};
"#
        .to_string(),
    );

    let deployment_templates = vec![
        r#"async function main() {
  const [deployer] = await ethers.getSigners();

  console.log('Deploying LendingProtocol with the account:', deployer.address);

  const LendingProtocol = await ethers.getContractFactory('LendingProtocol');
  const lendingProtocol = await LendingProtocol.deploy(100); // 100% interest rate

  await lendingProtocol.deployed();

  console.log('LendingProtocol deployed to:', lendingProtocol.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
"#
        .to_string(),
    ];

    DappTemplate {
        id: "defi-lending".to_string(),
        name: "DeFi Lending Protocol".to_string(),
        description:
            "A decentralized lending protocol allowing users to deposit and borrow ETH"
                .to_string(),
        dapp_type: "DeFi".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![lending_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
