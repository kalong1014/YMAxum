// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use super::*;

/// 创建社交档案模板
pub fn create_social_profile_template() -> DappTemplate {
    let social_contract = SmartContractTemplate {
        name: "SocialProfile".to_string(),
        path: "contracts/SocialProfile.sol".to_string(),
        content: r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SocialProfile {
    struct Profile {
        string username;
        string bio;
        string avatar;
        uint256 followers;
        uint256 following;
        bool exists;
    }

    mapping(address => Profile) public profiles;
    mapping(address => mapping(address => bool)) public follows;

    event ProfileCreated(address indexed user, string username);
    event ProfileUpdated(address indexed user, string username, string bio, string avatar);
    event Followed(address indexed follower, address indexed following);
    event Unfollowed(address indexed follower, address indexed following);

    function createProfile(string memory username, string memory bio, string memory avatar) public {
        require(!profiles[msg.sender].exists, "Profile already exists");
        
        profiles[msg.sender] = Profile({
            username: username,
            bio: bio,
            avatar: avatar,
            followers: 0,
            following: 0,
            exists: true
        });
        
        emit ProfileCreated(msg.sender, username);
    }

    function updateProfile(string memory username, string memory bio, string memory avatar) public {
        require(profiles[msg.sender].exists, "Profile does not exist");
        
        Profile storage profile = profiles[msg.sender];
        profile.username = username;
        profile.bio = bio;
        profile.avatar = avatar;
        
        emit ProfileUpdated(msg.sender, username, bio, avatar);
    }

    function follow(address user) public {
        require(profiles[msg.sender].exists, "Your profile does not exist");
        require(profiles[user].exists, "User profile does not exist");
        require(!follows[msg.sender][user], "Already following");
        require(msg.sender != user, "Cannot follow yourself");
        
        follows[msg.sender][user] = true;
        profiles[msg.sender].following++;
        profiles[user].followers++;
        
        emit Followed(msg.sender, user);
    }

    function unfollow(address user) public {
        require(profiles[msg.sender].exists, "Your profile does not exist");
        require(profiles[user].exists, "User profile does not exist");
        require(follows[msg.sender][user], "Not following");
        
        follows[msg.sender][user] = false;
        profiles[msg.sender].following--;
        profiles[user].followers--;
        
        emit Unfollowed(msg.sender, user);
    }

    function getProfile(address user) public view returns (Profile memory) {
        return profiles[user];
    }

    function isFollowing(address follower, address following) public view returns (bool) {
        return follows[follower][following];
    }
}
"#
        .to_string(),
        deployment_params: vec![],
    };

    let frontend_template = FrontendTemplate {
        name: "SocialProfile".to_string(),
        path: "src/components/SocialProfile.jsx".to_string(),
        content: r#"import React, { useState, useEffect } from 'react';
import './SocialProfile.css';

const SocialProfile = ({ web3, contract }) => {
  const [profile, setProfile] = useState(null);
  const [username, setUsername] = useState('');
  const [bio, setBio] = useState('');
  const [avatar, setAvatar] = useState('');
  const [followAddress, setFollowAddress] = useState('');
  const [loading, setLoading] = useState(false);
  const [creatingProfile, setCreatingProfile] = useState(false);

  useEffect(() => {
    loadProfile();
  }, []);

  const loadProfile = async () => {
    if (!web3 || !contract) return;

    try {
      const accounts = await web3.eth.getAccounts();
      const userAddress = accounts[0];

      const userProfile = await contract.methods.getProfile(userAddress).call();
      if (userProfile.exists) {
        setProfile(userProfile);
        setUsername(userProfile.username);
        setBio(userProfile.bio);
        setAvatar(userProfile.avatar);
      }
    } catch (error) {
      console.error('Error loading profile:', error);
    }
  };

  const createOrUpdateProfile = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !username) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      if (profile) {
        // Update existing profile
        await contract.methods.updateProfile(username, bio, avatar).send({
          from: accounts[0]
        });
        alert('Profile updated successfully!');
      } else {
        // Create new profile
        await contract.methods.createProfile(username, bio, avatar).send({
          from: accounts[0]
        });
        alert('Profile created successfully!');
        setCreatingProfile(false);
      }

      loadProfile();
    } catch (error) {
      console.error('Error saving profile:', error);
      alert('Profile operation failed!');
    } finally {
      setLoading(false);
    }
  };

  const handleFollow = async (e) => {
    e.preventDefault();
    if (!web3 || !contract || !followAddress) return;

    setLoading(true);
    try {
      const accounts = await web3.eth.getAccounts();

      await contract.methods.follow(followAddress).send({
        from: accounts[0]
      });

      alert('Followed successfully!');
      setFollowAddress('');
    } catch (error) {
      console.error('Error following:', error);
      alert('Follow operation failed!');
    } finally {
      setLoading(false);
    }
  };

  if (!profile && !creatingProfile) {
    return (
      <div className="social-profile">
        <h2>Social Profile</h2>
        <p>You don't have a profile yet. Create one!</p>
        <button onClick={() => setCreatingProfile(true)}>Create Profile</button>
      </div>
    );
  }

  return (
    <div className="social-profile">
      <h2>Social Profile</h2>

      {profile && (
        <div className="profile-card">
          <div className="profile-header">
            <div className="avatar">
              {avatar ? (
                <img src={avatar} alt="Avatar" />
              ) : (
                <div className="avatar-placeholder">{profile.username.charAt(0)}</div>
              )}
            </div>
            <div className="profile-info">
              <h3>{profile.username}</h3>
              <p>{profile.bio}</p>
              <div className="stats">
                <span>Followers: {profile.followers}</span>
                <span>Following: {profile.following}</span>
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="profile-form">
        <h3>{profile ? 'Edit Profile' : 'Create Profile'}</h3>
        <form onSubmit={createOrUpdateProfile}>
          <div className="form-group">
            <label>Username</label>
            <input
              type="text"
              placeholder="Username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Bio</label>
            <textarea
              placeholder="About me"
              value={bio}
              onChange={(e) => setBio(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label>Avatar URL</label>
            <input
              type="text"
              placeholder="https://..."
              value={avatar}
              onChange={(e) => setAvatar(e.target.value)}
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Saving...' : (profile ? 'Update Profile' : 'Create Profile')}
          </button>
        </form>
      </div>

      <div className="follow-form">
        <h3>Follow User</h3>
        <form onSubmit={handleFollow}>
          <div className="form-group">
            <label>User Address</label>
            <input
              type="text"
              placeholder="0x..."
              value={followAddress}
              onChange={(e) => setFollowAddress(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Following...' : 'Follow'}
          </button>
        </form>
      </div>
    </div>
  );
};

export default SocialProfile;
"#
        .to_string(),
        component_type: "profile".to_string(),
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

  console.log('Deploying SocialProfile with the account:', deployer.address);

  const SocialProfile = await ethers.getContractFactory('SocialProfile');
  const socialProfile = await SocialProfile.deploy();

  await socialProfile.deployed();

  console.log('SocialProfile deployed to:', socialProfile.address);
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
        id: "social-profile".to_string(),
        name: "Social Profile".to_string(),
        description: "A decentralized social profile system with follow functionality"
            .to_string(),
        dapp_type: "Social".to_string(),
        blockchain: "Ethereum".to_string(),
        frontend_framework: "React".to_string(),
        smart_contract_templates: vec![social_contract],
        frontend_templates: vec![frontend_template],
        config_templates,
        deployment_templates,
    }
}
