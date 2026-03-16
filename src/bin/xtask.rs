use std::error::Error;
use std::process::{Command, ExitCode};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_default();

    let status = match command.as_str() {
        "protoc-setup" => run_protoc_setup()?,
        "protoc-clean" => run_protoc_clean()?,
        _ => {
            eprintln!("Uso: cargo xtask <protoc-setup|protoc-clean>");
            return Ok(ExitCode::from(2));
        }
    };

    Ok(if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(status.code().unwrap_or(1) as u8)
    })
}

fn run_protoc_setup() -> Result<std::process::ExitStatus, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                ".\\scripts\\protoc-setup.ps1",
            ])
            .status()?)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(Command::new("make").arg("protoc-setup").status()?)
    }
}

fn run_protoc_clean() -> Result<std::process::ExitStatus, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                ".\\scripts\\protoc-clean.ps1",
            ])
            .status()?)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(Command::new("make").arg("protoc-clean").status()?)
    }
}
