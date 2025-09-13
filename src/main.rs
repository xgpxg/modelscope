use clap::Parser;
use modelscope::ModelScope;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// model id
    #[arg(short, long)]
    model_id: String,
    /// save dir, if not set, use current dir, will create if not exists
    #[arg(short, long, default_value = "")]
    save_dir: PathBuf,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ModelScope::download(&args.model_id, &args.save_dir)
    .await?;

    Ok(())
}
