// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建能源管理模板
pub fn create_energy_template() -> DappTemplate {
    let energy_contract = SmartContractTemplate {
        name: "EnergyTrading".to_string(),
        path: "contracts/EnergyTrading.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract EnergyTrading {
    struct EnergyToken {
        uint256 id;
        string producer;
        uint256 amount;
        uint256 price;
        bool forSale;
        address owner;
    }

    mapping(uint256 => EnergyToken) public energyTokens;
    uint256 public tokenCount;

    event EnergyProduced(uint256 id, string producer, uint256 amount, address owner);
    event EnergyListed(uint256 id, uint256 price);
    event EnergySold(uint256 id, address buyer, uint256 amount, uint256 price);

    function produceEnergy(string memory producer, uint256 amount, uint256 price) public {
        uint256 id = tokenCount;
        tokenCount++;

        energyTokens[id] = EnergyToken({
            id: id,
            producer: producer,
            amount: amount,
            price: price,
            forSale: true,
            owner: msg.sender
        });

        emit EnergyProduced(id, producer, amount, msg.sender);
    }

    function buyEnergy(uint256 id) public payable {
        EnergyToken memory token = energyTokens[id];
        require(token.forSale, "Energy not for sale");
        require(msg.value >= token.price * token.amount, "Insufficient funds");
        require(token.owner != msg.sender, "Cannot buy your own energy");

        address seller = token.owner;
        energyTokens[id].owner = msg.sender;
        energyTokens[id].forSale = false;

        payable(seller).transfer(msg.value);

        emit EnergySold(id, msg.sender, token.amount, token.price);
    }

    function listEnergy(uint256 id, uint256 price) public {
        require(energyTokens[id].owner == msg.sender, "Not the owner");
        energyTokens[id].price = price;
        energyTokens[id].forSale = true;
        emit EnergyListed(id, price);
    }

    function getEnergyToken(uint256 id) public view returns (EnergyToken memory) {
        return energyTokens[id];
    }

    function getAllEnergyTokens() public view returns (EnergyToken[] memory) {
        EnergyToken[] memory result = new EnergyToken[](tokenCount);
        
        for (uint256 i = 0; i < tokenCount; i++) {
            result[i] = energyTokens[i];
        }
        
        return result;
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "EnergyTrading".to_string(),
        path: "src/components/EnergyTrading.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './EnergyTrading.css';

const EnergyTrading = ({ web3, contract }) => {
  const [energyTokens, setEnergyTokens] = useState([]);
  const [producer, setProducer] = useState('');
  const [amount, setAmount] = useState('');
  const [price, setPrice] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadEnergyTokens();
  }, []);

  const loadEnergyTokens = async () => {
    if (!web3 || !contract) return;

    try {
      const allTokens = await contract.methods.getAllEnergyTokens().call();
      setEnergyTokens(allTokens);
    } catch (error) {
      console.error('Error loading energy tokens:', error);
    }
  };

  const produceEnergy = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !producer || !amount || !price) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const ethPrice = web3.utils.toWei(price, 'ether');

      await contract.methods.produceEnergy(producer, amount, ethPrice).send({
        from: accounts[0]
      });

      alert('Energy produced successfully!');
      setProducer('');
      setAmount('');
      setPrice('');
      loadEnergyTokens();
    } catch (error) {
      console.error('Error producing energy:', error);
      alert('Energy production failed!');
    } finally {
      setLoading(false);
    }
  };

  const buyEnergy = async (id, price, amount) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const totalPrice = web3.utils.toBN(price).mul(web3.utils.toBN(amount));

      await contract.methods.buyEnergy(id).send({
        from: accounts[0],
        value: totalPrice.toString()
      });

      alert('Energy purchased successfully!');
      loadEnergyTokens();
    } catch (error) {
      console.error('Error buying energy:', error);
      alert('Energy purchase failed!');
    } finally {
      setLoading(false);
    }
  };

  const listEnergy = async (id) => {
    if (!web3 || !contract) return;

    const newPrice = prompt('Enter new price per unit (ETH):');
    if (!newPrice) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const ethPrice = web3.utils.toWei(newPrice, 'ether');

      await contract.methods.listEnergy(id, ethPrice).send({
        from: accounts[0]
      });

      alert('Energy listed successfully!');
      loadEnergyTokens();
    } catch (error) {
      console.error('Error listing energy:', error);
      alert('Energy listing failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="energy-trading">
      <h2>Energy Trading</h2>

      <div className="produce-energy">
        <h3>Produce Energy</h3>
        <form onSubmit={produceEnergy}>
          <div className="form-group">
            <label>Producer</label>
            <input
              type="text"
              placeholder="Producer name"
              value={producer}
              onChange={(e) => setProducer(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Amount (kWh)</label>
            <input
              type="number"
              step="1"
              placeholder="Amount"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Price per kWh (ETH)</label>
            <input
              type="number"
              step="0.0001"
              placeholder="Price"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Producing...' : 'Produce Energy'}
          </button>
        </form>
      </div>

      <div className="energy-tokens">
        <h3>Energy Tokens</h3>
        {energyTokens.length === 0 ? (
          <p>No energy tokens yet. Produce the first one!</p>
        ) : (
          energyTokens.map((token) => (
            <div key={token.id} className="token-card">
              <h4>Energy Token #{token.id}</h4>
              <p>Producer: {token.producer}</p>
              <p>Amount: {token.amount} kWh</p>
              <p>Price per kWh: {web3 ? web3.utils.fromWei(token.price, 'ether') : 0} ETH</p>
              <p>For Sale: {token.forSale ? 'Yes' : 'No'}</p>
              <p>Owner: {token.owner}</p>
              
              <div className="token-actions">
                {token.forSale ? (
                  <button 
                    onClick={() => buyEnergy(token.id, token.price, token.amount)}
                    disabled={loading}
                  >
                    {loading ? 'Buying...' : 'Buy Energy'}
                  </button>
                ) : (
                  <button 
                    onClick={() => listEnergy(token.id)}
                    disabled={loading}
                  >
                    {loading ? 'Listing...' : 'List for Sale'}
                  </button>
                )}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default EnergyTrading;
"#
        .to_string(),
        component_type: "energy".to_string(),
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

  console.log('Deploying EnergyTrading with the account:', deployer.address);

  const EnergyTrading = await ethers.getContractFactory('EnergyTrading');
  const energyTrading = await EnergyTrading.deploy();

  await energyTrading.deployed();

  console.log('EnergyTrading deployed to:', energyTrading.address);
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
        id: "energy".to_string(),
        name: "Energy Trading".to_string(),
        description: "A decentralized energy trading system".to_string(),
        dapp_type: "Energy".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![energy_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
