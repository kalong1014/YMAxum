// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建供应链模板
pub fn create_supply_chain_template() -> DappTemplate {
    let supply_chain_contract = SmartContractTemplate {
        name: "SupplyChain".to_string(),
        path: "contracts/SupplyChain.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SupplyChain {
    struct Product {
        uint256 id;
        string name;
        string description;
        address manufacturer;
        uint256 manufactureDate;
        string currentLocation;
        bool verified;
    }

    mapping(uint256 => Product) public products;
    mapping(uint256 => mapping(address => bool)) public productHistory;
    uint256 public productCount;

    event ProductCreated(uint256 id, string name, address manufacturer);
    event ProductVerified(uint256 id, address verifier);
    event ProductLocationUpdated(uint256 id, string location);

    function createProduct(string memory name, string memory description, string memory location) public {
        uint256 id = productCount;
        productCount++;

        products[id] = Product({
            id: id,
            name: name,
            description: description,
            manufacturer: msg.sender,
            manufactureDate: block.timestamp,
            currentLocation: location,
            verified: false
        });

        productHistory[id][msg.sender] = true;
        emit ProductCreated(id, name, msg.sender);
    }

    function updateLocation(uint256 id, string memory newLocation) public {
        require(productHistory[id][msg.sender], "Not authorized to update this product");
        products[id].currentLocation = newLocation;
        emit ProductLocationUpdated(id, newLocation);
    }

    function verifyProduct(uint256 id) public {
        require(productHistory[id][msg.sender], "Not authorized to verify this product");
        products[id].verified = true;
        emit ProductVerified(id, msg.sender);
    }

    function addParticipant(uint256 id, address participant) public {
        require(productHistory[id][msg.sender], "Not authorized to add participants");
        productHistory[id][participant] = true;
    }

    function getProduct(uint256 id) public view returns (Product memory) {
        return products[id];
    }

    function isParticipant(uint256 id, address participant) public view returns (bool) {
        return productHistory[id][participant];
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "SupplyChain".to_string(),
        path: "src/components/SupplyChain.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './SupplyChain.css';

const SupplyChain = ({ web3, contract }) => {
  const [products, setProducts] = useState([]);
  const [productName, setProductName] = useState('');
  const [productDescription, setProductDescription] = useState('');
  const [productLocation, setProductLocation] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadProducts();
  }, []);

  const loadProducts = async () => {
    if (!web3 || !contract) return;

    try {
      const productCount = await contract.methods.productCount().call();
      const loadedProducts = [];

      for (let i = 0; i < productCount; i++) {
        const product = await contract.methods.getProduct(i).call();
        loadedProducts.push(product);
      }

      setProducts(loadedProducts);
    } catch (error) {
      console.error('Error loading products:', error);
    }
  };

  const createProduct = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !productName || !productDescription || !productLocation) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.createProduct(productName, productDescription, productLocation).send({
        from: accounts[0]
      });

      alert('Product created successfully!');
      setProductName('');
      setProductDescription('');
      setProductLocation('');
      loadProducts();
    } catch (error) {
      console.error('Error creating product:', error);
      alert('Product creation failed!');
    } finally {
      setLoading(false);
    }
  };

  const updateLocation = async (id, newLocation) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.updateLocation(id, newLocation).send({
        from: accounts[0]
      });

      alert('Location updated successfully!');
      loadProducts();
    } catch (error) {
      console.error('Error updating location:', error);
      alert('Location update failed!');
    } finally {
      setLoading(false);
    }
  };

  const verifyProduct = async (id) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.verifyProduct(id).send({
        from: accounts[0]
      });

      alert('Product verified successfully!');
      loadProducts();
    } catch (error) {
      console.error('Error verifying product:', error);
      alert('Product verification failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="supply-chain">
      <h2>Supply Chain</h2>

      <div className="create-product">
        <h3>Create Product</h3>
        <form onSubmit={createProduct}>
          <div className="form-group">
            <label>Product Name</label>
            <input
              type="text"
              placeholder="Product name"
              value={productName}
              onChange={(e) => setProductName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              placeholder="Product description"
              value={productDescription}
              onChange={(e) => setProductDescription(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Initial Location</label>
            <input
              type="text"
              placeholder="Location"
              value={productLocation}
              onChange={(e) => setProductLocation(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Creating...' : 'Create Product'}
          </button>
        </form>
      </div>

      <div className="products">
        <h3>Products</h3>
        {products.length === 0 ? (
          <p>No products yet. Create the first one!</p>
        ) : (
          products.map((product) => (
            <div key={product.id} className="product-card">
              <h4>{product.name}</h4>
              <p>{product.description}</p>
              <p>Manufacturer: {product.manufacturer}</p>
              <p>Manufacture Date: {new Date(product.manufactureDate * 1000).toLocaleString()}</p>
              <p>Current Location: {product.currentLocation}</p>
              <p>Verified: {product.verified ? 'Yes' : 'No'}</p>
              
              <div className="product-actions">
                <button 
                  onClick={() => {
                    const newLocation = prompt('Enter new location:');
                    if (newLocation) updateLocation(product.id, newLocation);
                  }}
                  disabled={loading}
                >
                  {loading ? 'Updating...' : 'Update Location'}
                </button>
                {!product.verified && (
                  <button 
                    onClick={() => verifyProduct(product.id)}
                    disabled={loading}
                  >
                    {loading ? 'Verifying...' : 'Verify Product'}
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

export default SupplyChain;
"#
        .to_string(),
        component_type: "supply-chain".to_string(),
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

  console.log('Deploying SupplyChain with the account:', deployer.address);

  const SupplyChain = await ethers.getContractFactory('SupplyChain');
  const supplyChain = await SupplyChain.deploy();

  await supplyChain.deployed();

  console.log('SupplyChain deployed to:', supplyChain.address);
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
        id: "supply-chain".to_string(),
        name: "Supply Chain".to_string(),
        description: "A decentralized supply chain tracking system".to_string(),
        dapp_type: "Supply Chain".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![supply_chain_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
