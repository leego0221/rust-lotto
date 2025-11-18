mod view;
mod util;
mod error;
mod service;
mod domain;

use view::input_view;
use view::output_view;
use util::input_parser;
use service::lotto_service;
use service::lotto_rank_service::LottoRankService;
use domain::purchase_amount::PurchaseAmount;
use domain::winning_numbers::WinningNumbers;
use domain::bonus_number::BonusNumber;

fn main() {
    // 구입금액 입력
    let purchase_amount_input = input_view::read_purchase_amount();
    let purchase_amount_value = input_parser::parse_unsigned_integer(&purchase_amount_input);
    let purchase_amount = match purchase_amount_value {
        Ok(value) => match PurchaseAmount::new(value) {
            Ok(value) => value,
            Err(e) => panic!("{}", e.message()),
        },
        Err(e) => panic!("{}", e.message()),
    };

    // 구입 금액에 해당하는 만큼 로또 발행하기
    let lottos = lotto_service::purchase(&purchase_amount);

    // 구매 개수, 구매한 로또 리스트 출력
    output_view::show_purchase_count(lottos.len());
    output_view::show_purchased_lottos(&lottos);

    // 당첨 번호 입력
    let winning_numbers_input = input_view::read_winning_numbers();
    let winning_numbers_value = input_parser::parse_winning_number(&winning_numbers_input);
    let winning_numbers = match winning_numbers_value {
        Ok(value) => match WinningNumbers::new(value) {
            Ok(value) => value,
            Err(e) => panic!("{}", e.message()),
        },
        Err(e) => panic!("{}", e.message()),
    };

    // 보너스 번호 입력
    let bonus_number_input = input_view::read_bonus_number();
    let bonus_number_value = input_parser::parse_unsigned_integer(&bonus_number_input);
    let bonus_number = match bonus_number_value {
        Ok(value) => match BonusNumber::new(value) {
            Ok(value) => value,
            Err(e) => panic!("{}", e.message()),
        },
        Err(e) => panic!("{}", e.message()),
    };

    // 당첨 번호와 보너스 번호 중복 검증
    match lotto_service::check_duplicate(&winning_numbers, &bonus_number) {
        Ok(()) => (),
        Err(e) => panic!("{}", e.message()),
    };

    // 번호 일치 여부에 따라 등수 매긴 뒤 내부에 저장하기
    let mut lotto_rank_service = LottoRankService::new();
    lotto_rank_service.determine_ranks(&lottos, &winning_numbers, &bonus_number);
    let rank_counter = lotto_rank_service.get_rank();

    // 총 구매 금액에 따른 수익률 계산하기
    let profit_rate = lotto_rank_service.calculate_profit_rate(&purchase_amount);

    // 로또 순위 집계표 및 수익률 출력하기
    output_view::show_winning_statistics(&rank_counter);
    output_view::show_profit_rate(profit_rate);
}
