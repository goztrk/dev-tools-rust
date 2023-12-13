use clap::{ArgAction, Args};
use glob::glob;
use regex::Regex;
use std::path::Path;
use std::process::{self, Command};

#[derive(Args, Debug)]
pub struct PkgCmd {
    /// Compile requirements*.in files into requirements*.txt using `pip-tools`
    #[arg(short, long, action = ArgAction::SetTrue)]
    compile: Option<bool>,

    /// Install requirements.txt files
    #[arg(short, long, action = ArgAction::SetTrue)]
    install: Option<bool>,
}

impl PkgCmd {
    pub fn run(args: &PkgCmd) {
        PkgCmd::preprocess();

        if args.compile.unwrap_or(false) {
            PkgCmd::run_cmd("venv/bin/python", &["-m", "pip", "install", "pip-tools"]);
            println!("pip-tools installed");

            let mut file_found = false;
            for entry in glob("./requirements*.in").expect("Failed to get requirements*.in files") {
                if let Ok(path) = entry {
                    file_found = true;
                    let source = path.to_str().unwrap();
                    let mut out = path.clone();
                    out.set_extension("txt");
                    let output = out.to_str().unwrap();
                    PkgCmd::run_cmd(
                        "venv/bin/pip-compile",
                        &[
                            "--resolver=backtracking",
                            source,
                            "--output-file",
                            output,
                            "--no-strip-extras",
                        ],
                    );
                    println!("{:?} file processed", source);
                }
            }

            if !file_found {
                eprintln!("No requirements file found.");
            }
        }
        if args.install.unwrap_or(false) {
            let mut file_found = false;
            for entry in glob("./requirements*.txt").expect("Failed to get requirements*.txt files") {
                if let Ok(path) = entry {
                    file_found = true;
                    let filename = path.to_str().unwrap();
                    PkgCmd::run_cmd("venv/bin/python", &["-m", "pip", "install", "-r", filename]);
                    println!("Packages in {:?} are now installed", filename);
                }
            }

            if !file_found {
                eprintln!("No requirements*.txt file found.");
            }
        }
    }

    fn run_cmd(cmd: &str, args: &[&str]) -> String {
        let string_args = args.join(" ");
        let output = Command::new(cmd)
            .args(args)
            .output()
            .expect("Failed to run command");

        if output.status.success() == false {
            eprintln!("Failed to run command {:?}", [cmd, &string_args].join(" "));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            process::exit(1);
        }

        String::from_utf8(output.stdout).unwrap()
    }

    fn preprocess() {
        // First check if `venv` folder exists
        if Path::new("venv").exists() == false {
            eprintln!("\"venv\" folder could not be found");
            process::exit(0)
        }

        let output = PkgCmd::run_cmd("venv/bin/python", &["-m", "pip", "install", "--upgrade", "pip"]);

        let re_version = Regex::new(r"\((?P<version>\d+\.\d+\.\d+)\)").unwrap();
        let re_new = Regex::new(r"Successfully installed pip-(?P<version>\d+\.\d+\.\d+)").unwrap();

        let version = &re_version.captures(&output).unwrap()["version"];
        if let Some(caps) = re_new.captures(&output) {
            println!("PIP upgraded from {:?} to {:?}", version, &caps["version"])
        } else {
            println!("PIP {} is up to date", version)
        }
    }
}
