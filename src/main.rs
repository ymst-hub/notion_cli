mod file_local;
fn main() {
    //ファイルを開きキー設定を確認する。
    let keys: Vec<String> = match file_local::read_local_env() {
        Ok(val) => val,
        Err(_) => {
            println!("Notionのキー設定がされていません。");
            file_local::create_local_env();
            std::process::exit(1);
        },
    };

    //コマンドを取得し、matchで判定する
    loop {
        let command_print = "コマンドを入力してください\n1 : ボードを全件取得\n2 : タスクのフラグを変更する\n3 : キーの再設定\nその他 : 終了する";
        let command:i64 = match file_local::input_std(command_print).parse(){
            Ok(val) => val,
            Err(_) => std::process::exit(2),
        };
        match command {
            1 => todo!(),//ボードの全件取得
            2 => todo!(),//タスクの移動
            3 => file_local::create_local_env(),//ファイルの再設定
            _ => std::process::exit(3),//終了
        }
    }

}
