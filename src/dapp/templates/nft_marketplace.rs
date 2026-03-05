// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建 NFT 市场模板
pub fn create_nft_marketplace_template() -> DappTemplate {
    let nft_contract = SmartContractTemplate {
        name: "NFTMarketplace".to_string(),
        path: "contracts/NFTMarketplace.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract NFTMarketplace is ERC721, Ownable {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    struct NFT {
        uint256 tokenId;
        address creator;
        string uri;
        uint256 price;
        bool forSale;
    }

    mapping(uint256 => NFT) public nfts;
    mapping(uint256 => address) public tokenOwners;

    event NFTCreated(uint256 tokenId, address creator, string uri, uint256 price);
    event NFTListed(uint256 tokenId, uint256 price);
    event NFTSold(uint256 tokenId, address buyer, uint256 price);

    constructor() ERC721("MarketplaceNFT", "MNFT") {}

    function createNFT(string memory uri, uint256 price) public returns (uint256) {
        _tokenIds.increment();
        uint256 newTokenId = _tokenIds.current();
        
        _safeMint(msg.sender, newTokenId);
        
        nfts[newTokenId] = NFT({
            tokenId: newTokenId,
            creator: msg.sender,
            uri: uri,
            price: price,
            forSale: true
        });
        
        tokenOwners[newTokenId] = msg.sender;
        
        emit NFTCreated(newTokenId, msg.sender, uri, price);
        return newTokenId;
    }

    function buyNFT(uint256 tokenId) public payable {
        NFT memory nft = nfts[tokenId];
        require(nft.forSale, "NFT not for sale");
        require(msg.value >= nft.price, "Insufficient funds");
        require(tokenOwners[tokenId] != msg.sender, "Cannot buy your own NFT");

        address seller = tokenOwners[tokenId];
        tokenOwners[tokenId] = msg.sender;
        nfts[tokenId].forSale = false;

        _transfer(seller, msg.sender, tokenId);
        payable(seller).transfer(msg.value);

        emit NFTSold(tokenId, msg.sender, msg.value);
    }

    function listNFT(uint256 tokenId, uint256 price) public {
        require(tokenOwners[tokenId] == msg.sender, "Not the owner");
        nfts[tokenId].price = price;
        nfts[tokenId].forSale = true;
        emit NFTListed(tokenId, price);
    }

    function getNFT(uint256 tokenId) public view returns (NFT memory) {
        return nfts[tokenId];
    }

    function getAllNFTs() public view returns (NFT[] memory) {
        uint256 totalNFTs = _tokenIds.current();
        NFT[] memory result = new NFT[](totalNFTs);
        
        for (uint256 i = 1; i <= totalNFTs; i++) {
            result[i-1] = nfts[i];
        }
        
        return result;
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "NFTMarketplace".to_string(),
        path: "src/components/NFTMarketplace.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './NFTMarketplace.css';

const NFTMarketplace = ({ web3, contract }) => {
  const [nfts, setNfts] = useState([]);
  const [uri, setUri] = useState('');
  const [price, setPrice] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadNFTs();
  }, []);

  const loadNFTs = async () => {
    if (!web3 || !contract) return;

    try {
      const allNFTs = await contract.methods.getAllNFTs().call();
      setNfts(allNFTs);
    } catch (error) {
      console.error('Error loading NFTs:', error);
    }
  };

  const createNFT = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !uri || !price) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();
      const ethPrice = web3.utils.toWei(price, 'ether');

      await contract.methods.createNFT(uri, ethPrice).send({
        from: accounts[0]
      });

      alert('NFT created successfully!');
      setUri('');
      setPrice('');
      loadNFTs();
    } catch (error) {
      console.error('Error creating NFT:', error);
      alert('NFT creation failed!');
    } finally {
      setLoading(false);
    }
  };

  const buyNFT = async (tokenId, price) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.buyNFT(tokenId).send({
        from: accounts[0],
        value: price
      });

      alert('NFT purchased successfully!');
      loadNFTs();
    } catch (error) {
      console.error('Error buying NFT:', error);
      alert('NFT purchase failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="nft-marketplace">
      <h2>NFT Marketplace</h2>

      <div className="create-nft">
        <h3>Create NFT</h3>
        <form onSubmit={createNFT}>
          <input
            type="text"
            placeholder="NFT Metadata URI"
            value={uri}
            onChange={(e) => setUri(e.target.value)}
            required
          />
          <input
            type="number"
            step="0.01"
            placeholder="Price (ETH)"
            value={price}
            onChange={(e) => setPrice(e.target.value)}
            required
          />
          <button type="submit" disabled={loading}>
            {loading ? 'Creating...' : 'Create NFT'}
          </button>
        </form>
      </div>

      <div className="nft-grid">
        {nfts.map((nft) => (
          <div key={nft.tokenId} className="nft-card">
            <div className="nft-image">
              <img src={nft.uri} alt={`NFT ${nft.tokenId}`} />
            </div>
            <div className="nft-info">
              <h4>NFT #{nft.tokenId}</h4>
              <p>Creator: {nft.creator}</p>
              <p>Price: {web3 ? web3.utils.fromWei(nft.price, 'ether') : 0} ETH</p>
              {nft.forSale ? (
                <button 
                  onClick={() => buyNFT(nft.tokenId, nft.price)}
                  disabled={loading}
                >
                  {loading ? 'Buying...' : 'Buy NFT'}
                </button>
              ) : (
                <button disabled>Sold</button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default NFTMarketplace;
"#
        .to_string(),
        component_type: "marketplace".to_string(),
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

  console.log('Deploying NFTMarketplace with the account:', deployer.address);

  const NFTMarketplace = await ethers.getContractFactory('NFTMarketplace');
  const nftMarketplace = await NFTMarketplace.deploy();

  await nftMarketplace.deployed();

  console.log('NFTMarketplace deployed to:', nftMarketplace.address);
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
        id: "nft-marketplace".to_string(),
        name: "NFT Marketplace".to_string(),
        description: "A decentralized marketplace for creating and trading NFTs".to_string(),
        dapp_type: "NFT".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![nft_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
