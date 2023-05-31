use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn test_create_list_and_delete() {
    let vault_name = "integration_test_certificate_create";
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("vault")
        .arg("create")
        .arg(vault_name)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        format!(
            "Creating vault {}\nVault created successfully\n",
            vault_name
        )
        .into_bytes()
    );

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("vault")
        .arg("list")
        .output()
        .expect("Failed to execute command");

    let out = String::from_utf8(output.stdout).expect("Not a UTF8 string");

    assert!(output.status.success());
    assert!(out.contains(vault_name));

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("vault")
        .arg("delete")
        .arg(vault_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        let res = stdin.write_all(b"y\n");

        res.unwrap_or_else(|e| println!("{}", e));
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        format!(
            "Deleting vault: {}\nThis will delete all secrets and certiticates in the vault\nAre you sure? (y/N)\nVaults deleted\n",
            vault_name
        )
        .into_bytes()
    );
}
