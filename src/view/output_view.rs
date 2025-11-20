use std::collections::HashMap;

use crate::domain::{Lotto, LottoRank};

pub struct OutputView;

impl OutputView {
    pub fn show_purchase_count(purchase_count: usize) {
        println!("\n{purchase_count}개를 구매했습니다.");
    }

    pub fn show_purchased_lottos(lottos: &Vec<Lotto>) {
        for lotto in lottos.iter() {
            let mut display_lotto = lotto.numbers().to_vec();
            display_lotto.sort();
            println!("{:?}", display_lotto);
        }
    }

    pub fn show_winning_statistics(rank_counter: &HashMap<LottoRank, i32>) {
        println!("\n당첨 통계\n---");
        println!("3개 일치 (5,000원) - {}개", rank_counter.get(&LottoRank::FIFTH).unwrap());
        println!("4개 일치 (50,000원) - {}개", rank_counter.get(&LottoRank::FOURTH).unwrap());
        println!("5개 일치 (1,500,000원) - {}개", rank_counter.get(&LottoRank::THIRD).unwrap());
        println!("5개 일치, 보너스 볼 일치 (30,000,000원) - {}개", rank_counter.get(&LottoRank::SECOND).unwrap());
        println!("6개 일치 (2,000,000,000원) - {}개", rank_counter.get(&LottoRank::FIRST).unwrap());
    }

    pub fn show_profit_rate(profit_rate: f64) {
        println!("총 수익률은 {:.1}%입니다.", profit_rate);
    }
}
