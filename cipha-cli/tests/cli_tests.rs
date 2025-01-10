use assert_cmd::Command;


#[test]
fn test_cli_version_args() {
    let mut cmd = Command::cargo_bin("cipha-cli").unwrap();
    cmd.arg("-V")
    .assert().success().stdout("cipha-cli 0.1.0\n");
}


#[test]
fn test_cli_encode_rot13() {
    let mut cmd = Command::cargo_bin("cipha-cli").unwrap();
    cmd.arg("encode")
        .arg("--cipher")
        .arg("rot13")
        .arg("--message")
        .arg("Hello, World!")
        .assert().success().stdout("Uryyb, Jbeyq!\n");
}

#[test]
fn test_cli_encode_caesar() {
    let mut cmd = Command::cargo_bin("cipha-cli").unwrap();
    cmd.arg("encode")
        .arg("--cipher")
        .arg("caesar")
        .arg("--message")
        .arg("Hello, World!")
        .arg("--shift")
        .arg("3")
        .assert().success().stdout("Khoor, Zruog!\n");
}

#[test]
fn test_cli_decode_rot13() {
    let mut cmd = Command::cargo_bin("cipha-cli").unwrap();
    cmd.arg("decode")
        .arg("--cipher")
        .arg("rot13")
        .arg("--message")
        .arg("Uryyb, Jbeyq!")
        .assert().success().stdout("Hello, World!\n");
}

#[test]
fn test_cli_decode_caesar() {
    let mut cmd = Command::cargo_bin("cipha-cli").unwrap();
    cmd.arg("decode")
        .arg("--cipher")
        .arg("caesar")
        .arg("--message")
        .arg("Khoor, Zruog!")
        .arg("--shift")
        .arg("3")
        .assert().success().stdout("Hello, World!\n");
}