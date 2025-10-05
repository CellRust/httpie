use anyhow::Result;
use colored::Colorize;
use mime::Mime;
use reqwest::{Response, header};

// 打印服务器版本号和状态码
pub fn print_status(res: &Response) {
    let status = format!("{:?} {}", res.version(), res.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
pub fn print_headers(res: &Response) {
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

// 打印服务器返回的 HTTP body
pub fn print_body(mime: Option<Mime>, body: &String) {
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
pub fn get_content_type(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// 打印整个响应
pub async fn print_res(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);
    let mime = get_content_type(&res);
    let body = res.text().await?;
    print_body(mime, &body);
    Ok(())
}
