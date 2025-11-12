pub fn parse_unsigned_integer(input: &str) -> u32 {
    input.trim()
        .parse()
        .expect("입력값을 u32로 변환하는 데에 실패했습니다.")
}

pub fn parse_winning_number(input: &str) -> Vec<u32> {
    input.split(",")
        .map(|number| parse_unsigned_integer(number))
        .collect()
}