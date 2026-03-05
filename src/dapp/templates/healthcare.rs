// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建医疗健康模板
pub fn create_healthcare_template() -> DappTemplate {
    let healthcare_contract = SmartContractTemplate {
        name: "HealthcareRecord".to_string(),
        path: "contracts/HealthcareRecord.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract HealthcareRecord {
    struct MedicalRecord {
        uint256 id;
        address patient;
        address doctor;
        string diagnosis;
        string treatment;
        uint256 timestamp;
        bool verified;
    }

    mapping(uint256 => MedicalRecord) public records;
    mapping(address => uint256[]) public patientRecords;
    mapping(address => bool) public authorizedDoctors;
    uint256 public recordCount;

    event RecordCreated(uint256 id, address patient, address doctor);
    event RecordVerified(uint256 id, address verifier);

    modifier onlyDoctor() {
        require(authorizedDoctors[msg.sender], "Only authorized doctors can perform this action");
        _;
    }

    function addDoctor(address doctor) public {
        authorizedDoctors[doctor] = true;
    }

    function createRecord(address patient, string memory diagnosis, string memory treatment) public onlyDoctor {
        uint256 id = recordCount;
        recordCount++;

        records[id] = MedicalRecord({
            id: id,
            patient: patient,
            doctor: msg.sender,
            diagnosis: diagnosis,
            treatment: treatment,
            timestamp: block.timestamp,
            verified: false
        });

        patientRecords[patient].push(id);
        emit RecordCreated(id, patient, msg.sender);
    }

    function verifyRecord(uint256 id) public onlyDoctor {
        records[id].verified = true;
        emit RecordVerified(id, msg.sender);
    }

    function getRecord(uint256 id) public view returns (MedicalRecord memory) {
        return records[id];
    }

    function getPatientRecords(address patient) public view returns (uint256[] memory) {
        return patientRecords[patient];
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "HealthcareRecord".to_string(),
        path: "src/components/HealthcareRecord.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './HealthcareRecord.css';

const HealthcareRecord = ({ web3, contract }) => {
  const [records, setRecords] = useState([]);
  const [patientAddress, setPatientAddress] = useState('');
  const [diagnosis, setDiagnosis] = useState('');
  const [treatment, setTreatment] = useState('');
  const [loading, setLoading] = useState(false);
  const [userAddress, setUserAddress] = useState('');

  useEffect(() => {
    loadUserAddress();
  }, []);

  const loadUserAddress = async () => {
    if (!web3) return;
    try {
      const accounts = await web3.eth.getAccounts();
      setUserAddress(accounts[0]);
    } catch (error) {
      console.error('Error loading user address:', error);
    }
  };

  const loadRecords = async () => {
    if (!web3 || !contract || !patientAddress) return;

    try {
      setLoading(true);
      const recordIds = await contract.methods.getPatientRecords(patientAddress).call();
      const loadedRecords = [];

      for (let i = 0; i < recordIds.length; i++) {
        const record = await contract.methods.getRecord(recordIds[i]).call();
        loadedRecords.push(record);
      }

      setRecords(loadedRecords);
    } catch (error) {
      console.error('Error loading records:', error);
    } finally {
      setLoading(false);
    }
  };

  const createRecord = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !patientAddress || !diagnosis || !treatment) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.createRecord(patientAddress, diagnosis, treatment).send({
        from: accounts[0]
      });

      alert('Record created successfully!');
      setDiagnosis('');
      setTreatment('');
      loadRecords();
    } catch (error) {
      console.error('Error creating record:', error);
      alert('Record creation failed!');
    } finally {
      setLoading(false);
    }
  };

  const verifyRecord = async (id) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.verifyRecord(id).send({
        from: accounts[0]
      });

      alert('Record verified successfully!');
      loadRecords();
    } catch (error) {
      console.error('Error verifying record:', error);
      alert('Record verification failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="healthcare-record">
      <h2>Healthcare Record</h2>

      <div className="patient-input">
        <h3>Patient Records</h3>
        <div className="form-group">
          <label>Patient Address</label>
          <input
            type="text"
            placeholder="0x..."
            value={patientAddress}
            onChange={(e) => setPatientAddress(e.target.value)}
          />
          <button onClick={loadRecords} disabled={loading}>
            {loading ? 'Loading...' : 'Load Records'}
          </button>
        </div>
      </div>

      <div className="create-record">
        <h3>Create Record</h3>
        <form onSubmit={createRecord}>
          <div className="form-group">
            <label>Patient Address</label>
            <input
              type="text"
              placeholder="0x..."
              value={patientAddress}
              onChange={(e) => setPatientAddress(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Diagnosis</label>
            <textarea
              placeholder="Diagnosis"
              value={diagnosis}
              onChange={(e) => setDiagnosis(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Treatment</label>
            <textarea
              placeholder="Treatment"
              value={treatment}
              onChange={(e) => setTreatment(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Creating...' : 'Create Record'}
          </button>
        </form>
      </div>

      <div className="records">
        <h3>Records</h3>
        {records.length === 0 ? (
          <p>No records found. Create the first one!</p>
        ) : (
          records.map((record) => (
            <div key={record.id} className="record-card">
              <h4>Record #{record.id}</h4>
              <p>Patient: {record.patient}</p>
              <p>Doctor: {record.doctor}</p>
              <p>Diagnosis: {record.diagnosis}</p>
              <p>Treatment: {record.treatment}</p>
              <p>Date: {new Date(record.timestamp * 1000).toLocaleString()}</p>
              <p>Verified: {record.verified ? 'Yes' : 'No'}</p>
              
              {!record.verified && (
                <button 
                  onClick={() => verifyRecord(record.id)}
                  disabled={loading}
                >
                  {loading ? 'Verifying...' : 'Verify Record'}
                </button>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default HealthcareRecord;
"#
        .to_string(),
        component_type: "healthcare".to_string(),
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

  console.log('Deploying HealthcareRecord with the account:', deployer.address);

  const HealthcareRecord = await ethers.getContractFactory('HealthcareRecord');
  const healthcareRecord = await HealthcareRecord.deploy();

  await healthcareRecord.deployed();

  console.log('HealthcareRecord deployed to:', healthcareRecord.address);

  // Add the deployer as an authorized doctor
  await healthcareRecord.addDoctor(deployer.address);
  console.log('Added deployer as authorized doctor');
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
        id: "healthcare".to_string(),
        name: "Healthcare Record".to_string(),
        description: "A decentralized healthcare record management system".to_string(),
        dapp_type: "Healthcare".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![healthcare_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
