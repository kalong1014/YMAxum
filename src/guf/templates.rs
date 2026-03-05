use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GUF 模板
/// 定义 GUF (Godot UI Framework) 的模板结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufTemplate {
    /// 模板 ID
    pub id: String,
    /// 模板名称
    pub name: String,
    /// 模板描述
    pub description: String,
    /// 行业类型
    pub industry: String,
    /// GUF 版本
    pub guf_version: String,
    /// 前端框架
    pub frontend_framework: String,
    /// 组件模板
    pub component_templates: Vec<GufComponentTemplate>,
    /// 样式模板
    pub style_templates: Vec<GufStyleTemplate>,
    /// 配置模板
    pub config_templates: HashMap<String, String>,
    /// 资源文件
    pub resource_files: Vec<GufResourceFile>,
}

/// GUF 组件模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufComponentTemplate {
    /// 组件名称
    pub name: String,
    /// 组件类型
    pub component_type: String,
    /// 组件路径
    pub path: String,
    /// 组件内容
    pub content: String,
    /// 组件属性
    pub props: HashMap<String, serde_json::Value>,
}

/// GUF 样式模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufStyleTemplate {
    /// 样式名称
    pub name: String,
    /// 样式路径
    pub path: String,
    /// 样式内容
    pub content: String,
    /// 样式类型
    pub style_type: String,
}

/// GUF 资源文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GufResourceFile {
    /// 资源名称
    pub name: String,
    /// 资源路径
    pub path: String,
    /// 资源类型
    pub resource_type: String,
    /// 资源内容（base64 编码）
    pub content: Option<String>,
}

/// GUF 模板库
/// 管理 GUF 模板的存储和检索
pub struct GufTemplateLibrary {
    /// 模板映射
    templates: HashMap<String, GufTemplate>,
}

impl GufTemplateLibrary {
    /// 创建新的 GUF 模板库
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
        };

        // 初始化内置模板
        library.initialize_templates();
        library
    }

    /// 初始化内置模板
    fn initialize_templates(&mut self) {
        // 金融行业模板
        self.templates.insert(
            "finance-dashboard".to_string(),
            self.create_finance_dashboard_template(),
        );

        // 电商行业模板
        self.templates.insert(
            "ecommerce-store".to_string(),
            self.create_ecommerce_store_template(),
        );

        // 社交媒体模板
        self.templates.insert(
            "social-media".to_string(),
            self.create_social_media_template(),
        );

        // 企业管理模板
        self.templates.insert(
            "enterprise-management".to_string(),
            self.create_enterprise_management_template(),
        );

        // 医疗健康模板
        self.templates.insert(
            "healthcare-portal".to_string(),
            self.create_healthcare_portal_template(),
        );

        // 教育学习模板
        self.templates.insert(
            "education-platform".to_string(),
            self.create_education_platform_template(),
        );

        // 房地产模板
        self.templates.insert(
            "real-estate".to_string(),
            self.create_real_estate_template(),
        );

        // 能源管理模板
        self.templates.insert(
            "energy-management".to_string(),
            self.create_energy_management_template(),
        );
    }

    /// 获取模板
    pub fn get_template(&self, template_id: &str) -> Option<&GufTemplate> {
        self.templates.get(template_id)
    }

    /// 获取所有模板
    pub fn get_all_templates(&self) -> Vec<&GufTemplate> {
        self.templates.values().collect()
    }

    /// 按行业获取模板
    pub fn get_templates_by_industry(&self, industry: &str) -> Vec<&GufTemplate> {
        self.templates
            .values()
            .filter(|template| template.industry == industry)
            .collect()
    }

    /// 创建金融仪表盘模板
    fn create_finance_dashboard_template(&self) -> GufTemplate {
        let dashboard_component = GufComponentTemplate {
            name: "FinanceDashboard".to_string(),
            component_type: "Dashboard".to_string(),
            path: "components/FinanceDashboard.gd".to_string(),
            content: r#"extends Control

# 金融仪表盘组件
# 用于展示金融数据和分析

@onready var balance_label = $VBoxContainer/BalanceLabel
@onready var transactions_list = $VBoxContainer/TransactionsList
@onready var chart = $VBoxContainer/Chart

func _ready():
	# 初始化仪表盘
	update_balance()
	load_transactions()
	update_chart()

func update_balance():
	# 更新余额显示
	balance_label.text = "Balance: $10,000.00"

func load_transactions():
	# 加载交易记录
	# 实际应用中会从API获取数据
	var transactions = [
		{"id": 1, "description": "Salary", "amount": 5000.00, "date": "2024-01-01"},
		{"id": 2, "description": "Rent", "amount": -1500.00, "date": "2024-01-05"},
		{"id": 3, "description": "Groceries", "amount": -200.00, "date": "2024-01-10"}
	]
	
	# 填充交易列表
	for transaction in transactions:
		var item = Label.new()
		item.text = "%s: $%.2f (%s)" % [transaction.description, transaction.amount, transaction.date]
		transactions_list.add_child(item)

func update_chart():
	# 更新图表
	# 实际应用中会使用ChartNode或其他图表库
	chart.text = "Financial Chart: January 2024"
"#
            .to_string(),
            props: HashMap::from([
                (
                    "title".to_string(),
                    serde_json::Value::String("Financial Dashboard".to_string()),
                ),
                (
                    "theme".to_string(),
                    serde_json::Value::String("dark".to_string()),
                ),
                (
                    "currency".to_string(),
                    serde_json::Value::String("USD".to_string()),
                ),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "FinanceTheme".to_string(),
            path: "styles/finance_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://c2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/finance_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
label/theme_override_colors/font_color = Color(0.9, 0.9, 0.9, 1)
panel/theme_override_styles/panel = ExtResource("StyleBoxFlat_1")
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "app_config.json".to_string(),
            r#"{
  "app_name": "Finance Dashboard",
  "version": "1.0.0",
  "api_endpoint": "https://api.finance.com",
  "refresh_interval": 60000,
  "theme": "dark"
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "finance_icon".to_string(),
                path: "icons/finance_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "chart_icon".to_string(),
                path: "icons/chart_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "finance-dashboard".to_string(),
            name: "Finance Dashboard".to_string(),
            description:
                "A professional financial dashboard template for tracking expenses and income"
                    .to_string(),
            industry: "Finance".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![dashboard_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建电商商店模板
    fn create_ecommerce_store_template(&self) -> GufTemplate {
        let store_component = GufComponentTemplate {
            name: "EcommerceStore".to_string(),
            component_type: "Store".to_string(),
            path: "components/EcommerceStore.gd".to_string(),
            content: r#"extends Control

# 电商商店组件
# 用于展示和销售商品

@onready var product_grid = $VBoxContainer/ProductGrid
@onready var cart_button = $VBoxContainer/CartButton
@onready var search_bar = $VBoxContainer/SearchBar

func _ready():
	# 初始化商店
	load_products()
	update_cart_count()

func load_products():
	# 加载商品列表
	# 实际应用中会从API获取数据
	var products = [
		{"id": 1, "name": "Smartphone", "price": 999.99, "image": "res://images/smartphone.png"},
		{"id": 2, "name": "Laptop", "price": 1499.99, "image": "res://images/laptop.png"},
		{"id": 3, "name": "Headphones", "price": 199.99, "image": "res://images/headphones.png"}
	]
	
	# 填充商品网格
	for product in products:
		var product_item = preload("res://components/ProductItem.gd").new()
		product_item.setup(product)
		product_grid.add_child(product_item)

func update_cart_count():
	# 更新购物车数量
	cart_button.text = "Cart (3)"

func search_products(query):
	# 搜索商品
	print("Searching for: " + query)
	# 实际应用中会调用API进行搜索
"#
            .to_string(),
            props: HashMap::from([
                (
                    "title".to_string(),
                    serde_json::Value::String("Ecommerce Store".to_string()),
                ),
                (
                    "currency".to_string(),
                    serde_json::Value::String("USD".to_string()),
                ),
                (
                    "products_per_page".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(12)),
                ),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "EcommerceTheme".to_string(),
            path: "styles/ecommerce_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://d2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/ecommerce_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.2, 0.2, 0.2, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "store_config.json".to_string(),
            r#"{
  "store_name": "My Ecommerce Store",
  "version": "1.0.0",
  "api_endpoint": "https://api.store.com",
  "currency": "USD",
  "shipping_options": [
    {"id": 1, "name": "Standard", "price": 5.99},
    {"id": 2, "name": "Express", "price": 12.99}
  ]
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "store_icon".to_string(),
                path: "icons/store_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "cart_icon".to_string(),
                path: "icons/cart_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "ecommerce-store".to_string(),
            name: "Ecommerce Store".to_string(),
            description: "A modern ecommerce store template for selling products online"
                .to_string(),
            industry: "Ecommerce".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![store_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建社交媒体模板
    fn create_social_media_template(&self) -> GufTemplate {
        let social_component = GufComponentTemplate {
            name: "SocialMediaFeed".to_string(),
            component_type: "Feed".to_string(),
            path: "components/SocialMediaFeed.gd".to_string(),
            content: r#"extends Control

# 社交媒体信息流组件
# 用于展示和互动社交媒体内容

@onready var feed_list = $VBoxContainer/FeedList
@onready var post_button = $VBoxContainer/PostButton
@onready var profile_button = $VBoxContainer/ProfileButton

func _ready():
	# 初始化社交媒体
	load_feed()
	update_profile()

func load_feed():
	# 加载信息流
	# 实际应用中会从API获取数据
	var posts = [
		{"id": 1, "user": "John Doe", "content": "Hello world!", "likes": 42, "comments": 5},
		{"id": 2, "user": "Jane Smith", "content": "Check out this amazing photo!", "likes": 128, "comments": 15},
		{"id": 3, "user": "Bob Johnson", "content": "Just finished my new project!", "likes": 76, "comments": 12}
	]
	
	# 填充信息流
	for post in posts:
		var post_item = preload("res://components/PostItem.gd").new()
		post_item.setup(post)
		feed_list.add_child(post_item)

func update_profile():
	# 更新个人资料
	profile_button.text = "Profile"

func create_post(content):
	# 创建新帖子
	print("Creating post: " + content)
	# 实际应用中会调用API创建帖子
"#.to_string(),
            props: HashMap::from([
                ("title".to_string(), serde_json::Value::String("Social Media".to_string())),
                ("feed_refresh_interval".to_string(), serde_json::Value::Number(serde_json::Number::from(30000))),
                ("max_post_length".to_string(), serde_json::Value::Number(serde_json::Number::from(500))),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "SocialTheme".to_string(),
            path: "styles/social_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://e2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/social_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.1, 0.1, 0.1, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "social_config.json".to_string(),
            r#"{
  "app_name": "Social Media App",
  "version": "1.0.0",
  "api_endpoint": "https://api.social.com",
  "feed_refresh_interval": 30000,
  "max_post_length": 500,
  "enable_notifications": true
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "social_icon".to_string(),
                path: "icons/social_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "post_icon".to_string(),
                path: "icons/post_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "social-media".to_string(),
            name: "Social Media".to_string(),
            description: "A modern social media template for building social networking apps"
                .to_string(),
            industry: "Social".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![social_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建企业管理模板
    fn create_enterprise_management_template(&self) -> GufTemplate {
        let management_component = GufComponentTemplate {
            name: "EnterpriseManagement".to_string(),
            component_type: "Management".to_string(),
            path: "components/EnterpriseManagement.gd".to_string(),
            content: r#"extends Control

# 企业管理组件
# 用于管理企业资源和流程

@onready var employees_list = $VBoxContainer/EmployeesList
@onready var projects_list = $VBoxContainer/ProjectsList
@onready var tasks_list = $VBoxContainer/TasksList

func _ready():
	# 初始化企业管理
	load_employees()
	load_projects()
	load_tasks()

func load_employees():
	# 加载员工列表
	# 实际应用中会从API获取数据
	var employees = [
		{"id": 1, "name": "John Doe", "position": "CEO", "department": "Executive"},
		{"id": 2, "name": "Jane Smith", "position": "CTO", "department": "Technology"},
		{"id": 3, "name": "Bob Johnson", "position": "CFO", "department": "Finance"}
	]
	
	# 填充员工列表
	for employee in employees:
		var employee_item = preload("res://components/EmployeeItem.gd").new()
		employee_item.setup(employee)
		employees_list.add_child(employee_item)

func load_projects():
	# 加载项目列表
	# 实际应用中会从API获取数据
	var projects = [
		{"id": 1, "name": "Website Redesign", "status": "In Progress", "deadline": "2024-02-01"},
		{"id": 2, "name": "Mobile App Development", "status": "Planning", "deadline": "2024-03-15"},
		{"id": 3, "name": "Marketing Campaign", "status": "Completed", "deadline": "2024-01-15"}
	]
	
	# 填充项目列表
	for project in projects:
		var project_item = preload("res://components/ProjectItem.gd").new()
		project_item.setup(project)
		projects_list.add_child(project_item)

func load_tasks():
	# 加载任务列表
	# 实际应用中会从API获取数据
	var tasks = [
		{"id": 1, "name": "Design homepage", "status": "To Do", "assignee": "Jane Smith"},
		{"id": 2, "name": "Implement login system", "status": "In Progress", "assignee": "Bob Johnson"},
		{"id": 3, "name": "Write documentation", "status": "Review", "assignee": "John Doe"}
	]
	
	# 填充任务列表
	for task in tasks:
		var task_item = preload("res://components/TaskItem.gd").new()
		task_item.setup(task)
		tasks_list.add_child(task_item)
"#
            .to_string(),
            props: HashMap::from([
                (
                    "title".to_string(),
                    serde_json::Value::String("Enterprise Management".to_string()),
                ),
                (
                    "company_name".to_string(),
                    serde_json::Value::String("Acme Corp".to_string()),
                ),
                (
                    "default_view".to_string(),
                    serde_json::Value::String("dashboard".to_string()),
                ),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "EnterpriseTheme".to_string(),
            path: "styles/enterprise_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://f2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/enterprise_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.2, 0.2, 0.2, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "enterprise_config.json".to_string(),
            r#"{
  "company_name": "Acme Corp",
  "version": "1.0.0",
  "api_endpoint": "https://api.enterprise.com",
  "modules": ["employees", "projects", "tasks", "documents"],
  "theme": "professional"
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "company_logo".to_string(),
                path: "icons/company_logo.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "employee_icon".to_string(),
                path: "icons/employee_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "enterprise-management".to_string(),
            name: "Enterprise Management".to_string(),
            description: "A comprehensive enterprise management template for managing employees, projects, and tasks".to_string(),
            industry: "Enterprise".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![management_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建医疗健康门户模板
    fn create_healthcare_portal_template(&self) -> GufTemplate {
        let healthcare_component = GufComponentTemplate {
            name: "HealthcarePortal".to_string(),
            component_type: "Portal".to_string(),
            path: "components/HealthcarePortal.gd".to_string(),
            content: r#"extends Control

# 医疗健康门户组件
# 用于管理医疗健康数据和服务

@onready var patient_info = $VBoxContainer/PatientInfo
@onready var appointments_list = $VBoxContainer/AppointmentsList
@onready var medical_records_list = $VBoxContainer/MedicalRecordsList

func _ready():
	# 初始化医疗健康门户
	load_patient_info()
	load_appointments()
	load_medical_records()

func load_patient_info():
	# 加载患者信息
	# 实际应用中会从API获取数据
	var patient = {"id": 1, "name": "John Doe", "age": 30, "gender": "Male"}
	patient_info.text = "Patient: %s (Age: %d, Gender: %s)" % [patient.name, patient.age, patient.gender]

func load_appointments():
	# 加载预约列表
	# 实际应用中会从API获取数据
	var appointments = [
		{"id": 1, "doctor": "Dr. Smith", "date": "2024-01-20", "time": "10:00 AM", "reason": "Routine Checkup"},
		{"id": 2, "doctor": "Dr. Johnson", "date": "2024-02-05", "time": "2:30 PM", "reason": "Follow-up"}
	]
	
	# 填充预约列表
	for appointment in appointments:
		var appointment_item = preload("res://components/AppointmentItem.gd").new()
		appointment_item.setup(appointment)
		appointments_list.add_child(appointment_item)

func load_medical_records():
	# 加载医疗记录
	# 实际应用中会从API获取数据
	var records = [
		{"id": 1, "date": "2023-12-15", "doctor": "Dr. Smith", "diagnosis": "Common Cold", "treatment": "Rest and fluids"},
		{"id": 2, "date": "2023-11-10", "doctor": "Dr. Johnson", "diagnosis": "Flu", "treatment": "Antiviral medication"}
	]
	
	# 填充医疗记录
	for record in records:
		var record_item = preload("res://components/MedicalRecordItem.gd").new()
		record_item.setup(record)
		medical_records_list.add_child(record_item)
"#.to_string(),
            props: HashMap::from([
                ("title".to_string(), serde_json::Value::String("Healthcare Portal".to_string())),
                ("hospital_name".to_string(), serde_json::Value::String("MediCare Hospital".to_string())),
                ("security_level".to_string(), serde_json::Value::String("high".to_string())),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "HealthcareTheme".to_string(),
            path: "styles/healthcare_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://a2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/healthcare_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.2, 0.2, 0.2, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "healthcare_config.json".to_string(),
            r#"{
  "hospital_name": "MediCare Hospital",
  "version": "1.0.0",
  "api_endpoint": "https://api.healthcare.com",
  "security_level": "high",
  "encryption_enabled": true,
  "theme": "clean"
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "hospital_logo".to_string(),
                path: "icons/hospital_logo.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "medical_icon".to_string(),
                path: "icons/medical_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "healthcare-portal".to_string(),
            name: "Healthcare Portal".to_string(),
            description:
                "A secure healthcare portal template for managing patient data and appointments"
                    .to_string(),
            industry: "Healthcare".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![healthcare_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建教育平台模板
    fn create_education_platform_template(&self) -> GufTemplate {
        let education_component = GufComponentTemplate {
            name: "EducationPlatform".to_string(),
            component_type: "Platform".to_string(),
            path: "components/EducationPlatform.gd".to_string(),
            content: r#"extends Control

# 教育平台组件
# 用于提供在线教育和学习资源

@onready var courses_list = $VBoxContainer/CoursesList
@onready var lessons_list = $VBoxContainer/LessonsList
@onready var progress_bar = $VBoxContainer/ProgressBar

func _ready():
	# 初始化教育平台
	load_courses()
	load_lessons()
	update_progress()

func load_courses():
	# 加载课程列表
	# 实际应用中会从API获取数据
	var courses = [
		{"id": 1, "name": "Introduction to Programming", "instructor": "Dr. Smith", "progress": 75},
		{"id": 2, "name": "Mathematics Fundamentals", "instructor": "Prof. Johnson", "progress": 45},
		{"id": 3, "name": "English Literature", "instructor": "Dr. Brown", "progress": 90}
	]
	
	# 填充课程列表
	for course in courses:
		var course_item = preload("res://components/CourseItem.gd").new()
		course_item.setup(course)
		courses_list.add_child(course_item)

func load_lessons():
	# 加载课程内容
	# 实际应用中会从API获取数据
	var lessons = [
		{"id": 1, "name": "Variables and Data Types", "completed": true},
		{"id": 2, "name": "Control Structures", "completed": true},
		{"id": 3, "name": "Functions", "completed": false},
		{"id": 4, "name": "Arrays and Objects", "completed": false}
	]
	
	# 填充课程内容
	for lesson in lessons:
		var lesson_item = preload("res://components/LessonItem.gd").new()
		lesson_item.setup(lesson)
		lessons_list.add_child(lesson_item)

func update_progress():
	# 更新进度
	progress_bar.value = 75
"#
            .to_string(),
            props: HashMap::from([
                (
                    "title".to_string(),
                    serde_json::Value::String("Education Platform".to_string()),
                ),
                (
                    "school_name".to_string(),
                    serde_json::Value::String("EduLearn Academy".to_string()),
                ),
                (
                    "student_name".to_string(),
                    serde_json::Value::String("John Doe".to_string()),
                ),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "EducationTheme".to_string(),
            path: "styles/education_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://b2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/education_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.2, 0.2, 0.2, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "education_config.json".to_string(),
            r#"{
  "school_name": "EduLearn Academy",
  "version": "1.0.0",
  "api_endpoint": "https://api.education.com",
  "courses_per_page": 10,
  "enable_chat": true,
  "theme": "academic"
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "school_logo".to_string(),
                path: "icons/school_logo.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "book_icon".to_string(),
                path: "icons/book_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "education-platform".to_string(),
            name: "Education Platform".to_string(),
            description: "A comprehensive education platform template for online learning and course management".to_string(),
            industry: "Education".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![education_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建房地产模板
    fn create_real_estate_template(&self) -> GufTemplate {
        let real_estate_component = GufComponentTemplate {
            name: "RealEstateListing".to_string(),
            component_type: "Listing".to_string(),
            path: "components/RealEstateListing.gd".to_string(),
            content: r#"extends Control

# 房地产列表组件
# 用于展示和管理房地产 listings

@onready var properties_grid = $VBoxContainer/PropertiesGrid
@onready var search_bar = $VBoxContainer/SearchBar
@onready var filters_panel = $VBoxContainer/FiltersPanel

func _ready():
	# 初始化房地产列表
	load_properties()
	setup_search()
	setup_filters()

func load_properties():
	# 加载房地产列表
	# 实际应用中会从API获取数据
	var properties = [
		{"id": 1, "title": "Modern Apartment", "price": 500000, "bedrooms": 3, "bathrooms": 2, "location": "Downtown"},
		{"id": 2, "title": "Spacious House", "price": 800000, "bedrooms": 4, "bathrooms": 3, "location": "Suburbs"},
		{"id": 3, "title": "Luxury Condo", "price": 1200000, "bedrooms": 2, "bathrooms": 2, "location": "Uptown"}
	]
	
	# 填充房地产列表
	for property in properties:
		var property_item = preload("res://components/PropertyItem.gd").new()
		property_item.setup(property)
		properties_grid.add_child(property_item)

func setup_search():
	# 设置搜索功能
	search_bar.placeholder_text = "Search properties..."

func setup_filters():
	# 设置过滤器
	# 实际应用中会有更多过滤选项
	filters_panel.visible = true
"#.to_string(),
            props: HashMap::from([
                ("title".to_string(), serde_json::Value::String("Real Estate Listings".to_string())),
                ("agency_name".to_string(), serde_json::Value::String("Prime Realty".to_string())),
                ("currency".to_string(), serde_json::Value::String("USD".to_string())),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "RealEstateTheme".to_string(),
            path: "styles/real_estate_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://c2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/real_estate_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.2, 0.2, 0.2, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "real_estate_config.json".to_string(),
            r#"{
  "agency_name": "Prime Realty",
  "version": "1.0.0",
  "api_endpoint": "https://api.realestate.com",
  "currency": "USD",
  "properties_per_page": 12,
  "theme": "modern"
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "agency_logo".to_string(),
                path: "icons/agency_logo.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "home_icon".to_string(),
                path: "icons/home_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "real-estate".to_string(),
            name: "Real Estate Listings".to_string(),
            description: "A modern real estate template for property listings and management"
                .to_string(),
            industry: "Real Estate".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![real_estate_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }

    /// 创建能源管理模板
    fn create_energy_management_template(&self) -> GufTemplate {
        let energy_component = GufComponentTemplate {
            name: "EnergyManagement".to_string(),
            component_type: "Management".to_string(),
            path: "components/EnergyManagement.gd".to_string(),
            content: r#"extends Control

# 能源管理组件
# 用于监控和管理能源使用

@onready var consumption_label = $VBoxContainer/ConsumptionLabel
@onready var production_label = $VBoxContainer/ProductionLabel
@onready var savings_label = $VBoxContainer/SavingsLabel
@onready var energy_chart = $VBoxContainer/EnergyChart

func _ready():
	# 初始化能源管理
	update_consumption()
	update_production()
	update_savings()
	update_chart()

func update_consumption():
	# 更新能源消耗
	consumption_label.text = "Consumption: 1,200 kWh"

func update_production():
	# 更新能源生产
	production_label.text = "Production: 800 kWh"

func update_savings():
	# 更新能源节省
	savings_label.text = "Savings: $150"

func update_chart():
	# 更新能源图表
	energy_chart.text = "Energy Usage: Last 30 Days"
"#
            .to_string(),
            props: HashMap::from([
                (
                    "title".to_string(),
                    serde_json::Value::String("Energy Management".to_string()),
                ),
                (
                    "facility_name".to_string(),
                    serde_json::Value::String("Green Facility".to_string()),
                ),
                (
                    "default_view".to_string(),
                    serde_json::Value::String("dashboard".to_string()),
                ),
            ]),
        };

        let style_template = GufStyleTemplate {
            name: "EnergyTheme".to_string(),
            path: "styles/energy_theme.tres".to_string(),
            content: r#"[gd_resource type="Theme" load_steps=2 format=3 uid="uid://d2a1b2c3d4e5f6"]

[ext_resource type="StyleBoxFlat" path="res://styles/energy_stylebox.tres" id="StyleBoxFlat_1"]

[resource]
button/theme_override_styles/button = ExtResource("StyleBoxFlat_1")
label/theme_override_colors/font_color = Color(0.2, 0.2, 0.2, 1)
"#
            .to_string(),
            style_type: "theme".to_string(),
        };

        let mut config_templates = HashMap::new();
        config_templates.insert(
            "energy_config.json".to_string(),
            r#"{
  "facility_name": "Green Facility",
  "version": "1.0.0",
  "api_endpoint": "https://api.energy.com",
  "refresh_interval": 60000,
  "enable_alerts": true,
  "theme": "green"
}
"#
            .to_string(),
        );

        let resource_files = vec![
            GufResourceFile {
                name: "energy_logo".to_string(),
                path: "icons/energy_logo.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
            GufResourceFile {
                name: "solar_icon".to_string(),
                path: "icons/solar_icon.svg".to_string(),
                resource_type: "image".to_string(),
                content: None,
            },
        ];

        GufTemplate {
            id: "energy-management".to_string(),
            name: "Energy Management".to_string(),
            description: "An energy management template for monitoring and optimizing energy usage"
                .to_string(),
            industry: "Energy".to_string(),
            guf_version: "4.4.0".to_string(),
            frontend_framework: "Godot".to_string(),
            component_templates: vec![energy_component],
            style_templates: vec![style_template],
            config_templates,
            resource_files,
        }
    }
}

/// 创建默认的 GUF 模板库
pub fn default_template_library() -> GufTemplateLibrary {
    GufTemplateLibrary::new()
}

/// 示例使用
pub fn example_usage() {
    let library = default_template_library();

    // 获取所有模板
    let templates = library.get_all_templates();
    println!("Available GUF templates:");
    for template in templates {
        println!(
            "- {}: {} ({})\n",
            template.id, template.name, template.industry
        );
    }

    // 获取特定行业的模板
    let finance_templates = library.get_templates_by_industry("Finance");
    println!("Finance templates:");
    for template in finance_templates {
        println!("- {}: {}", template.id, template.name);
    }
}
