use std::io;

pub struct InputView;

impl InputView {
    pub fn read_purchase_amount() -> String {
        println!("\n구입금액을 입력해 주세요.");
        let mut purchase_amount_input = String::new();

        io::stdin()
            .read_line(&mut purchase_amount_input)
            .expect("구입금액 읽기에 실패했습니다.");

        purchase_amount_input
    }

    pub fn read_winning_numbers() -> String {
        println!("\n당첨 번호를 입력해 주세요.");
        let mut winning_numbers_input = String::new();

        io::stdin()
            .read_line(&mut winning_numbers_input)
            .expect("당첨 번호 읽기에 실패했습니다.");

        winning_numbers_input
    }

    pub fn read_bonus_number() -> String {
        println!("\n보너스 번호를 입력해 주세요.");
        let mut bonus_number_input = String::new();

        io::stdin()
            .read_line(&mut bonus_number_input)
            .expect("보너스 번호 읽기에 실패했습니다.");

        bonus_number_input
    }
}
