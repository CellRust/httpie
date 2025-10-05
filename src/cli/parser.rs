use super::types::KvPair;
use anyhow::Result;
use reqwest::Url;

pub fn parse_url(s: &str) -> Result<String> {
    // 这里我们通过 reqwest 提供的 Url struct 来检验一下 url 参数是否合法
    // parse 方法是实现了 FromStr Trait 的类型提供的方法，可以将数据解析为想要的结构
    let _url: Url = s.parse()?;
    // 通过 into 方法进行类型转换，符合函数签名的类型要求
    Ok(s.into())
}

pub fn parse_kv_pair(s: &str) -> Result<KvPair> {
    // 为 KvPair Struct 实现了 FromStr Trait，
    // 可以直接为 s 调用 parse 方法解析为 KvPair
    Ok(s.parse()?)
}
