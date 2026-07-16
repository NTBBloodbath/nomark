use clap::Parser;

#[derive(Parser)]
#[command(name = "nomark", version, about = "Convert Markdown documents to Norg format")]
pub struct Args {
    #[arg(help = "Input files or directories (omit for stdin)")]
    pub input: Vec<String>,

    #[arg(short = 'o', long, help = "Output file (single input only)")]
    pub output: Option<String>,

    #[arg(short = 'd', long, help = "Output directory (batch mode)")]
    pub dir: Option<String>,

    #[arg(short = 'r', long, help = "Recurse into directories")]
    pub recursive: bool,

    #[arg(short = 'w', long, help = "Overwrite input files in-place (.md → .norg)")]
    pub overwrite: bool,

    #[arg(long, help = "Force output to stdout (batch mode)")]
    pub stdout: bool,
}
