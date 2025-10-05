use super::super::cli::{Get, Post};
use super::printer;
use anyhow::{Ok, Result};
use reqwest::Client;
use std::collections::HashMap;

// 处理 Get 子命令
pub async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    Ok(printer::print_res(res).await?)
}

// 处理 Post 子命令
pub async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let res = client.post(&args.url).json(&body).send().await?;
    Ok(printer::print_res(res).await?)
}
