use std::{
    fs,
    fs::File,
    io::{self, Write},
};

pub fn create_local_env() {
    let filename = "./env.local";
    let secret_key = input_std("シークレットキーを入力してください");
    let database_name = input_std("データベースの名前を入力してください");
    let mut file = File::create(filename.trim()).expect("ファイル作成エラー");
    let write_buf = format!("{}\n{}", secret_key, database_name);
    file.write_all(write_buf.as_bytes())
        .expect("ファイル書き込みエラー");
}

fn input_std(print_sentence: &str) -> String {
    println!("{}", print_sentence);
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("文字入力エラー");
    sentence.trim().to_string()
}

pub fn read_local_env() -> Vec<String> {
    let filename = "./env.local";
    let sentence = fs::read_to_string(filename).expect("ファイル読み込みエラー");
    let buf = sentence.split('\n').fold(Vec::new(), |mut s, i| {
        s.push(i.to_string());
        s
    });
    buf
}
