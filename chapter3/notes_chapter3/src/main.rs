fn main() {
    test_func1();
    test_func2();
}

fn test_func1() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
}

fn test_func2() {
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);
}

fn test_func3() {
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

fn test_func4() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
}

fn test_func5() {
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
}

fn test_func6() {
    // 회문!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    // 딱히 회문은 아님
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
}

fn test_func7() {
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);
}

fn test_func8() {
    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
}

fn test_func9() {
    let mut v = vec![10, 20, 30, 40, 50];
    assert_eq!(v.pop(), Some(50));
    assert_eq!(v.pop(), Some(40));
    assert_eq!(v.pop(), Some(30));
    assert_eq!(v.pop(), Some(20));
    assert_eq!(v.pop(), Some(10));
    assert_eq!(v.pop(), None);
}

fn test_func10() {
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";
}

fn test_func11() {
    let error_message = "too many pets".to_string();
    assert_eq!(
        format!("{}: {}", "error", error_message),
        "error: too many pets"
    );

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!("    clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}
