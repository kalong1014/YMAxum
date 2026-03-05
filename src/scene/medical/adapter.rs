// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 医疗场景适配器
//! 提供医疗相关的功能，包括患者管理、医生管理、预约管理、电子病历管理等

use crate::scene::SceneAdapter;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 患者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub contact: String,
    pub address: String,
    pub medical_history: Vec<String>, // 病史记录
}

/// 医生信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doctor {
    pub id: String,
    pub name: String,
    pub specialty: String,
    pub experience: u32,       // 从医经验（年）
    pub schedule: Vec<String>, // 排班信息
    pub contact: String,
}

/// 预约信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub date: String,   // ISO 8601 日期
    pub time: String,   // 时间
    pub status: String, // 状态：预约中、已确认、已完成、已取消
    pub reason: String, // 就诊原因
}

/// 电子病历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicMedicalRecord {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub diagnosis_date: String,    // ISO 8601 日期
    pub diagnosis: String,         // 诊断结果
    pub prescription: Vec<String>, // 处方
    pub treatment: Vec<String>,    // 治疗方案
    pub notes: String,             // 医生备注
}

/// 患者管理器
pub struct PatientManager {
    patients: Arc<RwLock<Vec<Patient>>>,
}

impl Default for PatientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PatientManager {
    pub fn new() -> Self {
        Self {
            patients: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_patient(&self, patient: Patient) {
        let mut patients = self.patients.write().await;
        patients.push(patient);
    }

    pub async fn get_patient(&self, patient_id: &str) -> Option<Patient> {
        let patients = self.patients.read().await;
        patients.iter().find(|p| p.id == patient_id).cloned()
    }

    pub async fn list_patients(&self) -> Vec<Patient> {
        let patients = self.patients.read().await;
        patients.clone()
    }
}

/// 医生管理器
pub struct DoctorManager {
    doctors: Arc<RwLock<Vec<Doctor>>>,
}

impl Default for DoctorManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DoctorManager {
    pub fn new() -> Self {
        Self {
            doctors: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_doctor(&self, doctor: Doctor) {
        let mut doctors = self.doctors.write().await;
        doctors.push(doctor);
    }

    pub async fn get_doctor(&self, doctor_id: &str) -> Option<Doctor> {
        let doctors = self.doctors.read().await;
        doctors.iter().find(|d| d.id == doctor_id).cloned()
    }

    pub async fn list_doctors(&self, specialty: Option<&str>) -> Vec<Doctor> {
        let doctors = self.doctors.read().await;
        if let Some(spec) = specialty {
            doctors
                .iter()
                .filter(|d| d.specialty == spec)
                .cloned()
                .collect()
        } else {
            doctors.clone()
        }
    }
}

/// 预约管理器
#[derive(Clone)]
pub struct AppointmentManager {
    appointments: Arc<RwLock<Vec<Appointment>>>,
}

impl Default for AppointmentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AppointmentManager {
    pub fn new() -> Self {
        Self {
            appointments: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_appointment(&self, appointment: Appointment) {
        let mut appointments = self.appointments.write().await;
        appointments.push(appointment);
    }

    pub async fn update_appointment_status(&self, appointment_id: &str, status: &str) {
        let mut appointments = self.appointments.write().await;
        if let Some(appointment) = appointments.iter_mut().find(|a| a.id == appointment_id) {
            appointment.status = status.to_string();
        }
    }

    pub async fn get_appointments(
        &self,
        patient_id: Option<&str>,
        doctor_id: Option<&str>,
    ) -> Vec<Appointment> {
        let appointments = self.appointments.read().await;
        appointments
            .iter()
            .filter(|a| {
                (patient_id.is_none() || a.patient_id == patient_id.unwrap())
                    && (doctor_id.is_none() || a.doctor_id == doctor_id.unwrap())
            })
            .cloned()
            .collect()
    }
}

/// 电子病历管理器
pub struct MedicalRecordManager {
    records: Arc<RwLock<Vec<ElectronicMedicalRecord>>>,
}

impl Default for MedicalRecordManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MedicalRecordManager {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_record(&self, record: ElectronicMedicalRecord) {
        let mut records = self.records.write().await;
        records.push(record);
    }

    pub async fn get_patient_records(&self, patient_id: &str) -> Vec<ElectronicMedicalRecord> {
        let records = self.records.read().await;
        records
            .iter()
            .filter(|r| r.patient_id == patient_id)
            .cloned()
            .collect()
    }

    pub async fn get_record(&self, record_id: &str) -> Option<ElectronicMedicalRecord> {
        let records = self.records.read().await;
        records.iter().find(|r| r.id == record_id).cloned()
    }
}

/// 医疗场景适配器
pub struct MedicalSceneAdapter {
    patient_manager: Option<PatientManager>,
    doctor_manager: Option<DoctorManager>,
    appointment_manager: Option<AppointmentManager>,
    record_manager: Option<MedicalRecordManager>,
    scene_name: &'static str,
    initialized: bool,
    started: bool,
}

impl Default for MedicalSceneAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl MedicalSceneAdapter {
    pub fn new() -> Self {
        Self {
            patient_manager: None,
            doctor_manager: None,
            appointment_manager: None,
            record_manager: None,
            scene_name: "medical",
            initialized: false,
            started: false,
        }
    }
}

impl SceneAdapter for MedicalSceneAdapter {
    fn name(&self) -> &'static str {
        self.scene_name
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            info!("Medical scene already initialized");
            return Ok(());
        }

        info!("Initializing medical scene...");

        self.patient_manager = Some(PatientManager::new());
        self.doctor_manager = Some(DoctorManager::new());
        self.appointment_manager = Some(AppointmentManager::new());
        self.record_manager = Some(MedicalRecordManager::new());

        self.initialized = true;
        info!("Medical scene initialized successfully");
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Medical scene not initialized".into());
        }

        if self.started {
            info!("Medical scene already started");
            return Ok(());
        }

        info!("Starting medical scene...");

        // 启动定时任务，例如预约提醒
        if let Some(_appointment_manager) = self.appointment_manager.clone() {
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                    info!("Checking appointments for reminders");
                    // 这里可以实现预约提醒逻辑
                }
            });
        }

        self.started = true;
        info!("Medical scene started successfully");
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.started {
            info!("Medical scene already stopped");
            return Ok(());
        }

        info!("Stopping medical scene...");
        // 这里可以实现停止逻辑，例如保存状态等

        self.started = false;
        info!("Medical scene stopped successfully");
        Ok(())
    }
}

