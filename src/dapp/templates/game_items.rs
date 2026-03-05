// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建游戏物品模板
pub fn create_game_items_template() -> DappTemplate {
    let game_contract = SmartContractTemplate {
        name: "GameItems".to_string(),
        path: "contracts/GameItems.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract GameItems is ERC1155, Ownable {
    uint256 public constant SWORD = 0;
    uint256 public constant SHIELD = 1;
    uint256 public constant POTION = 2;
    uint256 public constant GOLD = 3;

    mapping(uint256 => string) public itemNames;
    mapping(uint256 => uint256) public itemSupply;

    event ItemMinted(address indexed to, uint256 indexed id, uint256 amount);
    event ItemTransferred(address indexed from, address indexed to, uint256 indexed id, uint256 amount);

    constructor() ERC1155("https://game.example/api/items/{id}.json") {
        itemNames[SWORD] = "Sword";
        itemNames[SHIELD] = "Shield";
        itemNames[POTION] = "Potion";
        itemNames[GOLD] = "Gold";

        // Mint initial items to contract owner
        _mint(msg.sender, SWORD, 1, "");
        _mint(msg.sender, SHIELD, 1, "");
        _mint(msg.sender, POTION, 10, "");
        _mint(msg.sender, GOLD, 100, "");

        itemSupply[SWORD] = 1;
        itemSupply[SHIELD] = 1;
        itemSupply[POTION] = 10;
        itemSupply[GOLD] = 100;
    }

    function mintItem(address to, uint256 id, uint256 amount) public onlyOwner {
        _mint(to, id, amount, "");
        itemSupply[id] += amount;
        emit ItemMinted(to, id, amount);
    }

    function transferItem(address to, uint256 id, uint256 amount) public {
        require(balanceOf(msg.sender, id) >= amount, "Insufficient balance");
        safeTransferFrom(msg.sender, to, id, amount, "");
        emit ItemTransferred(msg.sender, to, id, amount);
    }

    function getItemName(uint256 id) public view returns (string memory) {
        return itemNames[id];
    }

    function getItemSupply(uint256 id) public view returns (uint256) {
        return itemSupply[id];
    }

    function getInventory(address owner) public view returns (uint256[] memory, uint256[] memory) {
        uint256[] memory ids = new uint256[](4);
        uint256[] memory balances = new uint256[](4);

        ids[0] = SWORD;
        ids[1] = SHIELD;
        ids[2] = POTION;
        ids[3] = GOLD;

        balances[0] = balanceOf(owner, SWORD);
        balances[1] = balanceOf(owner, SHIELD);
        balances[2] = balanceOf(owner, POTION);
        balances[3] = balanceOf(owner, GOLD);

        return (ids, balances);
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "GameInventory".to_string(),
        path: "src/components/GameInventory.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './GameInventory.css';

const GameInventory = ({ web3, contract }) => {
  const [inventory, setInventory] = useState([]);
  const [transferAddress, setTransferAddress] = useState('');
  const [transferItemId, setTransferItemId] = useState('0');
  const [transferAmount, setTransferAmount] = useState('1');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadInventory();
  }, []);

  const loadInventory = async () => {
    if (!web3 || !contract) return;

    try {
      const accounts = await web3.eth.getAccounts();
      const userAddress = accounts[0];

      // Get inventory
      const [ids, balances] = await contract.methods.getInventory(userAddress).call();

      // Get item names
      const items = [];
      for (let i = 0; i < ids.length; i++) {
        const name = await contract.methods.getItemName(ids[i]).call();
        items.push({
          id: ids[i],
          name,
          balance: balances[i]
        });
      }

      setInventory(items);
    } catch (error) {
      console.error('Error loading inventory:', error);
    }
  };

  const transferItem = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !transferAddress || !transferItemId || !transferAmount) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.transferItem(transferAddress, transferItemId, transferAmount).send({
        from: accounts[0]
      });

      alert('Item transferred successfully!');
      setTransferAddress('');
      setTransferItemId('0');
      setTransferAmount('1');
      loadInventory();
    } catch (error) {
      console.error('Error transferring item:', error);
      alert('Item transfer failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="game-inventory">
      <h2>Game Inventory</h2>

      <div className="inventory">
        <h3>Your Items</h3>
        <div className="items-grid">
          {inventory.map((item) => (
            <div key={item.id} className="item-card">
              <div className="item-icon">
                {/* Item icon would go here */}
                <div className="icon-placeholder">{item.name.charAt(0)}</div>
              </div>
              <div className="item-info">
                <h4>{item.name}</h4>
                <p>Quantity: {item.balance}</p>
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="transfer-form">
        <h3>Transfer Item</h3>
        <form onSubmit={transferItem}>
          <div className="form-group">
            <label>Recipient Address</label>
            <input
              type="text"
              placeholder="0x..."
              value={transferAddress}
              onChange={(e) => setTransferAddress(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Item</label>
            <select
              value={transferItemId}
              onChange={(e) => setTransferItemId(e.target.value)}
              required
            >
              <option value="0">Sword</option>
              <option value="1">Shield</option>
              <option value="2">Potion</option>
              <option value="3">Gold</option>
            </select>
          </div>
          <div className="form-group">
            <label>Amount</label>
            <input
              type="number"
              min="1"
              value={transferAmount}
              onChange={(e) => setTransferAmount(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Transferring...' : 'Transfer'}
          </button>
        </form>
      </div>
    </div>
  );
};

export default GameInventory;
"#
        .to_string(),
        component_type: "inventory".to_string(),
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
  },
  dependencies: {
    '@openzeppelin/contracts': '^4.8.0'
  }
};
"#
        .to_string(),
    );

    let deployment_templates = vec![
        r#"async function main() {
  const [deployer] = await ethers.getSigners();

  console.log('Deploying GameItems with the account:', deployer.address);

  const GameItems = await ethers.getContractFactory('GameItems');
  const gameItems = await GameItems.deploy();

  await gameItems.deployed();

  console.log('GameItems deployed to:', gameItems.address);
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
        id: "game-items".to_string(),
        name: "Game Items".to_string(),
        description: "A game item system using ERC-1155 tokens".to_string(),
        dapp_type: "Game".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![game_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
