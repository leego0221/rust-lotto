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
        let purchase_amount = Self::read_purchase_amount();

        let lottos = lotto_service::purchase(&purchase_amount);
        output_view::show_purchase_count(lottos.len());
        output_view::show_purchased_lottos(&lottos);

        let winning_numbers = Self::read_winning_numbers();
        let bonus_number = Self::read_bonus_number(&winning_numbers);

        let mut lotto_rank_service = LottoRankService::new();
        lotto_rank_service.determine_ranks(&lottos, &winning_numbers, &bonus_number);

        let rank_counter = lotto_rank_service.get_rank();
        let profit_rate = lotto_rank_service.calculate_profit_rate(&purchase_amount);
        output_view::show_winning_statistics(&rank_counter);
        output_view::show_profit_rate(profit_rate);
    }

    fn read_purchase_amount() -> PurchaseAmount {
        loop {
            let input_value = input_view::read_purchase_amount();
            
            let parsed_value = match input_parser::parse_unsigned_integer(&input_value) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            };

            match PurchaseAmount::new(parsed_value) {
                Ok(v) => break v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            }
        }
    }

    fn read_winning_numbers() -> WinningNumbers {
        loop {
            let input_value = input_view::read_winning_numbers();

            let parsed_value = match input_parser::parse_winning_number(&input_value) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            };

            match WinningNumbers::new(parsed_value) {
                Ok(v) => break v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            }
        }
    }

    fn read_bonus_number(winning_numbers: &WinningNumbers) -> BonusNumber {
        loop {
            let input = input_view::read_bonus_number();

            let value = match input_parser::parse_unsigned_integer(&input) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                }
            };

            let bonus_number = match BonusNumber::new(value) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                }
            };

            match lotto_service::check_duplicate(winning_numbers, &bonus_number) {
                Ok(()) => break bonus_number,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                }
            }
        }
    }
}