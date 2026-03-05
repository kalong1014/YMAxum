// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

//! 教育场景适配器
//! 提供在线教育相关的功能，包括课程管理、学生管理、教师管理、学习进度跟踪等

use crate::scene::SceneAdapter;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 课程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub description: String,
    pub teacher_id: String,
    pub duration: u32, // 课程时长（分钟）
    pub price: f64,
    pub category: String,
    pub level: String, // 初级、中级、高级
    pub enrolled_students: u32,
}

/// 学生信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub grade: String,
    pub enrolled_courses: Vec<String>, // 已报名课程ID
}

/// 教师信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub id: String,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub experience: u32,             // 教学经验（年）
    pub taught_courses: Vec<String>, // 已教授课程ID
}

/// 学习进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProgress {
    pub student_id: String,
    pub course_id: String,
    pub completed_lessons: u32,
    pub total_lessons: u32,
    pub last_accessed: String, // ISO 8601 时间
    pub score: Option<f64>,    // 课程评分
}

/// 课程管理器
pub struct CourseManager {
    courses: Arc<RwLock<Vec<Course>>>,
}

impl Default for CourseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CourseManager {
    pub fn new() -> Self {
        Self {
            courses: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_course(&self, course: Course) {
        let mut courses = self.courses.write().await;
        courses.push(course);
    }

    pub async fn get_course(&self, course_id: &str) -> Option<Course> {
        let courses = self.courses.read().await;
        courses.iter().find(|c| c.id == course_id).cloned()
    }

    pub async fn list_courses(&self, category: Option<&str>) -> Vec<Course> {
        let courses = self.courses.read().await;
        if let Some(cat) = category {
            courses
                .iter()
                .filter(|c| c.category == cat)
                .cloned()
                .collect()
        } else {
            courses.clone()
        }
    }
}

/// 学生管理器
pub struct StudentManager {
    students: Arc<RwLock<Vec<Student>>>,
}

impl Default for StudentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StudentManager {
    pub fn new() -> Self {
        Self {
            students: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_student(&self, student: Student) {
        let mut students = self.students.write().await;
        students.push(student);
    }

    pub async fn get_student(&self, student_id: &str) -> Option<Student> {
        let students = self.students.read().await;
        students.iter().find(|s| s.id == student_id).cloned()
    }

    pub async fn list_students(&self) -> Vec<Student> {
        let students = self.students.read().await;
        students.clone()
    }
}

/// 教师管理器
pub struct TeacherManager {
    teachers: Arc<RwLock<Vec<Teacher>>>,
}

impl Default for TeacherManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TeacherManager {
    pub fn new() -> Self {
        Self {
            teachers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_teacher(&self, teacher: Teacher) {
        let mut teachers = self.teachers.write().await;
        teachers.push(teacher);
    }

    pub async fn get_teacher(&self, teacher_id: &str) -> Option<Teacher> {
        let teachers = self.teachers.read().await;
        teachers.iter().find(|t| t.id == teacher_id).cloned()
    }

    pub async fn list_teachers(&self, subject: Option<&str>) -> Vec<Teacher> {
        let teachers = self.teachers.read().await;
        if let Some(subj) = subject {
            teachers
                .iter()
                .filter(|t| t.subject == subj)
                .cloned()
                .collect()
        } else {
            teachers.clone()
        }
    }
}

/// 学习进度管理器
#[derive(Clone)]
pub struct LearningProgressManager {
    progress: Arc<RwLock<Vec<LearningProgress>>>,
}

impl Default for LearningProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LearningProgressManager {
    pub fn new() -> Self {
        Self {
            progress: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn update_progress(&self, progress: LearningProgress) {
        let mut progress_list = self.progress.write().await;
        if let Some(existing) = progress_list
            .iter_mut()
            .find(|p| p.student_id == progress.student_id && p.course_id == progress.course_id)
        {
            *existing = progress;
        } else {
            progress_list.push(progress);
        }
    }

    pub async fn get_progress(
        &self,
        student_id: &str,
        course_id: &str,
    ) -> Option<LearningProgress> {
        let progress_list = self.progress.read().await;
        progress_list
            .iter()
            .find(|p| p.student_id == student_id && p.course_id == course_id)
            .cloned()
    }

    pub async fn get_student_progress(&self, student_id: &str) -> Vec<LearningProgress> {
        let progress_list = self.progress.read().await;
        progress_list
            .iter()
            .filter(|p| p.student_id == student_id)
            .cloned()
            .collect()
    }
}

/// 教育场景适配器
pub struct EducationSceneAdapter {
    course_manager: Option<CourseManager>,
    student_manager: Option<StudentManager>,
    teacher_manager: Option<TeacherManager>,
    progress_manager: Option<LearningProgressManager>,
    scene_name: &'static str,
    initialized: bool,
    started: bool,
}

impl Default for EducationSceneAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl EducationSceneAdapter {
    pub fn new() -> Self {
        Self {
            course_manager: None,
            student_manager: None,
            teacher_manager: None,
            progress_manager: None,
            scene_name: "education",
            initialized: false,
            started: false,
        }
    }
}

impl SceneAdapter for EducationSceneAdapter {
    fn name(&self) -> &'static str {
        self.scene_name
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            info!("Education scene already initialized");
            return Ok(());
        }

        info!("Initializing education scene...");

        self.course_manager = Some(CourseManager::new());
        self.student_manager = Some(StudentManager::new());
        self.teacher_manager = Some(TeacherManager::new());
        self.progress_manager = Some(LearningProgressManager::new());

        self.initialized = true;
        info!("Education scene initialized successfully");
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            return Err("Education scene not initialized".into());
        }

        if self.started {
            info!("Education scene already started");
            return Ok(());
        }

        info!("Starting education scene...");

        // 启动定时任务，例如每日学习报告生成
        if let Some(_progress_manager) = self.progress_manager.clone() {
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(24 * 3600)).await;
                    info!("Generating daily learning reports");
                    // 这里可以实现每日学习报告生成逻辑
                }
            });
        }

        self.started = true;
        info!("Education scene started successfully");
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.started {
            info!("Education scene already stopped");
            return Ok(());
        }

        info!("Stopping education scene...");
        // 这里可以实现停止逻辑，例如保存状态等

        self.started = false;
        info!("Education scene stopped successfully");
        Ok(())
    }
}

