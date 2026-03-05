// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建教育模板
pub fn create_education_template() -> DappTemplate {
    let education_contract = SmartContractTemplate {
        name: "EducationCertificate".to_string(),
        path: "contracts/EducationCertificate.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract EducationCertificate {
    struct Certificate {
        uint256 id;
        string studentName;
        string courseName;
        string institution;
        uint256 issueDate;
        address issuer;
        bool verified;
    }

    mapping(uint256 => Certificate) public certificates;
    mapping(address => uint256[]) public studentCertificates;
    mapping(address => bool) public authorizedInstitutions;
    uint256 public certificateCount;

    event CertificateIssued(uint256 id, string studentName, string courseName, address issuer);
    event CertificateVerified(uint256 id, address verifier);

    modifier onlyInstitution() {
        require(authorizedInstitutions[msg.sender], "Only authorized institutions can issue certificates");
        _;
    }

    function addInstitution(address institution) public {
        authorizedInstitutions[institution] = true;
    }

    function issueCertificate(string memory studentName, string memory courseName, string memory institution) public onlyInstitution {
        uint256 id = certificateCount;
        certificateCount++;

        certificates[id] = Certificate({
            id: id,
            studentName: studentName,
            courseName: courseName,
            institution: institution,
            issueDate: block.timestamp,
            issuer: msg.sender,
            verified: false
        });

        // For demonstration purposes, we'll use a dummy address for the student
        // In a real application, this would be the student's actual address
        address dummyStudentAddress = address(uint160(id));
        studentCertificates[dummyStudentAddress].push(id);

        emit CertificateIssued(id, studentName, courseName, msg.sender);
    }

    function verifyCertificate(uint256 id) public {
        certificates[id].verified = true;
        emit CertificateVerified(id, msg.sender);
    }

    function getCertificate(uint256 id) public view returns (Certificate memory) {
        return certificates[id];
    }

    function getStudentCertificates(address student) public view returns (uint256[] memory) {
        return studentCertificates[student];
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "EducationCertificate".to_string(),
        path: "src/components/EducationCertificate.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './EducationCertificate.css';

const EducationCertificate = ({ web3, contract }) => {
  const [certificates, setCertificates] = useState([]);
  const [studentName, setStudentName] = useState('');
  const [courseName, setCourseName] = useState('');
  const [institution, setInstitution] = useState('');
  const [loading, setLoading] = useState(false);
  const [studentAddress, setStudentAddress] = useState('');

  const loadCertificates = async () => {
    if (!web3 || !contract || !studentAddress) return;

    try {
      setLoading(true);
      const certificateIds = await contract.methods.getStudentCertificates(studentAddress).call();
      const loadedCertificates = [];

      for (let i = 0; i < certificateIds.length; i++) {
        const certificate = await contract.methods.getCertificate(certificateIds[i]).call();
        loadedCertificates.push(certificate);
      }

      setCertificates(loadedCertificates);
    } catch (error) {
      console.error('Error loading certificates:', error);
    } finally {
      setLoading(false);
    }
  };

  const issueCertificate = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !studentName || !courseName || !institution) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.issueCertificate(studentName, courseName, institution).send({
        from: accounts[0]
      });

      alert('Certificate issued successfully!');
      setStudentName('');
      setCourseName('');
      setInstitution('');
    } catch (error) {
      console.error('Error issuing certificate:', error);
      alert('Certificate issuance failed!');
    } finally {
      setLoading(false);
    }
  };

  const verifyCertificate = async (id) => {
    if (!web3 || !contract) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.verifyCertificate(id).send({
        from: accounts[0]
      });

      alert('Certificate verified successfully!');
      loadCertificates();
    } catch (error) {
      console.error('Error verifying certificate:', error);
      alert('Certificate verification failed!');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="education-certificate">
      <h2>Education Certificate</h2>

      <div className="student-input">
        <h3>Student Certificates</h3>
        <div className="form-group">
          <label>Student Address</label>
          <input
            type="text"
            placeholder="0x..."
            value={studentAddress}
            onChange={(e) => setStudentAddress(e.target.value)}
          />
          <button onClick={loadCertificates} disabled={loading}>
            {loading ? 'Loading...' : 'Load Certificates'}
          </button>
        </div>
      </div>

      <div className="issue-certificate">
        <h3>Issue Certificate</h3>
        <form onSubmit={issueCertificate}>
          <div className="form-group">
            <label>Student Name</label>
            <input
              type="text"
              placeholder="Student name"
              value={studentName}
              onChange={(e) => setStudentName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Course Name</label>
            <input
              type="text"
              placeholder="Course name"
              value={courseName}
              onChange={(e) => setCourseName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Institution</label>
            <input
              type="text"
              placeholder="Institution name"
              value={institution}
              onChange={(e) => setInstitution(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Issuing...' : 'Issue Certificate'}
          </button>
        </form>
      </div>

      <div className="certificates">
        <h3>Certificates</h3>
        {certificates.length === 0 ? (
          <p>No certificates found. Issue the first one!</p>
        ) : (
          certificates.map((certificate) => (
            <div key={certificate.id} className="certificate-card">
              <h4>Certificate #{certificate.id}</h4>
              <p>Student: {certificate.studentName}</p>
              <p>Course: {certificate.courseName}</p>
              <p>Institution: {certificate.institution}</p>
              <p>Issue Date: {new Date(certificate.issueDate * 1000).toLocaleString()}</p>
              <p>Issuer: {certificate.issuer}</p>
              <p>Verified: {certificate.verified ? 'Yes' : 'No'}</p>
              
              {!certificate.verified && (
                <button 
                  onClick={() => verifyCertificate(certificate.id)}
                  disabled={loading}
                >
                  {loading ? 'Verifying...' : 'Verify Certificate'}
                </button>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default EducationCertificate;
"#
        .to_string(),
        component_type: "education".to_string(),
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

  console.log('Deploying EducationCertificate with the account:', deployer.address);

  const EducationCertificate = await ethers.getContractFactory('EducationCertificate');
  const educationCertificate = await EducationCertificate.deploy();

  await educationCertificate.deployed();

  console.log('EducationCertificate deployed to:', educationCertificate.address);

  // Add the deployer as an authorized institution
  await educationCertificate.addInstitution(deployer.address);
  console.log('Added deployer as authorized institution');
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
        id: "education".to_string(),
        name: "Education Certificate".to_string(),
        description: "A decentralized education certificate management system".to_string(),
        dapp_type: "Education".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![education_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
