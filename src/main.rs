use std::{fs, env};

fn main() {
    const PATH: &str = "/data/webapps/appenv";
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(),2);
    let env = &args[1];
    let content = fs::read_to_string(PATH).expect("无法读取appenv");
    let content = content.replace("\r\n", "\n");
    let out = modify(env, content);

    fs::write(PATH, &out).expect("写入appenv失败");
    println!("# 当前appenv为\n{}", out);
}

fn modify(env: &String, content: String) -> String {
    let mut out = String::new();
    for line in content.split("\n") {
        if line.len() != 0 {
            if line.starts_with("env=") {
                out.push_str("env=");
                out.push_str(env);
            } else {
                out.push_str(line);
            }
            out.push_str("\n");
        }
    }
    out.push_str("\n");
    out
}
