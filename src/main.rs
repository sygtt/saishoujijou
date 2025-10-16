use std::io::{self, Write};

fn main() {
    // データの個数の入力
    let mut n: usize;

    loop {
        // データの個数を標準入力で受け取る
        print!("データの個数は? ");
        io::stdout().flush().unwrap();
        let mut input = String::new(); // 標準入力から受け取る文字列
        io::stdin().read_line(&mut input).expect("入力エラー");

        // 受け取った値を符号なし整数に変換し、nに代入
        n = match input.trim().parse::<usize>() {
            Ok(val) => val,
            Err(_) => {
                println!("2以上の整数を入力してください。");
                continue;
            }
        };
        
        // nが2以上か判定、2以上ならデータの個数がそれでいいか確認
        if n < 2 {
            println!("2以上の整数を入力してください。");
            continue;
        } else {
            println!("データ数は{}でいいですか?", n);
            println!("はい:Y, いいえ:n");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).expect("入力エラー");
            // bufの改行文字を削除し小文字に
            let buf = buf.trim().to_lowercase();
            if buf == "n" {
                continue;
            } else {
                break;
            }
        }
    }

    // x_i, y_iの入力
    // xとyはf64型の動的配列
    let mut x = vec![0.0_f64; n];
    let mut y = vec![0.0_f64; n];

    println!("続いて、データの入力を行ってください。");
    println!("間違えて入力した場合、bと入力してください。");

    let mut i = 0;        // カウンタ変数i
    let mut current = "x"; // "x"と"y"のどっちを入力しているか管理
    let mut value;        // 配列に代入する値

    while i < n {
        // メッセージ
        print!("{}[{}] = ? ", current, i);
        io::stdout().flush().unwrap();

        // 入力
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");
        if input.trim() == "b" {
            // 戻る処理
            if current == "x" {
                // x[0]のとき、戻れないので弾く
                if i == 0 {
                    println!("不正な操作です。");
                    continue;
                }
                current = "y";
                i -= 1;
                println!("1つ前の{}[{}]に戻ります。", current, i);
                continue;
            } else {
                current = "x";
                println!("1つ前の{}[{}]に戻ります。", current, i);
                continue;
            }
        } else {
            // 型変換処理
            value = match input.trim().parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    println!("数値を入力してください。");
                    continue;
                }
            };
        }
        
        // 配列への代入・ループ継続の処理
        if current == "x" {
            x[i] = value;
            current = "y";
        } else {
            y[i] = value;
            i += 1;
            current = "x";
        }
    } 
    println!("データの入力が完了しました。");
    println!("x[] = {:?}", x);
    println!("y[] = {:?}", y);
    println!();
    println!("計算を開始するにはEnterを入力してください...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!();
    println!("計算を開始します。");

    // 最小二乗法の計算 (https://manabitimes.jp/math/942)
    // y = ax + b としたときのaとbの値を求める。
    // [xy]はsxyとして表す。
    // sx, sy, sxx, sxyの計算
    let (mut sx, mut sy, mut sxx,mut sxy) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
    for i in 0..n {
        sx += x[i];
        sy += y[i];
        sxx += x[i] * x[i];
        sxy += x[i] * y[i];       
    }

    // sx, sy, sxx, sxyの値を出力
    println!("[x] = {}", sx);
    println!("[y] = {}", sy);
    println!("[xx] = {}", sxx);
    println!("[xy] = {}", sxy);
    
    // a, bの値の計算
    let n = n as f64;
    let a = (n * sxy - sx * sy) / (n * sxx - sx * sx);
    let b = (sxx * sy - sx * sxy) / (n * sxx - sx * sx);

    // 結果の出力
    println!();
    println!("\n-----計算結果------");
    println!("a = {}, b = {}", a, b);

    // 何かキー入力を待つ
    println!();
    println!();
    println!("終了するにはEnterを入力してください...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
