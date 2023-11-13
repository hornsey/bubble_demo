fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut result: Option<u32> = Some(0);

    for &num in numbers {
        match result {
            Some(value) => {
                // 使用 checked_add 进行加法运算，检查是否溢出
                result = value.checked_add(num);
            }
            None => {
                // 如果已经溢出，直接返回 None
                return None;
            }
        }
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred!"),
    }

    let numbers = vec![1, 2, 3, 4, 5, u32::MAX];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred!"),
    }

}
