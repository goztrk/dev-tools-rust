use std::process::{Command, self};

pub fn run_cmd(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to run process");

    if output.status.success() == false {
        let string_args = args.join(" ");
        eprintln!("Failed to run command {:?}", [cmd, &string_args].join(" "));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        process::exit(1);
    }

    String::from_utf8(output.stdout).expect("Failed to decode stderr")
}
