// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建房地产模板
pub fn create_real_estate_template() -> DappTemplate {
    let real_estate_contract = SmartContractTemplate {
        name: "RealEstateToken".to_string(),
        path: "contracts/RealEstateToken.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract RealEstateToken is ERC721, Ownable {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    struct Property {
        uint256 tokenId;
        string propertyName;
        string location;
        uint256 price;
        string description;
        bool forSale;
        address owner;
    }

    mapping(uint256 => Property) public properties;

    event PropertyMinted(uint256 tokenId, string propertyName, address owner);
    event PropertyListed(uint256 tokenId, uint256 price);
    event PropertySold(uint256 tokenId, address buyer, uint256 price);

    constructor() ERC721("RealEstateToken", "RET") {}

    function mintProperty(string memory propertyName, string memory location, uint256 price, string memory description) public onlyOwner {
        _tokenIds.increment();
        uint256 newTokenId = _tokenIds.current();
        
        _safeMint(msg.sender, newTokenId);
        
        properties[newTokenId] = Property({
            tokenId: newTokenId,
            propertyName: propertyName,
            location: location,
            price: price,
            description: description,
            forSale: true,
            owner: msg.sender
        });
        
        emit PropertyMinted(newTokenId, propertyName, msg.sender);
    }

    function buyProperty(uint256 tokenId) public payable {
        Property memory property = properties[tokenId];
        require(property.forSale, "Property not for sale");
        require(msg.value >= property.price, "Insufficient funds");
        require(ownerOf(tokenId) != msg.sender, "Cannot buy your own property");

        address seller = ownerOf(tokenId);
        _transfer(seller, msg.sender, tokenId);
        properties[tokenId].owner = msg.sender;
        properties[tokenId].forSale = false;

        payable(seller).transfer(msg.value);

        emit PropertySold(tokenId, msg.sender, msg.value);
    }

    function listProperty(uint256 tokenId, uint256 price) public {
        require(ownerOf(tokenId) == msg.sender, "Not the owner");
        properties[tokenId].price = price;
        properties[tokenId].forSale = true;
        emit PropertyListed(tokenId, price);
    }

    function getProperty(uint256 tokenId) public view returns (Property memory) {
        return properties[tokenId];
    }

    function getAllProperties() public view returns (Property[] memory) {
        uint256 totalProperties = _tokenIds.current();
        Property[] memory result = new Property[](totalProperties);
        
        for (uint256 i = 1; i <= totalProperties; i++) {
            result[i-1] = properties[i];
        }
        
        return result;
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "RealEstateToken".to_string(),
        path: "src/components/RealEstateToken.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './RealEstateToken.css';

const RealEstateToken = ({ web3, contract }) => {
  const [properties, setProperties] = useState([]);
  const [propertyName, setPropertyName] = useState('');
  const [location, setLocation] = useState('');
  const [price, setPrice] = useState('');
  const [description, setDescription] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadProperties();
  }, []);

  const loadProperties = async () => {
    if (!web3 || !contract) return;

    try {
      const allProperties = await contract.methods.getAllProperties().call();
      setProperties(allProperties);
    } catch (error) {
      console.error('Error loading properties:', error);
    }
  };

  const mintProperty = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !propertyName || !location || !price || !description) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const ethPrice = web3.utils.toWei(price, 'ether');

      await contract.methods.mintProperty(propertyName, location, ethPrice, description).send({
        from: accounts[0]
      });

      alert('Property minted successfully!');
      setPropertyName('');
      setLocation('');
      setPrice('');
      setDescription('');
      loadProperties();
    } catch (error) {
      console.error('Error minting property:', error);
      alert('Property minting failed!');
    } finally {
      setLoading(false);
    }
  };

  const buyProperty = async (tokenId, price) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.buyProperty(tokenId).send({
        from: accounts[0],
        value: price
      });

      alert('Property purchased successfully!');
      loadProperties();
    } catch (error) {
      console.error('Error buying property:', error);
      alert('Property purchase failed!');
    } finally {
      setLoading(false);
    }
  };

  const listProperty = async (tokenId) => {
    if (!web3 || !contract) return;

    const newPrice = prompt('Enter new price (ETH):');
    if (!newPrice) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const ethPrice = web3.utils.toWei(newPrice, 'ether');

      await contract.methods.listProperty(tokenId, ethPrice).send({
        from: accounts[0]
      });

      alert('Property listed successfully!');
      loadProperties();
    } catch (error) {
      console.error('Error listing property:', error);
      alert('Property listing failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="real-estate-token">
      <h2>Real Estate Token</h2>

      <div className="mint-property">
        <h3>Mint Property</h3>
        <form onSubmit={mintProperty}>
          <div className="form-group">
            <label>Property Name</label>
            <input
              type="text"
              placeholder="Property name"
              value={propertyName}
              onChange={(e) => setPropertyName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Location</label>
            <input
              type="text"
              placeholder="Location"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Price (ETH)</label>
            <input
              type="number"
              step="0.01"
              placeholder="Price"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              placeholder="Property description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Minting...' : 'Mint Property'}
          </button>
        </form>
      </div>

      <div className="properties">
        <h3>Properties</h3>
        {properties.length === 0 ? (
          <p>No properties yet. Mint the first one!</p>
        ) : (
          properties.map((property) => (
            <div key={property.tokenId} className="property-card">
              <h4>{property.propertyName}</h4>
              <p>Location: {property.location}</p>
              <p>Price: {web3 ? web3.utils.fromWei(property.price, 'ether') : 0} ETH</p>
              <p>Description: {property.description}</p>
              <p>For Sale: {property.forSale ? 'Yes' : 'No'}</p>
              <p>Owner: {property.owner}</p>
              
              <div className="property-actions">
                {property.forSale ? (
                  <button 
                    onClick={() => buyProperty(property.tokenId, property.price)}
                    disabled={loading}
                  >
                    {loading ? 'Buying...' : 'Buy Property'}
                  </button>
                ) : (
                  <button 
                    onClick={() => listProperty(property.tokenId)}
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

export default RealEstateToken;
"#
        .to_string(),
        component_type: "real-estate".to_string(),
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

  console.log('Deploying RealEstateToken with the account:', deployer.address);

  const RealEstateToken = await ethers.getContractFactory('RealEstateToken');
  const realEstateToken = await RealEstateToken.deploy();

  await realEstateToken.deployed();

  console.log('RealEstateToken deployed to:', realEstateToken.address);
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
        id: "real-estate".to_string(),
        name: "Real Estate Token".to_string(),
        description: "A decentralized real estate tokenization system".to_string(),
        dapp_type: "Real Estate".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![real_estate_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
