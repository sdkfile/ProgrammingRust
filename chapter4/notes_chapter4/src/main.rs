fn main() {
    println!("Hello, world!");
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn box_example() {
    let point = Box::new((0.625, 0.5)); // 여기서 point가 할당되고,
    let label = format!("{:?}", point); // 여기서 label이 할당되며,
    assert_eq!(LABEL, "(0.625, 0.5)");
} // 여기서 둘다 드롭된다.

fn struct_example() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn move_example() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s; // s의 소유권이 t로 이동한다.
               // let u = s; // s는 이미 t로 이동했으므로, s를 사용할 수 없다. ERROR!
}

fn clone_example() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone(); // s의 복사본이 t로 이동한다.
    let u = s.clone(); // s의 복사본이 u로 이동한다.
}

fn move_example2() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // 여기서 값 "Govinda"는 드롭된다.
}

fn move_example3() {
    let mut s = "Govinda".to_string();
    let t = s; // "Govinda"의 소유권이 t로 이동한다.
    s = "Siddhartha".to_string(); // 아무것도 드롭되지 않는다.
}

fn move_from_vector1() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // 벡터에서 아무 요소나 꺼낸다
    // let third = v[2]; // 오류: 벡터 밖으로 꺼낼 수 없다.
    // let fifth = v[4]; // 오류: 벡터 밖으로 꺼낼 수 없다.
}

fn move_from_vector2() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // 1. 벡터 끝에 있는 값을 꺼낸다.
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. 주어진 색인에 있는 값을 벡터 밖으로 옮기고, 마지막 요소를 그 자리로 옮긴다.
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. 꺼내려는 값을 다른 값과 맞바꾼다.
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // 4. 벡터에 뭐가 남았는지 보자
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

fn vector_in_loop() {
    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

fn struct_move() {
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    // 아래와 같이 하면 안됨
    // let first_name = composers[0].name;

    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);

    //std::mem::replace로 None을 넣는거 보다는 take를 쓰는게 더 좋음
    let first_name = composers[0].name.take();
}

fn copy_example() {
    // 여기서는 값이 이동된다.
    let string1 = "somnambulance".to_string();
    let string2 = string1;

    // 여기서는 값이 복사된다. i32와 같은 원시 타입들은 Copy 타입이기 때문
    let num1: i32 = 36;
    let num2 = num1;
}

fn rc_example1() {
    use std::rc::Rc;

    // 러스트는 여기 있는 타입들도 전부 대신 추론해주지만, 여기서는 뭐가 뭔지 알기 쉽도록 직접 적었다.
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    // Rc는 mutable하지 않다.
    // s.push_str(" noodles"); // 오류: `Rc<String>`은 `String`처럼 mutable하지 않다.

    assert!(s.contains("shira"));
    assert_eq!(t.find("tak"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
