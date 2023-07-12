fn main() {
    loop_usage()
}

fn for_loop() {
    let strings: Vec<String> = vec![
        "my youth is yours",
        "a truth so loud you can't ignore",
        "my youth, my youth, my youth",
        "my youth is yours",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    for rs in &strings {
        println!("String {:?} is at address {:p}", *rs, rs);
    }
}
fn next_line<'a>() -> Option<&'a str> {
    Some("answer: 42")
}
fn loop_usage() {
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };
    println!("{}", answer);
}

fn loop_usage2() {
    let sqrt = 'outer: loop {
        let n = next_number();
        for i in 1.. {
            let square = i * i;
            if square == n {
                // 제곱근 구함
                break 'outer i;
            }
            if square > n {
                // n은 완전제곱수가 아니므로 다음으로 넘어감
                break;
            }
        }
    };
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot_index = partition(slice);
    quicksort(&mut slice[..pivot_index]);
    quicksort(&mut slice[pivot_index + 1..]);
}

fn closure_test() {
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    assert_eq!(is_even(2), true);
}
