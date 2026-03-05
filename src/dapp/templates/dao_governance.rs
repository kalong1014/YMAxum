// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建 DAO 治理模板
pub fn create_dao_governance_template() -> DappTemplate {
    let dao_contract = SmartContractTemplate {
        name: "DAOGovernance".to_string(),
        path: "contracts/DAOGovernance.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract DAOGovernance is ERC20, Ownable {
    struct Proposal {
        uint256 id;
        string description;
        uint256 voteCount;
        bool executed;
        uint256 endTime;
    }

    mapping(uint256 => Proposal) public proposals;
    mapping(uint256 => mapping(address => bool)) public votes;
    uint256 public proposalCount;
    uint256 public votingPeriod;

    event ProposalCreated(uint256 id, string description, uint256 endTime);
    event Voted(uint256 proposalId, address voter, uint256 votes);
    event ProposalExecuted(uint256 id, bool executed);

    constructor(uint256 _initialSupply, uint256 _votingPeriod) ERC20("DAO Token", "DAOT") {
        _mint(msg.sender, _initialSupply);
        votingPeriod = _votingPeriod;
    }

    function createProposal(string memory description) public {
        require(balanceOf(msg.sender) > 0, "Must hold DAO tokens to create proposals");
        
        uint256 id = proposalCount;
        proposalCount++;
        
        proposals[id] = Proposal({
            id: id,
            description: description,
            voteCount: 0,
            executed: false,
            endTime: block.timestamp + votingPeriod
        });
        
        emit ProposalCreated(id, description, proposals[id].endTime);
    }

    function vote(uint256 proposalId) public {
        require(balanceOf(msg.sender) > 0, "Must hold DAO tokens to vote");
        require(!votes[proposalId][msg.sender], "Already voted on this proposal");
        require(block.timestamp < proposals[proposalId].endTime, "Voting period ended");
        require(!proposals[proposalId].executed, "Proposal already executed");
        
        uint256 voterBalance = balanceOf(msg.sender);
        proposals[proposalId].voteCount += voterBalance;
        votes[proposalId][msg.sender] = true;
        
        emit Voted(proposalId, msg.sender, voterBalance);
    }

    function executeProposal(uint256 proposalId) public {
        require(block.timestamp >= proposals[proposalId].endTime, "Voting period not ended");
        require(!proposals[proposalId].executed, "Proposal already executed");
        
        proposals[proposalId].executed = true;
        emit ProposalExecuted(proposalId, true);
    }

    function getProposal(uint256 proposalId) public view returns (Proposal memory) {
        return proposals[proposalId];
    }

    function getAllProposals() public view returns (Proposal[] memory) {
        Proposal[] memory result = new Proposal[](proposalCount);
        
        for (uint256 i = 0; i < proposalCount; i++) {
            result[i] = proposals[i];
        }
        
        return result;
    }
}
"#
        .to_string(),
        deployment_params: vec!["1000000000000000000000".to_string(), "86400".to_string()],
    };

    let frontend_template = FrontendTemplate {
        name: "DAOGovernance".to_string(),
        path: "src/components/DAOGovernance.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './DAOGovernance.css';

const DAOGovernance = ({ web3, contract }) => {
  const [proposals, setProposals] = useState([]);
  const [proposalDescription, setProposalDescription] = useState('');
  const [loading, setLoading] = useState(false);
  const [tokenBalance, setTokenBalance] = useState('0');

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    if (!web3 || !contract) return;

    try {
      const accounts = await web3.eth.getAccounts();
      const userAddress = accounts[0];

      // Get token balance
      const balance = await contract.methods.balanceOf(userAddress).call();
      setTokenBalance(web3.utils.fromWei(balance, 'ether'));

      // Get proposals
      const proposalCount = await contract.methods.proposalCount().call();
      const loadedProposals = [];

      for (let i = 0; i < proposalCount; i++) {
        const proposal = await contract.methods.getProposal(i).call();
        const hasVoted = await contract.methods.votes(i, userAddress).call();
        loadedProposals.push({ ...proposal, hasVoted });
      }

      setProposals(loadedProposals);
    } catch (error) {
      console.error('Error loading data:', error);
    }
  };

  const createProposal = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !proposalDescription) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.createProposal(proposalDescription).send({
        from: accounts[0]
      });

      alert('Proposal created successfully!');
      setProposalDescription('');
      loadData();
    } catch (error) {
      console.error('Error creating proposal:', error);
      alert('Proposal creation failed!');
    } finally {
      setLoading(false);
    }
  };

  const voteOnProposal = async (proposalId) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.vote(proposalId).send({
        from: accounts[0]
      });

      alert('Vote submitted successfully!');
      loadData();
    } catch (error) {
      console.error('Error voting:', error);
      alert('Voting failed!');
    } finally {
      setLoading(false);
    }
  };

  const executeProposal = async (proposalId) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.executeProposal(proposalId).send({
        from: accounts[0]
      });

      alert('Proposal executed successfully!');
      loadData();
    } catch (error) {
      console.error('Error executing proposal:', error);
      alert('Proposal execution failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="dao-governance">
      <h2>DAO Governance</h2>
      
      <div className="user-info">
        <h3>Your DAO Token Balance</h3>
        <p>{tokenBalance} DAOT</p>
      </div>

      <div className="create-proposal">
        <h3>Create Proposal</h3>
        <form onSubmit={createProposal}>
          <textarea
            placeholder="Proposal description"
            value={proposalDescription}
            onChange={(e) => setProposalDescription(e.target.value)}
            required
          />
          <button type="submit" disabled={loading}>
            {loading ? 'Creating...' : 'Create Proposal'}
          </button>
        </form>
      </div>

      <div className="proposals">
        <h3>Active Proposals</h3>
        {proposals.length === 0 ? (
          <p>No proposals yet. Create the first one!</p>
        ) : (
          proposals.map((proposal) => (
            <div key={proposal.id} className="proposal-card">
              <h4>Proposal #{proposal.id}</h4>
              <p className="description">{proposal.description}</p>
              <p className="vote-count">Votes: {web3 ? web3.utils.fromWei(proposal.voteCount, 'ether') : 0} DAOT</p>
              <p className="end-time">
                Voting ends: {new Date(proposal.endTime * 1000).toLocaleString()}
              </p>
              
              <div className="proposal-actions">
                {!proposal.hasVoted && !proposal.executed ? (
                  <button 
                    onClick={() => voteOnProposal(proposal.id)}
                    disabled={loading}
                  >
                    {loading ? 'Voting...' : 'Vote'}
                  </button>
                ) : proposal.hasVoted ? (
                  <button disabled>Voted</button>
                ) : null}
                
                {!proposal.executed && Date.now() > proposal.endTime * 1000 ? (
                  <button 
                    onClick={() => executeProposal(proposal.id)}
                    disabled={loading}
                  >
                    {loading ? 'Executing...' : 'Execute'}
                  </button>
                ) : proposal.executed ? (
                  <button disabled>Executed</button>
                ) : null}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default DAOGovernance;
"#
        .to_string(),
        component_type: "governance".to_string(),
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

  console.log('Deploying DAOGovernance with the account:', deployer.address);

  const DAOGovernance = await ethers.getContractFactory('DAOGovernance');
  // Deploy with 1000 tokens (18 decimals) and 24 hour voting period
  const daoGovernance = await DAOGovernance.deploy(
    ethers.utils.parseEther('1000'),
    86400
  );

  await daoGovernance.deployed();

  console.log('DAOGovernance deployed to:', daoGovernance.address);
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
        id: "dao-governance".to_string(),
        name: "DAO Governance".to_string(),
        description: "A decentralized autonomous organization with token-based governance"
            .to_string(),
        dapp_type: "DAO".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![dao_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
