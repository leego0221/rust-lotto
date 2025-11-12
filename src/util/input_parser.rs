pub fn parse_unsigned_integer(input: String) -> u32 {
    input.trim()
        .parse()
        .expect("입력값을 u32로 변환하는 데에 실패했습니다.")
}