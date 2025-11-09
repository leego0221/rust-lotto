use std::io;

fn main() {
    println!("구입금액을 입력해 주세요.");

    let mut purchase_amount = String::new();

    io::stdin()
        .read_line(&mut purchase_amount)
        .expect("구입금액 읽기에 실패했습니다.");

    println!("[DEBUG] 구입금액: {purchase_amount}");
}
