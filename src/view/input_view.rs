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

    pub fn read_selection_mode() -> String {
        println!("\n로또 번호는 '전체 자동' 또는 '수동 포함' 두 가지 방식 중 선택이 가능합니다.");
        println!("원하는 방식을 선택해주세요. (a: 전체 자동, m: 수동 포함)");
        let mut selection_mode_input = String::new();

        io::stdin()
            .read_line(&mut selection_mode_input)
            .expect("선택 모드 읽기에 실패했습니다.");

        selection_mode_input
    }

    pub fn read_manual_count() -> String {
        println!("이 중 몇 개를 로또를 수동 선택할지 입력해주세요.");
        let mut manual_count_input = String::new();

        io::stdin()
            .read_line(&mut manual_count_input)
            .expect("수동 선택 개수 읽기에 실패했습니다.");

        manual_count_input
    }

    pub fn read_manual_numbers() -> String {
        println!("\n수동으로 로또 번호를 입력해 주세요.");
        let mut manual_numbers_input = String::new();

        io::stdin()
            .read_line(&mut manual_numbers_input)
            .expect("수동 로또 번호 읽기에 실패했습니다.");

        manual_numbers_input
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
