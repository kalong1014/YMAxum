@echo off
setlocal enabledelayedexpansion

REM Set log file
set LOG_FILE=pre-release-cleanup.log

REM Clear log file
echo. > %LOG_FILE%
echo =============================== >> %LOG_FILE%
echo YMAxum Pre-Release Cleanup Script >> %LOG_FILE%
echo Execution Time: %date% %time% >> %LOG_FILE%
echo =============================== >> %LOG_FILE%

REM Display start message
echo YMAxum Pre-Release Cleanup Script
echo Execution Time: %date% %time%
echo Log File: %LOG_FILE%
echo ===============================
echo Usage: pre-release-cleanup.bat [OPTIONS]
echo Options:
echo   /keep-docs     Keep MD documentation files (for framework reuse)
echo   /clean-all     Clean all files including documentation

echo ===============================

REM Define directories and files to clean
set BUILD_DIR=target
set NODE_MODULES=node_modules
set LOG_DIRS=logs
set TEMP_DIRS=tmp temp .tmp .temp
set CACHE_DIRS=.cache cache
set DEBUG_FILES=*.pdb *.ilk *.exp *.lib *.bak
set PLUGIN_TARGET_DIRS=plugins\analytics\target plugins\monitoring\target
set TEST_OUTPUT_DIR=test_output
set PERFORMANCE_RESULTS_DIR=performance_results
set DEV_DOCUMENTS_DIR=.trae\documents
set DIST_DIR=dist
set VSCODE_DIR=.vscode
set DOCKER_DIR=docker
set HELM_DIR=helm
set K8S_DIR=k8s
set EXAMPLES_DIR=examples
set KEYS_DIR=keys
set ENV_FILE=.env.example
set CARGO_LOCK=Cargo.lock
set GITHUB_DIR=.github
set DOC_FILES=*.md

REM Parse command line arguments
set KEEP_DOCS=false
set CLEAN_ALL=false

:PARSE_ARGS
if "%1"=="/keep-docs" set KEEP_DOCS=true & shift & goto PARSE_ARGS
if "%1"=="/clean-all" set CLEAN_ALL=true & shift & goto PARSE_ARGS
if not "%1"=="" (
    echo Warning: Unknown argument %1 >> %LOG_FILE%
    echo Warning: Unknown argument %1
    shift
    goto PARSE_ARGS
)

REM Check if current directory is project root
if not exist "src" (
    echo Error: Current directory is not project root. Please run this script in YMAxum project root directory. >> %LOG_FILE%
    echo Error: Current directory is not project root. Please run this script in YMAxum project root directory.
    goto END
)

REM Clean build directory
echo Cleaning build directory... >> %LOG_FILE%
echo Cleaning build directory...
if exist "%BUILD_DIR%" (
    rd /s /q "%BUILD_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned build directory %BUILD_DIR% >> %LOG_FILE%
        echo Success: Cleaned build directory %BUILD_DIR%
    ) else (
        echo Warning: Error cleaning build directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning build directory. Continuing with next steps.
    )
) else (
    echo Info: Build directory %BUILD_DIR% does not exist >> %LOG_FILE%
    echo Info: Build directory %BUILD_DIR% does not exist
)

REM Clean node_modules directory
echo Cleaning node_modules directory... >> %LOG_FILE%
echo Cleaning node_modules directory...
if exist "%NODE_MODULES%" (
    rd /s /q "%NODE_MODULES%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned node_modules directory >> %LOG_FILE%
        echo Success: Cleaned node_modules directory
    ) else (
        echo Warning: Error cleaning node_modules directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning node_modules directory. Continuing with next steps.
    )
) else (
    echo Info: node_modules directory does not exist >> %LOG_FILE%
    echo Info: node_modules directory does not exist
)

REM Clean log directories
echo Cleaning log directories... >> %LOG_FILE%
echo Cleaning log directories...
for %%d in (%LOG_DIRS%) do (
    if exist "%%d" (
        rd /s /q "%%d" 2>> %LOG_FILE%
        if !errorlevel! equ 0 (
            echo Success: Cleaned log directory %%d >> %LOG_FILE%
            echo Success: Cleaned log directory %%d
        ) else (
            echo Warning: Error cleaning log directory %%d. Continuing with next steps. >> %LOG_FILE%
            echo Warning: Error cleaning log directory %%d. Continuing with next steps.
        )
    )
)

REM Clean temporary directories
echo Cleaning temporary directories... >> %LOG_FILE%
echo Cleaning temporary directories...
for %%d in (%TEMP_DIRS%) do (
    if exist "%%d" (
        rd /s /q "%%d" 2>> %LOG_FILE%
        if !errorlevel! equ 0 (
            echo Success: Cleaned temporary directory %%d >> %LOG_FILE%
            echo Success: Cleaned temporary directory %%d
        ) else (
            echo Warning: Error cleaning temporary directory %%d. Continuing with next steps. >> %LOG_FILE%
            echo Warning: Error cleaning temporary directory %%d. Continuing with next steps.
        )
    )
)

REM Clean cache directories
echo Cleaning cache directories... >> %LOG_FILE%
echo Cleaning cache directories...
for %%d in (%CACHE_DIRS%) do (
    if exist "%%d" (
        rd /s /q "%%d" 2>> %LOG_FILE%
        if !errorlevel! equ 0 (
            echo Success: Cleaned cache directory %%d >> %LOG_FILE%
            echo Success: Cleaned cache directory %%d
        ) else (
            echo Warning: Error cleaning cache directory %%d. Continuing with next steps. >> %LOG_FILE%
            echo Warning: Error cleaning cache directory %%d. Continuing with next steps.
        )
    )
)

REM Clean debug files
echo Cleaning debug files... >> %LOG_FILE%
echo Cleaning debug files...
for %%f in (%DEBUG_FILES%) do (
    del /s /q "%%f" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned debug files %%f >> %LOG_FILE%
        echo Success: Cleaned debug files %%f
    ) else (
        echo Warning: Error cleaning debug files %%f. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning debug files %%f. Continuing with next steps.
    )
)

REM Clean log files
echo Cleaning log files... >> %LOG_FILE%
echo Cleaning log files...
del /s /q "*.log" 2>> %LOG_FILE%
if !errorlevel! equ 0 (
    echo Success: Cleaned log files >> %LOG_FILE%
    echo Success: Cleaned log files
) else (
    echo Warning: Error cleaning log files. Continuing with next steps. >> %LOG_FILE%
    echo Warning: Error cleaning log files. Continuing with next steps.
)

REM Clean plugin target directories
echo Cleaning plugin target directories... >> %LOG_FILE%
echo Cleaning plugin target directories...
for %%d in (%PLUGIN_TARGET_DIRS%) do (
    if exist "%%d" (
        rd /s /q "%%d" 2>> %LOG_FILE%
        if !errorlevel! equ 0 (
            echo Success: Cleaned plugin target directory %%d >> %LOG_FILE%
            echo Success: Cleaned plugin target directory %%d
        ) else (
            echo Warning: Error cleaning plugin target directory %%d. Continuing with next steps. >> %LOG_FILE%
            echo Warning: Error cleaning plugin target directory %%d. Continuing with next steps.
        )
    )
)

REM Clean test output directory
echo Cleaning test output directory... >> %LOG_FILE%
echo Cleaning test output directory...
if exist "%TEST_OUTPUT_DIR%" (
    rd /s /q "%TEST_OUTPUT_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned test output directory %TEST_OUTPUT_DIR% >> %LOG_FILE%
        echo Success: Cleaned test output directory %TEST_OUTPUT_DIR%
    ) else (
        echo Warning: Error cleaning test output directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning test output directory. Continuing with next steps.
    )
) else (
    echo Info: Test output directory %TEST_OUTPUT_DIR% does not exist >> %LOG_FILE%
    echo Info: Test output directory %TEST_OUTPUT_DIR% does not exist
)

REM Clean performance results directory
echo Cleaning performance results directory... >> %LOG_FILE%
echo Cleaning performance results directory...
if exist "%PERFORMANCE_RESULTS_DIR%" (
    rd /s /q "%PERFORMANCE_RESULTS_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned performance results directory %PERFORMANCE_RESULTS_DIR% >> %LOG_FILE%
        echo Success: Cleaned performance results directory %PERFORMANCE_RESULTS_DIR%
    ) else (
        echo Warning: Error cleaning performance results directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning performance results directory. Continuing with next steps.
    )
) else (
    echo Info: Performance results directory %PERFORMANCE_RESULTS_DIR% does not exist >> %LOG_FILE%
    echo Info: Performance results directory %PERFORMANCE_RESULTS_DIR% does not exist
)

REM Clean development documents directory
echo Cleaning development documents directory... >> %LOG_FILE%
echo Cleaning development documents directory...
if exist "%DEV_DOCUMENTS_DIR%" (
    rd /s /q "%DEV_DOCUMENTS_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned development documents directory %DEV_DOCUMENTS_DIR% >> %LOG_FILE%
        echo Success: Cleaned development documents directory %DEV_DOCUMENTS_DIR%
    ) else (
        echo Warning: Error cleaning development documents directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning development documents directory. Continuing with next steps.
    )
) else (
    echo Info: Development documents directory %DEV_DOCUMENTS_DIR% does not exist >> %LOG_FILE%
    echo Info: Development documents directory %DEV_DOCUMENTS_DIR% does not exist
)

REM Clean additional directories
REM Clean dist directory
echo Cleaning dist directory... >> %LOG_FILE%
echo Cleaning dist directory...
if exist "%DIST_DIR%" (
    rd /s /q "%DIST_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned dist directory %DIST_DIR% >> %LOG_FILE%
        echo Success: Cleaned dist directory %DIST_DIR%
    ) else (
        echo Warning: Error cleaning dist directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning dist directory. Continuing with next steps.
    )
) else (
    echo Info: Dist directory %DIST_DIR% does not exist >> %LOG_FILE%
    echo Info: Dist directory %DIST_DIR% does not exist
)

REM Clean VSCode directory
echo Cleaning VSCode directory... >> %LOG_FILE%
echo Cleaning VSCode directory...
if exist "%VSCODE_DIR%" (
    rd /s /q "%VSCODE_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned VSCode directory %VSCODE_DIR% >> %LOG_FILE%
        echo Success: Cleaned VSCode directory %VSCODE_DIR%
    ) else (
        echo Warning: Error cleaning VSCode directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning VSCode directory. Continuing with next steps.
    )
) else (
    echo Info: VSCode directory %VSCODE_DIR% does not exist >> %LOG_FILE%
    echo Info: VSCode directory %VSCODE_DIR% does not exist
)

REM Clean Docker directory
echo Cleaning Docker directory... >> %LOG_FILE%
echo Cleaning Docker directory...
if exist "%DOCKER_DIR%" (
    rd /s /q "%DOCKER_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned Docker directory %DOCKER_DIR% >> %LOG_FILE%
        echo Success: Cleaned Docker directory %DOCKER_DIR%
    ) else (
        echo Warning: Error cleaning Docker directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning Docker directory. Continuing with next steps.
    )
) else (
    echo Info: Docker directory %DOCKER_DIR% does not exist >> %LOG_FILE%
    echo Info: Docker directory %DOCKER_DIR% does not exist
)

REM Clean Helm directory
echo Cleaning Helm directory... >> %LOG_FILE%
echo Cleaning Helm directory...
if exist "%HELM_DIR%" (
    rd /s /q "%HELM_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned Helm directory %HELM_DIR% >> %LOG_FILE%
        echo Success: Cleaned Helm directory %HELM_DIR%
    ) else (
        echo Warning: Error cleaning Helm directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning Helm directory. Continuing with next steps.
    )
) else (
    echo Info: Helm directory %HELM_DIR% does not exist >> %LOG_FILE%
    echo Info: Helm directory %HELM_DIR% does not exist
)

REM Clean K8s directory
echo Cleaning K8s directory... >> %LOG_FILE%
echo Cleaning K8s directory...
if exist "%K8S_DIR%" (
    rd /s /q "%K8S_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned K8s directory %K8S_DIR% >> %LOG_FILE%
        echo Success: Cleaned K8s directory %K8S_DIR%
    ) else (
        echo Warning: Error cleaning K8s directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning K8s directory. Continuing with next steps.
    )
) else (
    echo Info: K8s directory %K8S_DIR% does not exist >> %LOG_FILE%
    echo Info: K8s directory %K8S_DIR% does not exist
)

REM Clean examples directory
echo Cleaning examples directory... >> %LOG_FILE%
echo Cleaning examples directory...
if exist "%EXAMPLES_DIR%" (
    rd /s /q "%EXAMPLES_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned examples directory %EXAMPLES_DIR% >> %LOG_FILE%
        echo Success: Cleaned examples directory %EXAMPLES_DIR%
    ) else (
        echo Warning: Error cleaning examples directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning examples directory. Continuing with next steps.
    )
) else (
    echo Info: Examples directory %EXAMPLES_DIR% does not exist >> %LOG_FILE%
    echo Info: Examples directory %EXAMPLES_DIR% does not exist
)

REM Clean keys directory
echo Cleaning keys directory... >> %LOG_FILE%
echo Cleaning keys directory...
if exist "%KEYS_DIR%" (
    rd /s /q "%KEYS_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned keys directory %KEYS_DIR% >> %LOG_FILE%
        echo Success: Cleaned keys directory %KEYS_DIR%
    ) else (
        echo Warning: Error cleaning keys directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning keys directory. Continuing with next steps.
    )
) else (
    echo Info: Keys directory %KEYS_DIR% does not exist >> %LOG_FILE%
    echo Info: Keys directory %KEYS_DIR% does not exist
)

REM Clean GitHub directory
echo Cleaning GitHub directory... >> %LOG_FILE%
echo Cleaning GitHub directory...
if exist "%GITHUB_DIR%" (
    rd /s /q "%GITHUB_DIR%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned GitHub directory %GITHUB_DIR% >> %LOG_FILE%
        echo Success: Cleaned GitHub directory %GITHUB_DIR%
    ) else (
        echo Warning: Error cleaning GitHub directory. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning GitHub directory. Continuing with next steps.
    )
) else (
    echo Info: GitHub directory %GITHUB_DIR% does not exist >> %LOG_FILE%
    echo Info: GitHub directory %GITHUB_DIR% does not exist
)

REM Clean individual files
echo Cleaning individual files... >> %LOG_FILE%
echo Cleaning individual files...

REM Clean env.example file
if exist "%ENV_FILE%" (
    del /q "%ENV_FILE%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned env.example file >> %LOG_FILE%
        echo Success: Cleaned env.example file
    ) else (
        echo Warning: Error cleaning env.example file. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning env.example file. Continuing with next steps.
    )
) else (
    echo Info: env.example file does not exist >> %LOG_FILE%
    echo Info: env.example file does not exist
)

REM Clean Cargo.lock file
if exist "%CARGO_LOCK%" (
    del /q "%CARGO_LOCK%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned Cargo.lock file >> %LOG_FILE%
        echo Success: Cleaned Cargo.lock file
    ) else (
        echo Warning: Error cleaning Cargo.lock file. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning Cargo.lock file. Continuing with next steps.
    )
) else (
    echo Info: Cargo.lock file does not exist >> %LOG_FILE%
    echo Info: Cargo.lock file does not exist
)

REM Clean documentation files based on command line arguments
if "%CLEAN_ALL%"=="true" (
    echo Cleaning documentation files... >> %LOG_FILE%
    echo Cleaning documentation files...
    del /s /q "%DOC_FILES%" 2>> %LOG_FILE%
    if !errorlevel! equ 0 (
        echo Success: Cleaned documentation files >> %LOG_FILE%
        echo Success: Cleaned documentation files
    ) else (
        echo Warning: Error cleaning documentation files. Continuing with next steps. >> %LOG_FILE%
        echo Warning: Error cleaning documentation files. Continuing with next steps.
    )
) else if "%KEEP_DOCS%"=="false" (
    echo Info: Keeping documentation files (use /clean-all to remove them) >> %LOG_FILE%
    echo Info: Keeping documentation files (use /clean-all to remove them)
) else (
    echo Info: Keeping documentation files as requested >> %LOG_FILE%
    echo Info: Keeping documentation files as requested
)

REM Restore log file (keep current execution log)
echo =============================== >> %LOG_FILE%
echo Cleanup Completed! >> %LOG_FILE%
echo =============================== >> %LOG_FILE%

REM Display completion message
echo ===============================
echo Cleanup Completed!
echo Detailed log information is available in: %LOG_FILE%
echo ===============================

:END
echo Script execution finished.
pause
