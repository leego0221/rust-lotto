use crate::domain::lotto::Lotto;

pub fn show_purchase_count(purchase_count: u32) {
    println!("\n{purchase_count}개를 구매했습니다.");
}

pub fn show_purchased_lottos(lottos: &Vec<Lotto>) {
    for lotto in lottos.iter() {
        let mut sorted_lotto = lotto.get_numbers();
        sorted_lotto.sort();
        println!("{:?}", sorted_lotto);
    }
}