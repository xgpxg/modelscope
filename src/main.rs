use clap::Parser;
use modelscope::{DownloadOptions, ModelScope};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: SubCommand,
}

impl Args {
    fn default_save_dir() -> PathBuf {
        let path = env::home_dir().expect("Failed to get home directory");
        path.join(".modelscope").join("models")
    }
}

#[derive(Debug, Clone, Parser)]
enum SubCommand {
    /// Download model
    Download {
        /// Model ID
        #[arg(short, long)]
        model_id: String,
        /// The path to save the model, will be created if not exists
        #[arg(short, long, default_value_os_t = Args::default_save_dir())]
        save_dir: PathBuf,
        /// Specify files to download
        #[arg(long, value_delimiter = ',')]
        files: Option<Vec<String>>,
    },
    /// Login to modelscope use your token
    Login {
        /// modelscope token
        #[arg(short, long)]
        token: String,
    },
    /// Logout
    Logout,
    /// List all local models
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        SubCommand::Download {
            model_id,
            save_dir,
            files,
        } => {
            ModelScope::download_with_options(&model_id, &save_dir, DownloadOptions { files })
                .await?;
        }
        SubCommand::Login { token } => {
            ModelScope::login(&token).await?;
        }
        SubCommand::Logout => {
            ModelScope::logout().await?;
        }
        SubCommand::List => {
            let models = ModelScope::list().await?;
            if models.is_empty() {
                println!();
                println!("No local models found.");
                println!();
            } else {
                println!();
                println!("Found {} local Models", models.len());
                println!();
                for (index, model) in models.iter().enumerate() {
                    println!("{:2}. {:<50} {}", index + 1, model.0, model.1);
                }
                println!();
            }
        }
    };

    Ok(())
}
