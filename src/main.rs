mod view;
mod util;
mod domain;
mod error;

use view::input_view;
use view::output_view;
use util::input_parser;
use util::number_generator;
use domain::purchase_amount::PurchaseAmount;
use domain::winning_numbers::WinningNumbers;
use domain::bonus_number::BonusNumber;
use domain::lotto::Lotto;
use domain::lotto_rank::LottoRank;

use std::collections::HashMap;

use strum::IntoEnumIterator;

fn main() {
    // 구입금액 입력
    let purchase_amount_input = input_view::read_purchase_amount();
    let purchase_amount_value = input_parser::parse_unsigned_integer(&purchase_amount_input);
    let purchase_amount = match purchase_amount_value {
        Ok(value) => match PurchaseAmount::new(value) {
            Ok(value) => value,
            Err(e) => panic!("{}", e.message()),
        },
        Err(e) => panic!("{}", e.message())
    };

    // 구입 금액에 해당하는 만큼 로또 발행하기
    let mut lottos: Vec<Lotto> = Vec::new();
    let purchase_count = purchase_amount.money() / 1000;
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
        Err(e) => panic!("{}", e.message())
    };

    // 번호 일치 여부에 따라 등수 매긴 뒤 내부에 저장하기
    let mut rank_counter = HashMap::new();
    for rank in LottoRank::iter() {
        rank_counter.insert(rank, 0);
    }

    for lotto in lottos {
        // 당첨 번호 확인
        let lotto_number = lotto.get_numbers();
        let numbers_count = winning_numbers.numbers()
            .iter()
            // 이 부분에서 자동으로 참조자 단계를 맞춰주지만, 명시적으로 확인할 수 있도록 *number 적용
            .filter(|number| lotto_number.contains(*number))
            .count();

        // 보너스 번호 확인
        let bonus_number = bonus_number.number();
        let bonus_matched = lotto_number.contains(&bonus_number);

        // 등수 매핑
        let rank = match (numbers_count, bonus_matched) {
            (6, _) => LottoRank::FIRST,
            (5, true) => LottoRank::SECOND,
            (5, false) => LottoRank::THIRD,
            (4, _) => LottoRank::FOURTH,
            (3, _) => LottoRank::FIFTH,
            _ => LottoRank::NOTHING,
        };

        // 로또 순위 집계표 갱신
        let count = rank_counter
            .get(&rank)
            .copied()
            .unwrap_or(0);
        rank_counter.insert(rank, count + 1);
    }

    // 총 구매 금액에 따른 수익률 계산하기
    let mut total_prize: u64 = 0;
    for (rank, count) in rank_counter.iter() {
        let prize = rank.get_prize() as u64;
        let count = *count as u64;
        total_prize += prize * count;
    }

    let total_purchase = purchase_amount.money() as u64;
    let profit_rate = total_prize as f64 / total_purchase as f64 * 100.0;

    // 로또 순위 집계표 및 수익률 출력하기
    output_view::show_winning_statistics(&rank_counter);
    output_view::show_profit_rate(profit_rate);
}
