use anyhow::{Error, Ok, Result};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, Response, Url, header};
use std::{collections::HashMap, str::FromStr};

#[derive(Parser)]
#[command(version, about, long_about = None)]
// 主命令：通过 derive 派生宏派生 Parser Trait 解析命令行内容
struct Httpie {
    // 通过 clap 提供的 command 属性宏为被标记的结构声明子命令字段
    // command 宏的作用域：结构体/枚举级别和字段级别，配置命令级别的行为
    #[command(subcommand)]
    method: HttpMethod,
}

// 子命令：通过 derive 派生宏派生 Subcommand Trait 声明子命令
// Subcommand Trait 只能用于 enum
#[derive(Subcommand)]
enum HttpMethod {
    Get(Get),
    Post(Post),
}

// 参数：通过 derive 派生宏派生 Args Trait 声明参数
/// feed get with an url and we will retrieve the response for you
#[derive(Args)]
struct Get {
    // 通过 clap 提供的 arg 属性宏标记某一字段的自定义解析函数，可以做一些合法性校验
    // arg 宏的作用域：用于字段级别，配置单个参数的解析行为
    #[arg(value_parser = parse_url)]
    url: String,
}

/// feed post with an url and optional key=value pairs.
/// We will post the data as JSON, and retrieve the response for you
#[derive(Args)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Clone, Debug)]
struct KvPair {
    k: String,
    v: String,
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

fn parse_url(s: &str) -> Result<String> {
    // 这里我们通过 reqwest 提供的 Url struct 来检验一下 url 参数是否合法
    // parse 方法是实现了 FromStr Trait 的类型提供的方法，可以将数据解析为想要的结构
    let _url: Url = s.parse()?;
    // 通过 into 方法进行类型转换，符合函数签名的类型要求
    Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    // 为 KvPair Struct 实现了 FromStr Trait，
    // 可以直接为 s 调用 parse 方法解析为 KvPair
    Ok(s.parse()?)
}

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

// 处理 Get 子命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    Ok(print_res(res).await?)
}

// 处理 Post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let res = client.post(&args.url).json(&body).send().await?;
    Ok(print_res(res).await?)
}

// 打印服务器版本号和状态码
fn print_status(res: &Response) {
    let status = format!("{:?} {}", res.version(), res.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(res: &Response) {
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

// 打印服务器返回的 HTTP body
fn print_body(mime: Option<Mime>, body: &String) {
    match mime {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().yellow())
        }
        // 其它 Mime Type，我们就直接输出
        _ => println!("{}", body),
    }
}

// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// 打印整个响应
async fn print_res(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);
    let mime = get_content_type(&res);
    let body = res.text().await?;
    print_body(mime, &body);
    Ok(())
}
