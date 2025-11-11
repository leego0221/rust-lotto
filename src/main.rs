use std::io;

fn main() {
    // 구입금액 입력
    println!("구입금액을 입력해 주세요.");
    let mut purchase_amount_input = String::new();

    io::stdin()
        .read_line(&mut purchase_amount_input)
        .expect("구입금액 읽기에 실패했습니다.");

    let purchase_amount: u32 = purchase_amount_input
        .trim()
        .parse()
        .expect("구입금액을 u32로 변환하는 데에 실패했습니다.");


    // 당첨 번호 입력
    println!("당첨 번호를 입력해 주세요.");
    let mut winning_numbers_input = String::new();

    io::stdin()
        .read_line(&mut winning_numbers_input)
        .expect("당첨 번호 읽기에 실패했습니다.");

    let winning_numbers_value: Vec<&str> = winning_numbers_input
        .split(",")
        .collect();

    let winning_numbers: Vec<u32> = winning_numbers_value
        .iter()
        .map(|number| number.trim().parse().expect("당첨 번호를 u32로 변환하는 데에 실패했습니다."))
        .collect();


    // 보너스 번호 입력
    println!("보너스 번호를 입력해 주세요.");
    let mut bonus_number_input = String::new();

    io::stdin()
        .read_line(&mut bonus_number_input)
        .expect("보너스 번호 읽기에 실패했습니다");

    let bonus_number: u32 = bonus_number_input
        .trim()
        .parse()
        .expect("보너스 번호를 u32로 변환하는 데에 실패했습니다.");

    
    // 디버깅 출력
    println!("[DEBUG] 구입금액: {purchase_amount}");
    for (index, number) in winning_numbers.iter().enumerate() {
        println!("[DEBUG] 당첨 번호 {}: {}", index + 1, number);
    }
    println!("[DEBUG] 보너스 번호: {bonus_number}");
}
