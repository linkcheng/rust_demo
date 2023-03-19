fn main() {
    let numbers1 = &[1, 2, 3, 4, 5];
    let numbers2 = &[u32::MAX, 1];

    println!("numbers1={:?}", numbers1);
    let result =sum_u32(numbers1);
    println!("numbers1 sum ={:?}", result);

    assert_eq!(result, Some(15));
    println!();
    println!("numbers2={:?}", numbers2);
    let result =sum_u32(numbers2);
    println!("numbers2 sum ={:?}", result);
    assert_eq!(result, None);
}

fn sum_u32(arr : &[u32]) -> Option<u32> {
    // let mut result = 0u64;
    // for &i in arr {
    //     result += i as u64;
    // }
    // if result > u32::MAX.into() {
    //     return None;
    // }
    // Some(result as u32)

    // from chatgpt
    let mut result: u32 = 0;

    for &num in arr {
        match result.checked_add(num) {
            Some(i) => result = i,
            None => return None,
        }
    }

    Some(result)
}