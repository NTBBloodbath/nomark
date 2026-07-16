use std::process::Command;

#[test]
fn help_flag_succeeds() {
    let output = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .arg("--help")
        .output()
        .expect("failed to run nomark --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nomark"));
    assert!(stdout.contains("Markdown"));
}

#[test]
fn stdin_empty_produces_empty_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn nomark");

    let result = output.wait_with_output().expect("failed to run nomark");
    assert!(result.status.success());
    assert!(result.stdout.is_empty());
}

#[test]
fn stdin_converts_basic_text() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn nomark");

    use std::io::Write;
    child
        .stdin
        .take()
        .expect("failed to take stdin")
        .write_all(b"# Hello\n\n**bold** text")
        .expect("failed to write stdin");

    let result = child.wait_with_output().expect("failed to run nomark");
    assert!(result.status.success());
}

#[test]
fn single_file_to_stdout() -> std::io::Result<()> {
    use std::io::Write;

    let dir = tempfile::tempdir()?;
    let md_path = dir.path().join("test.md");
    let mut f = std::fs::File::create(&md_path)?;
    write!(f, "# Title")?;
    drop(f);

    let output = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .arg(md_path.to_str().unwrap())
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, "* Title");
    Ok(())
}

#[test]
fn single_file_to_output() -> std::io::Result<()> {
    use std::io::Write;

    let dir = tempfile::tempdir()?;
    let md_path = dir.path().join("test.md");
    let norg_path = dir.path().join("test.norg");
    let mut f = std::fs::File::create(&md_path)?;
    write!(f, "# Title")?;
    drop(f);

    let output = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .arg(md_path.to_str().unwrap())
        .arg("-o")
        .arg(norg_path.to_str().unwrap())
        .output()?;
    assert!(output.status.success());
    let content = std::fs::read_to_string(&norg_path)?;
    assert_eq!(content, "* Title");
    Ok(())
}

#[test]
fn batch_to_directory() -> std::io::Result<()> {
    use std::io::Write;

    let dir = tempfile::tempdir()?;
    let md1 = dir.path().join("a.md");
    let md2 = dir.path().join("b.md");
    let out_dir = dir.path().join("out");
    std::fs::create_dir(&out_dir)?;
    write!(std::fs::File::create(&md1)?, "# A")?;
    write!(std::fs::File::create(&md2)?, "# B")?;

    let output = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .arg(md1.to_str().unwrap())
        .arg(md2.to_str().unwrap())
        .arg("-d")
        .arg(out_dir.to_str().unwrap())
        .output()?;
    assert!(output.status.success());

    assert_eq!(std::fs::read_to_string(out_dir.join("a.norg"))?, "* A");
    assert_eq!(std::fs::read_to_string(out_dir.join("b.norg"))?, "* B");
    Ok(())
}

#[test]
fn overwrite_in_place() -> std::io::Result<()> {
    use std::io::Write;

    let dir = tempfile::tempdir()?;
    let md_path = dir.path().join("test.md");
    write!(std::fs::File::create(&md_path)?, "# Title")?;

    let output = Command::new(env!("CARGO_BIN_EXE_nomark"))
        .arg("-w")
        .arg(md_path.to_str().unwrap())
        .output()?;
    assert!(output.status.success());

    let norg_path = dir.path().join("test.norg");
    assert!(norg_path.exists());
    assert_eq!(std::fs::read_to_string(norg_path)?, "* Title");
    Ok(())
}
