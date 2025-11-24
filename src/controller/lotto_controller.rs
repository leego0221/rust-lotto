use crate::domain::{BonusNumber, Lotto, ManualCount, PurchaseAmount, SelectionMode, WinningNumbers};
use crate::service::{LottoRankService, LottoService};
use crate::util::{InputParser, NumberGenerator};
use crate::view::{InputView, OutputView};

pub struct LottoController;

impl LottoController {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        OutputView::show_main_title();
        let purchase_amount = Self::read_purchase_amount();
        let selection_mode = Self::read_selection_mode();

        let pending_purchase_count = purchase_amount.money() / 1000 as u32;
        let manual_count = match selection_mode {
            SelectionMode::Auto => 0,
            SelectionMode::Manual => Self::read_manual_count(pending_purchase_count).count(),
        };
        let auto_count = pending_purchase_count - manual_count;

        let mut lottos = Vec::new();
        for i in 1..=manual_count {
            let lotto = Self::generate_manual_lotto(i);
            lottos.push(lotto);
        }
        for _ in 1..=auto_count {
            let lotto = Self::generate_auto_lotto();
            lottos.push(lotto);
        }
        
        OutputView::show_purchase_count(lottos.len());
        OutputView::show_purchased_lottos(&lottos, manual_count);

        let winning_numbers = Self::read_winning_numbers();
        let bonus_number = Self::read_bonus_number(&winning_numbers);

        let mut lotto_rank_service = LottoRankService::new();
        lotto_rank_service.determine_ranks(&lottos, &winning_numbers, &bonus_number);

        let rank_counter = lotto_rank_service.get_rank();
        let profit_rate = lotto_rank_service.calculate_profit_rate(&purchase_amount);
        OutputView::show_winning_statistics(&rank_counter);
        OutputView::show_profit_rate(profit_rate);
    }

    fn read_purchase_amount() -> PurchaseAmount {
        loop {
            let input_value = InputView::read_purchase_amount();
            
            let parsed_value = match InputParser::parse_unsigned_integer(&input_value) {
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

    fn read_selection_mode() -> SelectionMode {
        loop {
            let input_value = InputView::read_selection_mode();

            let parsed_value = match InputParser::parse_character(&input_value) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            };

            match SelectionMode::from(parsed_value) {
                Ok(v) => break v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            }
        }
    }

    fn read_manual_count(pending_purchase_count: u32) -> ManualCount {
        loop {
            let input_value = InputView::read_manual_count(pending_purchase_count);

            let parsed_value = match InputParser::parse_unsigned_integer(&input_value) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            };

            match ManualCount::new(parsed_value, pending_purchase_count) {
                Ok(v) => break v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            }
        }
    }

    fn generate_manual_lotto(index: u32) -> Lotto {
        loop {
            let manual_numbers = Self::read_manual_numbers(index);

            match LottoService::purchase(manual_numbers) {
                Ok(v) => break v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            }
        }
    }

    fn read_manual_numbers(index: u32) -> Vec<u32> {
        loop {
            let input_value = InputView::read_manual_numbers(index);

            match InputParser::parse_numbers(&input_value) {
                Ok(v) => break v,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                },
            }
        }
    }

    fn generate_auto_lotto() -> Lotto {
        loop {
            let auto_numbers = NumberGenerator::generate_numbers_in_range(1, 45, 6);

            match LottoService::purchase(auto_numbers) {
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
            let input_value = InputView::read_winning_numbers();

            let parsed_value = match InputParser::parse_numbers(&input_value) {
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
            let input = InputView::read_bonus_number();

            let value = match InputParser::parse_unsigned_integer(&input) {
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

            match LottoService::check_duplicate(winning_numbers, &bonus_number) {
                Ok(()) => break bonus_number,
                Err(e) => {
                    eprintln!("{} 다시 입력해주세요.", e.message());
                    continue;
                }
            }
        }
    }
}
