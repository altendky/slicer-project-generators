use slicer_project_generator_bambu_studio::runtime::{self, Package};
use std::process::ExitCode;

fn run() -> anyhow::Result<bool> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let root = std::env::current_dir()?;
    if args.len() != 4 || args[0] != "--request" || args[2] != "--result" {
        anyhow::bail!(
            "usage: slicer-project-generator-bambu-studio --request request.json --result result.json"
        );
    }
    match Package::load(&std::env::current_exe()?) {
        Ok(package) => runtime::invoke(&root, &args[1], &args[3], &package),
        Err(_) => {
            let result = runtime::failure_result("package_identity_invalid",slicer_project_generator_bambu_studio::generator_protocol::ErrorCategory::InternalError,None);
            runtime::write_installation_failure(&root, &args[3], &result)?;
            Ok(false)
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
