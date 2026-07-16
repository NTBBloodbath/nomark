fn main() -> anyhow::Result<()> {
    use clap::Parser;
    use std::path::Path;

    let args = nomark::cli::Args::parse();

    if args.input.is_empty() {
        let input = read_stdin()?;
        let output = nomark::convert(&input)?;
        print!("{output}");
        return Ok(());
    }

    if let Some(out_file) = &args.output {
        if args.input.len() != 1 {
            anyhow::bail!("-o requires exactly one input file");
        }
        let input = std::fs::read_to_string(&args.input[0])?;
        let output = nomark::convert(&input)?;
        std::fs::write(out_file, &output)?;
        return Ok(());
    }

    let mut files: Vec<std::path::PathBuf> = Vec::new();

    for path in &args.input {
        let p = Path::new(path);
        if p.is_dir() && args.recursive {
            for entry in walkdir::WalkDir::new(p).into_iter().filter_entry(|e| {
                !e.file_name().to_str().map_or(false, |s| s.starts_with('.'))
            }) {
                let entry = entry?;
                if entry.file_type().is_file()
                    && entry.path().extension().map_or(false, |ext| ext == "md")
                {
                    files.push(entry.path().to_path_buf());
                }
            }
        } else if p.is_file() {
            files.push(p.to_path_buf());
        } else if p.is_dir() {
            anyhow::bail!("{} is a directory (use -r to recurse)", path);
        } else {
            anyhow::bail!("{} not found", path);
        }
    }

    if files.is_empty() {
        anyhow::bail!("no input files found");
    }

    if args.overwrite {
        for file in &files {
            let input = std::fs::read_to_string(file)?;
            let output = nomark::convert(&input)?;
            let norg_path = file.with_extension("norg");
            std::fs::write(&norg_path, &output)?;
        }
        return Ok(());
    }

    if let Some(dir) = &args.dir {
        let out_dir = Path::new(dir);
        std::fs::create_dir_all(out_dir)?;
        for file in &files {
            let input = std::fs::read_to_string(file)?;
            let output = nomark::convert(&input)?;
            let norg_name = file.file_stem().unwrap().to_str().unwrap();
            let out_path = out_dir.join(format!("{}.norg", norg_name));
            std::fs::write(out_path, &output)?;
        }
        return Ok(());
    }

    for file in &files {
        let input = std::fs::read_to_string(file)?;
        let output = nomark::convert(&input)?;
        print!("{output}");
    }
    Ok(())
}

fn read_stdin() -> anyhow::Result<String> {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
