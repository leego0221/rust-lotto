mod view;
mod util;
mod domain;

use std::io;
use view::input_view;
use util::input_parser;
use domain::purchase_amount::PurchaseAmount;
use domain::winning_numbers::WinningNumbers;

fn main() {
    // 구입금액 입력
    let purchase_amount_input = input_view::read_purchase_amount();
    let purchase_amount_value = input_parser::parse_unsigned_integer(purchase_amount_input);
    let purchase_amount = PurchaseAmount::new(purchase_amount_value);

    // 당첨 번호 입력
    let winning_numbers_input = input_view::read_winning_numbers();
    let winning_numbers_value = input_parser::parse_winning_number(&winning_numbers_input);
    let winning_numbers = WinningNumbers::new(winning_numbers_value);

    println!("{winning_numbers_input}");

    // 보너스 번호 입력
    println!("보너스 번호를 입력해 주세요.");
    let mut bonus_number_input = String::new();

    io::stdin()
        .read_line(&mut bonus_number_input)
        .expect("보너스 번호 읽기에 실패했습니다");

    let bonus_number: u32 = bonus_number_input
        .trim()
        .parse()
        .expect("보너스 번호를 u32로 변환하는 데에 실패했습니다.");

    
    // 디버깅 출력
    println!("[DEBUG] 구입금액: {:?}", purchase_amount.get_money());
    for (index, number) in winning_numbers.get_numbers().iter().enumerate() {
        println!("[DEBUG] 당첨 번호 {}: {}", index + 1, number);
    }
    println!("[DEBUG] 보너스 번호: {bonus_number}");
}
