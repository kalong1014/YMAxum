//! Security assessment module
//! Provides security assessment, risk analysis, and security score

use super::hardening::{HardeningResult, SecurityHardening};
use super::monitor::{MonitorStatistics, SecurityMonitor};
use super::scanner::{SecurityScanner, SecurityVulnerability, VulnerabilitySeverity};
use log::info;
use serde::{Deserialize, Serialize};

/// Security assessment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentConfig {
    /// Assessment items
    pub assessment_items: Vec<AssessmentItem>,
    /// Enable risk analysis
    pub enable_risk_analysis: bool,
    /// Enable security score
    pub enable_security_score: bool,
    /// Generate improvement suggestions
    pub generate_improvement_suggestions: bool,
}

impl Default for AssessmentConfig {
    fn default() -> Self {
        Self {
            assessment_items: vec![
                AssessmentItem::VulnerabilityAssessment,
                AssessmentItem::HardeningAssessment,
                AssessmentItem::MonitoringAssessment,
                AssessmentItem::ComplianceAssessment,
            ],
            enable_risk_analysis: true,
            enable_security_score: true,
            generate_improvement_suggestions: true,
        }
    }
}

/// Assessment item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentItem {
    /// Vulnerability assessment
    VulnerabilityAssessment,
    /// Hardening assessment
    HardeningAssessment,
    /// Monitoring assessment
    MonitoringAssessment,
    /// Compliance assessment
    ComplianceAssessment,
}

/// Assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentResult {
    /// Assessment item
    pub assessment_item: AssessmentItem,
    /// Assessment result
    pub result: AssessmentStatus,
    /// Assessment score
    pub score: Option<f64>,
    /// Risk level
    pub risk_level: Option<RiskLevel>,
    /// Security issues
    pub issues: Vec<SecurityIssue>,
    /// Improvement suggestions
    pub improvements: Vec<ImprovementSuggestion>,
}

/// Assessment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentStatus {
    /// Pass
    Pass,
    /// Fail
    Fail,
    /// Needs improvement
    NeedsImprovement,
    /// Pending
    Pending,
}

/// Risk level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// Security issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    /// Issue ID
    pub id: String,
    /// Issue type
    pub issue_type: String,
    /// Severity
    pub severity: VulnerabilitySeverity,
    /// Issue description
    pub description: String,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Priority
    pub priority: u8,
}

/// Improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    /// Suggestion ID
    pub id: String,
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    /// Suggestion description
    pub description: String,
    /// Expected impact
    pub expected_impact: String,
    /// Implementation difficulty
    pub difficulty: u8,
    /// Implementation cost
    pub cost: String,
}

/// Suggestion type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    /// Vulnerability fix
    VulnerabilityFix,
    /// Hardening measure
    HardeningMeasure,
    /// Monitoring improvement
    MonitoringImprovement,
    /// Compliance improvement
    ComplianceImprovement,
}

/// Security assessment tool
pub struct SecurityAssessment {
    /// Configuration
    config: AssessmentConfig,
    /// Assessment results
    results: Vec<AssessmentResult>,
    /// Overall assessment score
    overall_score: Option<f64>,
    /// Overall risk level
    overall_risk_level: Option<RiskLevel>,
}

impl SecurityAssessment {
    /// Create new security assessment tool
    pub fn new(config: AssessmentConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
            overall_score: None,
            overall_risk_level: None,
        }
    }

    /// Execute security assessment
    pub async fn assess(
        &mut self,
        scanner: &SecurityScanner,
        hardening: &SecurityHardening,
        monitor: &SecurityMonitor,
    ) -> Vec<AssessmentResult> {
        info!("Starting security assessment...");

        self.results.clear();

        // Execute each assessment item
        for assessment_item in &self.config.assessment_items {
            let result = self
                .assess_item(assessment_item, scanner, hardening, monitor)
                .await;
            self.results.push(result);
        }

        // Calculate overall score
        self.calculate_overall_score();

        // Calculate overall risk level
        self.calculate_overall_risk_level();

        info!(
            "Security assessment completed, assessed {} items",
            self.results.len()
        );

        self.results.clone()
    }

    /// Assess single item
    async fn assess_item(
        &self,
        item: &AssessmentItem,
        scanner: &SecurityScanner,
        hardening: &SecurityHardening,
        monitor: &SecurityMonitor,
    ) -> AssessmentResult {
        match item {
            AssessmentItem::VulnerabilityAssessment => self.assess_vulnerabilities(scanner),
            AssessmentItem::HardeningAssessment => self.assess_hardening(hardening),
            AssessmentItem::MonitoringAssessment => self.assess_monitoring(monitor).await,
            AssessmentItem::ComplianceAssessment => self.assess_compliance(),
        }
    }

    /// Assess vulnerabilities
    fn assess_vulnerabilities(&self, scanner: &SecurityScanner) -> AssessmentResult {
        info!("Assessing vulnerabilities...");

        let scan_stats = scanner.get_scan_stats();
        let vulnerabilities = scanner.get_scan_results();

        // Calculate vulnerability score
        let score = if scan_stats.vulnerabilities_found > 0 {
            let critical_weight = 10.0;
            let high_weight = 7.5;
            let medium_weight = 5.0;
            let low_weight = 2.5;
            let info_weight = 0.0;

            let total_weight = (scan_stats.critical_count as f64) * critical_weight
                + (scan_stats.high_count as f64) * high_weight
                + (scan_stats.medium_count as f64) * medium_weight
                + (scan_stats.low_count as f64) * low_weight
                + (scan_stats.info_count as f64) * info_weight;

            let max_possible_score = (scan_stats.vulnerabilities_found as f64) * critical_weight;
            let score = (max_possible_score - total_weight) / max_possible_score * 100.0;

            Some(score)
        } else {
            Some(100.0)
        };

        // Determine risk level
        let risk_level = if let Some(score) = score {
            if score >= 80.0 {
                RiskLevel::Low
            } else if score >= 60.0 {
                RiskLevel::Medium
            } else if score >= 40.0 {
                RiskLevel::High
            } else {
                RiskLevel::Critical
            }
        } else {
            RiskLevel::Low
        };

        // Extract security issues
        let issues: Vec<SecurityIssue> = vulnerabilities
            .iter()
            .map(|v| SecurityIssue {
                id: v.id.clone(),
                issue_type: format!("{:?}", v.vulnerability_type),
                severity: v.severity.clone(),
                description: v.description.clone(),
                affected_components: v.affected_components.clone(),
                priority: match v.severity {
                    VulnerabilitySeverity::Critical => 10,
                    VulnerabilitySeverity::High => 8,
                    VulnerabilitySeverity::Medium => 5,
                    VulnerabilitySeverity::Low => 3,
                    VulnerabilitySeverity::Info => 1,
                },
            })
            .collect();

        // Generate improvement suggestions
        let improvements = self.generate_vulnerability_improvements(vulnerabilities);

        let result = AssessmentResult {
            assessment_item: AssessmentItem::VulnerabilityAssessment,
            result: if scan_stats.vulnerabilities_found > 0 {
                AssessmentStatus::NeedsImprovement
            } else {
                AssessmentStatus::Pass
            },
            score,
            risk_level: Some(risk_level),
            issues,
            improvements,
        };

        info!(
            "Vulnerability assessment completed, score: {:?}, risk level: {:?}",
            score, risk_level
        );

        result
    }

    /// Assess hardening
    fn assess_hardening(&self, hardening: &SecurityHardening) -> AssessmentResult {
        info!("Assessing hardening...");

        let results = hardening.get_results();
        let success_count = results.iter().filter(|r| r.success).count();
        let total_count = results.len();

        // Calculate hardening score
        let score = if total_count > 0 {
            (success_count as f64 / total_count as f64) * 100.0
        } else {
            100.0
        };

        // Determine risk level
        let risk_level = if score >= 80.0 {
            RiskLevel::Low
        } else if score >= 60.0 {
            RiskLevel::Medium
        } else if score >= 40.0 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        // Extract security issues
        let issues: Vec<SecurityIssue> = results
            .iter()
            .filter(|r| !r.success)
            .map(|r| SecurityIssue {
                id: format!("hardening_{}", r.item.clone() as i32),
                issue_type: format!("{:?}", r.item),
                severity: VulnerabilitySeverity::High,
                description: r.description.clone(),
                affected_components: vec!["System".to_string()],
                priority: 8,
            })
            .collect();

        // Generate improvement suggestions
        let improvements = self.generate_hardening_improvements(results);

        let result = AssessmentResult {
            assessment_item: AssessmentItem::HardeningAssessment,
            result: if success_count == total_count {
                AssessmentStatus::Pass
            } else {
                AssessmentStatus::NeedsImprovement
            },
            score: Some(score),
            risk_level: Some(risk_level),
            issues,
            improvements,
        };

        info!(
            "Hardening assessment completed, score: {:.1}, risk level: {:?}",
            score, risk_level
        );

        result
    }

    /// Assess monitoring
    async fn assess_monitoring(&self, monitor: &SecurityMonitor) -> AssessmentResult {
        info!("Assessing monitoring...");

        let stats = monitor.get_statistics().await;
        let unhandled_alerts = stats.unhandled_alerts;

        // Calculate monitoring score
        let score = if stats.total_alerts > 0 {
            (stats.total_alerts - unhandled_alerts) as f64 / stats.total_alerts as f64 * 100.0
        } else {
            100.0
        };

        // Determine risk level
        let risk_level = if score >= 80.0 {
            RiskLevel::Low
        } else if score >= 60.0 {
            RiskLevel::Medium
        } else if score >= 40.0 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        // Extract security issues
        let issues: Vec<SecurityIssue> = if unhandled_alerts > 0 {
            vec![SecurityIssue {
                id: "monitor_001".to_string(),
                issue_type: "Unhandled security alerts".to_string(),
                severity: VulnerabilitySeverity::High,
                description: format!("{} security alerts are unhandled", unhandled_alerts),
                affected_components: vec!["Monitoring system".to_string()],
                priority: 9,
            }]
        } else {
            Vec::new()
        };

        // Generate improvement suggestions
        let improvements = self.generate_monitoring_improvements(&stats);

        let result = AssessmentResult {
            assessment_item: AssessmentItem::MonitoringAssessment,
            result: if unhandled_alerts == 0 {
                AssessmentStatus::Pass
            } else {
                AssessmentStatus::NeedsImprovement
            },
            score: Some(score),
            risk_level: Some(risk_level),
            issues,
            improvements,
        };

        info!(
            "Monitoring assessment completed, score: {:.1}, risk level: {:?}",
            score, risk_level
        );

        result
    }

    /// Assess compliance
    fn assess_compliance(&self) -> AssessmentResult {
        info!("Assessing compliance...");

        // Simulate compliance assessment
        // In actual project, implement the following:
        // 1. Check GDPR compliance
        // 2. Check PCI DSS compliance
        // 3. Check HIPAA compliance
        // 4. Check SOX compliance
        // 5. Check other regulatory compliance

        // Calculate compliance score
        let score = 85.0; // Simulated score

        // Determine risk level
        let risk_level = if score >= 80.0 {
            RiskLevel::Low
        } else if score >= 60.0 {
            RiskLevel::Medium
        } else if score >= 40.0 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        // Extract security issues
        let issues: Vec<SecurityIssue> = vec![
            SecurityIssue {
                id: "compliance_001".to_string(),
                issue_type: "Data privacy compliance".to_string(),
                severity: VulnerabilitySeverity::Medium,
                description:
                    "Need to supplement data protection measures to meet GDPR requirements"
                        .to_string(),
                affected_components: vec!["Data management system".to_string()],
                priority: 5,
            },
            SecurityIssue {
                id: "compliance_002".to_string(),
                issue_type: "Security compliance".to_string(),
                severity: VulnerabilitySeverity::Low,
                description: "Need to establish comprehensive security compliance framework"
                    .to_string(),
                affected_components: vec!["Security system".to_string()],
                priority: 3,
            },
        ];

        // Generate improvement suggestions
        let improvements = self.generate_compliance_improvements();

        let result = AssessmentResult {
            assessment_item: AssessmentItem::ComplianceAssessment,
            result: AssessmentStatus::NeedsImprovement,
            score: Some(score),
            risk_level: Some(risk_level),
            issues,
            improvements,
        };

        info!(
            "Compliance assessment completed, score: {:.1}, risk level: {:?}",
            score, risk_level
        );

        result
    }

    /// Generate vulnerability improvement suggestions
    fn generate_vulnerability_improvements(
        &self,
        vulnerabilities: &[SecurityVulnerability],
    ) -> Vec<ImprovementSuggestion> {
        let mut improvements = Vec::new();

        // Count vulnerabilities by severity
        let critical_count = vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, VulnerabilitySeverity::Critical))
            .count();
        let high_count = vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, VulnerabilitySeverity::High))
            .count();
        let medium_count = vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, VulnerabilitySeverity::Medium))
            .count();

        if critical_count > 0 {
            improvements.push(ImprovementSuggestion {
                id: "vuln_fix_001".to_string(),
                suggestion_type: SuggestionType::VulnerabilityFix,
                description: "Fix all critical vulnerabilities immediately".to_string(),
                expected_impact: "Significantly reduce security risk".to_string(),
                difficulty: 8,
                cost: "High".to_string(),
            });
        }

        if high_count > 0 {
            improvements.push(ImprovementSuggestion {
                id: "vuln_fix_002".to_string(),
                suggestion_type: SuggestionType::VulnerabilityFix,
                description: "Fix all high severity vulnerabilities".to_string(),
                expected_impact: "Reduce security risk".to_string(),
                difficulty: 6,
                cost: "Medium".to_string(),
            });
        }

        if medium_count > 5 {
            improvements.push(ImprovementSuggestion {
                id: "vuln_fix_003".to_string(),
                suggestion_type: SuggestionType::VulnerabilityFix,
                description: "Regularly fix medium severity vulnerabilities".to_string(),
                expected_impact: "Gradually reduce security risk".to_string(),
                difficulty: 5,
                cost: "Medium".to_string(),
            });
        }

        improvements
    }

    /// Generate hardening improvement suggestions
    fn generate_hardening_improvements(
        &self,
        results: &[HardeningResult],
    ) -> Vec<ImprovementSuggestion> {
        let mut improvements = Vec::new();

        let failed_items = results.iter().filter(|r| !r.success).count();

        if failed_items > 0 {
            improvements.push(ImprovementSuggestion {
                id: "hardening_001".to_string(),
                suggestion_type: SuggestionType::HardeningMeasure,
                description: "Fix all failed hardening items".to_string(),
                expected_impact: "Strengthen system security posture".to_string(),
                difficulty: 7,
                cost: "Medium".to_string(),
            });
        }

        improvements.push(ImprovementSuggestion {
            id: "hardening_002".to_string(),
            suggestion_type: SuggestionType::HardeningMeasure,
            description: "Regularly update security hardening measures".to_string(),
            expected_impact: "Maintain system security posture".to_string(),
            difficulty: 3,
            cost: "Low".to_string(),
        });

        improvements
    }

    /// Generate monitoring improvement suggestions
    fn generate_monitoring_improvements(
        &self,
        stats: &MonitorStatistics,
    ) -> Vec<ImprovementSuggestion> {
        let mut improvements = Vec::new();

        if stats.unhandled_alerts > 0 {
            improvements.push(ImprovementSuggestion {
                id: "monitor_001".to_string(),
                suggestion_type: SuggestionType::MonitoringImprovement,
                description:
                    "Establish alert handling mechanism and handle security alerts promptly"
                        .to_string(),
                expected_impact: "Improve security event response speed".to_string(),
                difficulty: 5,
                cost: "Medium".to_string(),
            });
        }

        improvements.push(ImprovementSuggestion {
            id: "monitor_002".to_string(),
            suggestion_type: SuggestionType::MonitoringImprovement,
            description: "Optimize monitoring rules and reduce false positives".to_string(),
            expected_impact: "Improve monitoring accuracy".to_string(),
            difficulty: 4,
            cost: "Low".to_string(),
        });

        improvements
    }

    /// Generate compliance improvement suggestions
    fn generate_compliance_improvements(&self) -> Vec<ImprovementSuggestion> {
        vec![
            ImprovementSuggestion {
                id: "compliance_001".to_string(),
                suggestion_type: SuggestionType::ComplianceImprovement,
                description:
                    "Establish comprehensive data protection measures to meet GDPR requirements"
                        .to_string(),
                expected_impact: "Ensure data privacy compliance".to_string(),
                difficulty: 8,
                cost: "High".to_string(),
            },
            ImprovementSuggestion {
                id: "compliance_002".to_string(),
                suggestion_type: SuggestionType::ComplianceImprovement,
                description:
                    "Establish comprehensive security compliance framework and audit mechanism"
                        .to_string(),
                expected_impact: "Ensure security compliance".to_string(),
                difficulty: 6,
                cost: "Medium".to_string(),
            },
            ImprovementSuggestion {
                id: "compliance_003".to_string(),
                suggestion_type: SuggestionType::ComplianceImprovement,
                description: "Regularly conduct compliance audits and security assessments"
                    .to_string(),
                expected_impact: "Maintain compliance status".to_string(),
                difficulty: 3,
                cost: "Low".to_string(),
            },
        ]
    }

    /// Calculate overall score
    fn calculate_overall_score(&mut self) {
        if self.results.is_empty() {
            return;
        }

        let total_score: f64 = self.results.iter().filter_map(|r| r.score).sum::<f64>();

        self.overall_score = Some(total_score / self.results.len() as f64);
    }

    /// Calculate overall risk level
    fn calculate_overall_risk_level(&mut self) {
        if let Some(overall_score) = self.overall_score {
            self.overall_risk_level = Some(if overall_score >= 80.0 {
                RiskLevel::Low
            } else if overall_score >= 60.0 {
                RiskLevel::Medium
            } else if overall_score >= 40.0 {
                RiskLevel::High
            } else {
                RiskLevel::Critical
            });
        }
    }

    /// Get assessment results
    pub fn get_results(&self) -> &[AssessmentResult] {
        &self.results
    }

    /// Get overall score
    pub fn get_overall_score(&self) -> Option<f64> {
        self.overall_score
    }

    /// Get overall risk level
    pub fn get_overall_risk_level(&self) -> Option<RiskLevel> {
        self.overall_risk_level
    }

    /// Generate security assessment report
    pub fn generate_assessment_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Security Assessment Report ===\n\n");

        // Overall assessment
        if let Some(overall_score) = self.overall_score {
            report.push_str("Overall Assessment:\n");
            report.push_str(&format!(
                "Overall security score: {:.1}/100\n",
                overall_score
            ));

            if let Some(risk_level) = &self.overall_risk_level {
                report.push_str(&format!("Overall risk level: {:?}\n\n", risk_level));
            }
        }

        // Detailed assessment results
        if !self.results.is_empty() {
            report.push_str("Detailed Assessment Results:\n");
            for (index, result) in self.results.iter().enumerate() {
                report.push_str(&format!("{}. {:?}\n", index + 1, result.assessment_item));
                report.push_str(&format!("   Assessment result: {:?}\n", result.result));

                if let Some(score) = result.score {
                    report.push_str(&format!("   Score: {:.1}/100\n", score));
                }

                if let Some(risk_level) = &result.risk_level {
                    report.push_str(&format!("   Risk level: {:?}\n", risk_level));
                }

                if !result.issues.is_empty() {
                    report.push_str("   Security issues:\n");
                    for (issue_index, issue) in result.issues.iter().enumerate() {
                        report.push_str(&format!(
                            "   {}. {} - {:?}\n",
                            issue_index + 1,
                            issue.id,
                            issue.severity
                        ));
                        report.push_str(&format!("      Description: {}\n", issue.description));
                        report.push_str(&format!(
                            "      Affected components: {:?}\n",
                            issue.affected_components
                        ));
                        report.push_str(&format!("      Priority: {}/10\n", issue.priority));
                    }
                }

                if !result.improvements.is_empty() {
                    report.push_str("   Improvement suggestions:\n");
                    for (suggestion_index, improvement) in result.improvements.iter().enumerate() {
                        report.push_str(&format!(
                            "   {}. {}\n",
                            suggestion_index + 1,
                            improvement.description
                        ));
                        report.push_str(&format!(
                            "      Expected impact: {}\n",
                            improvement.expected_impact
                        ));
                        report.push_str(&format!(
                            "      Implementation difficulty: {}/10\n",
                            improvement.difficulty
                        ));
                        report.push_str(&format!(
                            "      Implementation cost: {}\n\n",
                            improvement.cost
                        ));
                    }
                }
            }
        }

        // Overall recommendations
        report.push_str("Overall Recommendations:\n");
        report.push_str("1. Prioritize fixing critical and high severity vulnerabilities\n");
        report.push_str("2. Strengthen security hardening measures\n");
        report.push_str("3. Establish comprehensive security monitoring system\n");
        report.push_str("4. Regularly conduct security audits and compliance checks\n");
        report.push_str("5. Regularly conduct security assessments\n");
        report.push_str("6. Establish security incident response mechanism\n");
        report.push_str("7. Conduct security training and awareness activities\n");

        report
    }
}

impl Default for SecurityAssessment {
    fn default() -> Self {
        Self::new(AssessmentConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_assessment() {
        let scanner = SecurityScanner::default();
        let hardening = SecurityHardening::default();
        let monitor = SecurityMonitor::default();

        let mut assessment = SecurityAssessment::new(AssessmentConfig::default());

        // Execute assessment
        let results = assessment.assess(&scanner, &hardening, &monitor).await;

        assert_eq!(results.len(), 4);
        assert!(!results.is_empty());
    }

    #[tokio::test]
    async fn test_assess_vulnerabilities() {
        let scanner = SecurityScanner::default();
        let assessment = SecurityAssessment::new(AssessmentConfig::default());

        let result = assessment.assess_vulnerabilities(&scanner);

        assert!(matches!(
            result.assessment_item,
            AssessmentItem::VulnerabilityAssessment
        ));
        assert!(result.score.is_some());
    }

    #[tokio::test]
    async fn test_generate_assessment_report() {
        let scanner = SecurityScanner::default();
        let hardening = SecurityHardening::default();
        let monitor = SecurityMonitor::default();
        let mut assessment = SecurityAssessment::new(AssessmentConfig::default());

        let _ = assessment.assess(&scanner, &hardening, &monitor).await;

        let report = assessment.generate_assessment_report();

        assert!(report.contains("Security Assessment Report"));
        assert!(report.contains("Overall Assessment"));
    }
}
