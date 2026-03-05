.PHONY: help build test clean docs install lint fmt clippy coverage deps generate_tests code_quality_check cross_platform_test performance_monitor release generate_plugin_template security_scan generate_config generate_test_report performance_optimization

help:
	@echo "YMAxum开发工具"
	@echo ""
	@echo "可用命令:"
	@echo "  make build        - 构建项目"
	@echo "  make test         - 运行测试"
	@echo "  make clean        - 清理构建文件"
	@echo "  make docs         - 生成文档"
	@echo "  make install      - 安装项目"
	@echo "  make lint         - 运行代码检查"
	@echo "  make fmt          - 格式化代码"
	@echo "  make clippy       - 运行clippy"
	@echo "  make coverage     - 生成代码覆盖率报告"
	@echo "  make deps         - 管理依赖"
	@echo "  make generate_tests - 生成测试用例"
	@echo "  make code_quality_check - 代码质量检查"
	@echo "  make cross_platform_test - 跨平台测试"
	@echo "  make performance_monitor - 性能监控"
	@echo "  make release - 发布项目"
	@echo "  make generate_plugin_template - 生成插件模板"
	@echo "  make security_scan - 安全扫描"
	@echo "  make generate_config - 生成配置文件"
	@echo "  make generate_test_report - 生成测试报告"
	@echo "  make performance_optimization - 性能优化和基准测试"

build:
	cargo build --release

test:
	cargo test --lib -- --nocapture
	cargo test --test integration -- --nocapture

clean:
	cargo clean

docs:
	@echo "生成API文档..."
	cargo doc --no-deps --document-private-items
	@echo "复制文档到docs目录..."
	@if not exist "docs\api" mkdir docs\api
	@xcopy /E /I /Y "target\doc\*" "docs\api\"
	@echo "文档生成完成！"

install:
	cargo install --path .

lint:
	cargo fmt --check
	cargo clippy -- -D warnings

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

coverage:
	@echo "生成代码覆盖率报告..."
	@if not exist "target\coverage" mkdir target\coverage
	@echo "运行测试并生成覆盖率报告..."
	cargo tarpaulin --out Html --out Xml --out Lcov --output-dir target/coverage --timeout 300 --run-types Tests --exclude-files "tests/*,examples/*,benches/*" --exclude "tests::*,main" --report-uncoverable
	@echo "代码覆盖率报告生成完成！"
	@echo "HTML报告位置: target\coverage\index.html"

deps:
	@echo "管理依赖..."
	@echo "检查依赖更新..."
	cargo outdated || echo "cargo-outdated未安装，跳过"
	@echo "更新依赖..."
	cargo update
	@echo "审计依赖安全性..."
	cargo audit || echo "cargo-audit未安装，跳过"

generate_tests:
	@echo "生成测试用例..."
	@powershell -ExecutionPolicy Bypass -File scripts\generate_tests.ps1

code_quality_check:
	@echo "代码质量检查..."
	@powershell -ExecutionPolicy Bypass -File scripts\code_quality_check.ps1

cross_platform_test:
	@echo "跨平台测试..."
	@powershell -ExecutionPolicy Bypass -File scripts\cross_platform_test.ps1

performance_monitor:
	@echo "性能监控..."
	@powershell -ExecutionPolicy Bypass -File scripts\performance_monitor.ps1

release:
	@echo "发布项目..."
	@powershell -ExecutionPolicy Bypass -File scripts\release.ps1 $(VERSION)

generate_plugin_template:
	@echo "生成插件模板..."
	@powershell -ExecutionPolicy Bypass -File scripts\generate_plugin_template.ps1 $(PLUGIN_NAME) $(PLUGIN_VERSION) $(PLUGIN_AUTHOR) $(PLUGIN_DESCRIPTION) $(PLUGIN_TYPE) $(OUTPUT_DIR)

security_scan:
	@echo "安全扫描..."
	@powershell -ExecutionPolicy Bypass -File scripts\security_scan.ps1

generate_config:
	@echo "生成配置文件..."
	@powershell -ExecutionPolicy Bypass -File scripts\generate_config.ps1

generate_test_report:
	@echo "生成测试报告..."
	@powershell -ExecutionPolicy Bypass -File scripts/generate_test_report.ps1

performance_optimization:
	@echo "性能优化和基准测试..."
	@powershell -ExecutionPolicy Bypass -File scripts/performance_optimization.ps1

.PHONY: all
all: build test lint
