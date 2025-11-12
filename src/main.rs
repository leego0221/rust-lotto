mod view;
mod util;
mod domain;

use view::input_view;
use view::output_view;
use util::input_parser;
use util::number_generator;
use domain::purchase_amount::PurchaseAmount;
use domain::winning_numbers::WinningNumbers;
use domain::bonus_number::BonusNumber;
use domain::lotto::Lotto;

fn main() {
    // 구입금액 입력
    let purchase_amount_input = input_view::read_purchase_amount();
    let purchase_amount_value = input_parser::parse_unsigned_integer(&purchase_amount_input);
    let purchase_amount = PurchaseAmount::new(purchase_amount_value);

    // 구입 금액에 해당하는 만큼 로또 발행하기
    let mut lottos: Vec<Lotto> = Vec::new();
    let purchase_count = purchase_amount.get_money() / 1000;
    for _ in 0..purchase_count {
        let lotto_numbers = number_generator::generate_numbers_in_range(1, 45, 6);
        let lotto = Lotto::new(lotto_numbers);
        lottos.push(lotto);
    }

    // 구매 개수, 구매한 로또 리스트 출력
    output_view::show_purchase_count(purchase_count);
    output_view::show_purchased_lottos(&lottos);

    // 당첨 번호 입력
    let winning_numbers_input = input_view::read_winning_numbers();
    let winning_numbers_value = input_parser::parse_winning_number(&winning_numbers_input);
    let winning_numbers = WinningNumbers::new(winning_numbers_value);

    // 보너스 번호 입력
    let bonus_number_input = input_view::read_bonus_number();
    let bonus_number_value = input_parser::parse_unsigned_integer(&bonus_number_input);
    let bonus_number = BonusNumber::new(bonus_number_value);

    println!("[DEBUG] 구입금액: {:?}", purchase_amount.get_money());
    for (index, number) in winning_numbers.get_numbers().iter().enumerate() {
        println!("[DEBUG] 당첨 번호 {}: {}", index + 1, number);
    }
    println!("[DEBUG] 보너스 번호: {:?}", bonus_number.get_number());
}
