use anyhow::bail;
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;

const FILES_URL: &str = "https://modelscope.cn/api/v1/models/<model_id>/repo/files?Recursive=true";
const DOWNLOAD_URL: &str = "https://modelscope.cn/models/<model_id>/resolve/master/<path>";

const UA: (&str, &str) = (
    "User-Agent",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36",
);
pub struct ModelScope;

#[derive(Debug, Deserialize)]
struct ModelScopeResponse {
    #[serde(rename = "Code")]
    code: i32,
    #[serde(rename = "Data")]
    data: ModelScopeResponseData,
}

#[derive(Debug, Deserialize)]
struct ModelScopeResponseData {
    #[serde(rename = "Files")]
    files: Vec<RepoFile>,
}
#[derive(Debug, Deserialize)]
struct RepoFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Size")]
    size: u64,
}

const BAR_STYLE: &str = "{msg:<30} {bar} {decimal_bytes:<10} / {decimal_total_bytes:<10} {decimal_bytes_per_sec:<10} {percent:<3}% {eta_precise}";

impl ModelScope {
    pub async fn download(model_id: &str, save_dir: &PathBuf) -> anyhow::Result<()> {
        println!("downloading model {} to: {}", model_id, save_dir.display());

        fs::create_dir_all(save_dir)?;

        let files_url = FILES_URL.replace("<model_id>", model_id);
        let resp = reqwest::get(files_url).await?;
        let response = resp.json::<ModelScopeResponse>().await?;
        if response.code != 200 {
            bail!("Failed to get model files: {}", response.code);
        }
        let data = response.data;
        let repo_files = data.files;

        let client = reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(10));
        let client = Arc::new(client.build()?);

        let mut tasks = Vec::new();
        let bars = MultiProgress::new();

        for repo_file in repo_files.into_iter() {
            let model_id = model_id.to_string();
            let client = client.clone();
            let save_dir = save_dir.clone();

            let bar = ProgressBar::new(repo_file.size);
            let style = ProgressStyle::default_bar().template(BAR_STYLE)?;
            bar.set_style(style);

            bars.add(bar.clone());

            let task = tokio::spawn(async move {
                let res = Self::download_file(client, model_id, repo_file, save_dir, bar).await;
                if let Err(e) = res {
                    println!("Error downloading file: {}", e);
                    exit(1);
                }
            });

            tasks.push(task);
        }
        for task in tasks {
            task.await?;
        }

        Ok(())
    }

    async fn download_file(
        client: Arc<reqwest::Client>,
        model_id: String,
        repo_file: RepoFile,
        save_dir: PathBuf,
        bar: ProgressBar,
    ) -> anyhow::Result<()> {
        let path = &repo_file.path;
        let name = &repo_file.name;

        let file_path = save_dir.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = BufWriter::new(File::create(&file_path)?);

        let url = DOWNLOAD_URL
            .replace("<model_id>", &model_id)
            .replace("<path>", path);

        let response = client.get(url).header(UA.0, UA.1).send().await?;
        if !response.status().is_success() {
            bail!(
                "Failed to download file {}: HTTP {}",
                name,
                response.status()
            );
        }

        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;

            bar.set_message(name.clone());
            bar.inc(chunk.len() as u64);
        }
        file.flush()?;
        bar.finish();
        Ok(())
    }
}
