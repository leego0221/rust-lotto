use std::io;

fn main() {
    println!("구입금액을 입력해 주세요.");

    let mut purchase_amount = String::new();

    io::stdin()
        .read_line(&mut purchase_amount)
        .expect("구입금액 읽기에 실패했습니다.");

    println!("당첨 번호를 입력해 주세요.");

    let mut winning_numbers = String::new();

    io::stdin()
        .read_line(&mut winning_numbers)
        .expect("당첨 번호 읽기에 실패했습니다.");

    println!("보너스 번호를 입력해 주세요.");

    let mut bonus_number = String::new();

    io::stdin()
        .read_line(&mut bonus_number)
        .expect("보너스 번호 읽기에 실패했습니다");

    println!("[DEBUG] 구입금액: {purchase_amount}");
    println!("[DEBUG] 당첨 번호: {winning_numbers}");
    println!("[DEBUG] 보너스 번호: {bonus_number}");
}
