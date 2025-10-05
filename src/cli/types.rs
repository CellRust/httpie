use super::parser::*;
use anyhow::{Error, Ok, Result};
use clap::{Args, Parser, Subcommand};
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
// 主命令：通过 derive 派生宏派生 Parser Trait 解析命令行内容
pub struct Httpie {
    // 通过 clap 提供的 command 属性宏为被标记的结构声明子命令字段
    // command 宏的作用域：结构体/枚举级别和字段级别，配置命令级别的行为
    #[command(subcommand)]
    pub method: HttpMethod,
}

// 子命令：通过 derive 派生宏派生 Subcommand Trait 声明子命令
// Subcommand Trait 只能用于 enum
#[derive(Subcommand)]
pub enum HttpMethod {
    Get(Get),
    Post(Post),
}

// 参数：通过 derive 派生宏派生 Args Trait 声明参数
/// feed get with an url and we will retrieve the response for you
#[derive(Args)]
pub struct Get {
    // 通过 clap 提供的 arg 属性宏标记某一字段的自定义解析函数，可以做一些合法性校验
    // arg 宏的作用域：用于字段级别，配置单个参数的解析行为
    #[arg(value_parser = parse_url)]
    pub url: String,
}

/// feed post with an url and optional key=value pairs.
/// We will post the data as JSON, and retrieve the response for you
#[derive(Args)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,
    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Clone, Debug)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 将字符串按 '=' 分割，并且最多分为两部分，最后返回一个迭代器
        // 之所以 mut 是因为后续的 next 方法会消耗迭代器
        let mut parts = s.splitn(2, "=");

        // 对一个迭代器调用 next 方法，获取到下一个值
        let k = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid key-value pair: missing key"))?
            .to_string();

        let v = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid key-value pair: missing value"))?
            .to_string();

        // 返回解析后的键值对
        Ok(KvPair { k, v })
    }
}
