use crate::view::input_view;
use crate::view::output_view;
use crate::util::input_parser;
use crate::service::lotto_service;
use crate::service::lotto_rank_service::LottoRankService;
use crate::domain::purchase_amount::PurchaseAmount;
use crate::domain::winning_numbers::WinningNumbers;
use crate::domain::bonus_number::BonusNumber;

pub struct LottoController;

impl LottoController {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        // 구입금액 입력
        let purchase_amount = loop {
            let input = input_view::read_purchase_amount();
            let value = input_parser::parse_unsigned_integer(&input);

            match value {
                Ok(value) => match PurchaseAmount::new(value) {
                    Ok(value) => break value,
                    Err(e) => {
                        eprintln!("{} 다시 입력해주세요.", e.message());
                        continue;
                    },
                },
                Err(e) => {
                    eprintln!("{}", e.message());
                    continue;
                },
            };
        };

        // 구입 금액에 해당하는 만큼 로또 발행하기
        let lottos = lotto_service::purchase(&purchase_amount);

        // 구매 개수, 구매한 로또 리스트 출력
        output_view::show_purchase_count(lottos.len());
        output_view::show_purchased_lottos(&lottos);

        // 당첨 번호 입력
        let winning_numbers = loop {
            let input = input_view::read_winning_numbers();
            let value = input_parser::parse_winning_number(&input);

            match value {
                Ok(value) => match WinningNumbers::new(value) {
                    Ok(value) => break value,
                    Err(e) => {
                        eprintln!("{} 다시 입력해주세요.", e.message());
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            };
        };

        // 보너스 번호 입력
        let bonus_number = loop {
            let input = input_view::read_bonus_number();
            let value = input_parser::parse_unsigned_integer(&input);

            let number = match value {
                Ok(value) => match BonusNumber::new(value) {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("{} 다시 입력해주세요.", e.message());
                        continue;
                    },
                },
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                }
            };

            // 당첨 번호와 보너스 번호 중복 검증
            match lotto_service::check_duplicate(&winning_numbers, &number) {
                Ok(()) => break number,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                }
            };
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
}