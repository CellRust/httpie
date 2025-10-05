mod cli;
mod utils;

use cli::{HttpMethod, Httpie};
use utils::http::{get, post};

use anyhow::{Ok, Result};
use clap::Parser;
use reqwest::Client;

// 使用 tokio::main 属性宏自动添加处理异步操作的运行时
#[tokio::main]
async fn main() -> Result<()> {
    let args = Httpie::parse();

    // 生成一个 Http 客户端
    let client = Client::new();

    let result = match args.method {
        HttpMethod::Get(args) => get(client, &args).await?,
        HttpMethod::Post(args) => post(client, &args).await?,
    };

    Ok(result)
}
