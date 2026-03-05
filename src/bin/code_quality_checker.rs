//! Code quality checker binary
//! Checks code quality

use clap::Parser;
use std::path::Path;
use ymaxum::utils::code_quality_checker::CodeQualityChecker;

/// Code quality checker command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Project directory
    #[arg(short, long, default_value = ".")]
    project_dir: String,

    /// Output format (text, json, markdown)
    #[arg(short, long, default_value = "text")]
    output: String,

    /// Enable detailed analysis
    #[arg(long, default_value = "false")]
    detailed: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let project_dir = Path::new(&args.project_dir);

    println!("Starting code quality check...");
    println!("Project directory: {}", args.project_dir);
    println!("Output format: {}", args.output);
    println!("Detailed analysis: {}", args.detailed);
    println!("=");

    // Run code quality check
    if let Err(e) = CodeQualityChecker::check_code_quality(project_dir) {
        eprintln!("Error checking code quality: {}", e);
        std::process::exit(1);
    }

    println!("Code quality check completed successfully");
    println!("Report generated: code_quality_report.md");

    // Print summary based on output format
    match args.output.as_str() {
        "json" => {
            println!("=");
            println!("{{");
            println!("  \"status\": \"success\",");
            println!("  \"message\": \"Code quality check completed\",");
            println!("  \"report_path\": \"code_quality_report.md\"");
            println!("}}");
        }
        "text" | _ => {
            println!("=");
            println!("Summary:");
            println!("- Code quality check completed");
            println!("- Report saved to code_quality_report.md");
            println!("- Check details available in the report");
        }
    }
}
