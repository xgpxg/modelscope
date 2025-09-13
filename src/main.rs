use clap::Parser;
use modelscope::ModelScope;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// model id
    #[arg(short, long)]
    model_id: String,
    /// save dir, will auto create if not exists
    #[arg(short, long)]
    save_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ModelScope::download(&args.model_id, &args.save_dir).await?;

    Ok(())
}
