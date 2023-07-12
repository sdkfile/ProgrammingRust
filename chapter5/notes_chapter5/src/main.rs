use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    sort_works(&mut table);
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}

// shared reference, mutable reference => multiple readers or single writer
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}

fn reference_example1() {
    let x = 10;
    let r = &x; // &x는 x의 공유된 레퍼런스다.
    assert!(*r == 10); // r을 명시적으로 역참조 한다.
}

fn reference_example2() {
    let mut y = 32;
    let m = &mut y; // &mut y는 y의 변경할 수 있는 레퍼런스이다.
    *m += 32; // y의 값을 확인하기 위해서 m을 명시적으로 역참조한다.
    assert!(*m == 64); // 그리고 y의 새 값을 확인한다.
}

fn reference_example3() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // 명시적으로 역참조하고 있다는 점만 다를 뿐 윗줄과 똑같다.
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

fn reference_example4() {
    let mut v = vec![1973, 1968];
    v.sort(); // v의 변경할 수 있는 레퍼런스를 암묵적으로 빌려 온다.
    (&mut v).sort(); // 같은 일을 하지만 코드가 길고 번거롭다.
}

fn multiple_reference_example() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
}

fn compare_multiple_reference() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert_eq!(rrx, rry);

    assert_eq!(rx, ry); // 값은 같지만
    assert!(!std::ptr::eq(rx, ry)); // 주소는 다르다.
}

fn local_variable_reference() {
    let r;
    {
        let x = 1;
        r = &x;
    }
    assert_eq!(*r, 1); // r은 이미 드롭된 값 x를 참조하고 있음. 에러 발생.
}

fn local_variable_reference_fixed() {
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1); // x의 수명이 r보다 길다.
    }
}

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn using_smallest() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0); // 여기에 놓으면 parabola가 아직 살아있어서 컴파일 가능함.
    }
    // assert_eq!(*s, 0); // 오류: 드롭된 배열의 요소를 가리킴
}

fn struct_with_reference() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
            x
        }
    }
    println!("{}", r);
}

fn ff<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {
    r
}

fn reference_rule() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x; // 공유된 레퍼런스는 여러번 빌려와도 문제 없다.
    x += 10; // 오류: `x`는 공유된 레퍼런스가 이미 존재하므로 배정할 수 없음
    let m = &mut x; // `x`는 변경할 수 없는 레퍼런스가 존재하므로, 변경할 수 있는 레퍼런스를 빌려올 수 없음.

    println!("{}, {}, {}", r1, r2, m); // 여기서 사용되는 레퍼런스들의 수명은 적어도 여기까지는 지속되어야 함.

    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y; // 오류: `y`는 이미 변경할 수 있는 레퍼런스가 존재하므로, 두 번째 변경할 수 있는 레퍼런스를 빌려올 수 없음.
    let z = y; // 오류: `y`는 변경할 수 있는 레퍼런스가 이미 존재하므로, `y`를 사용할 수 없음.
    println!("{}, {}, {}", m1, m2, z);
}

fn reference_rule2() {
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0; // 공유된 레퍼런스로부터 다시 공유된 레퍼런스를 빌려오는 건 문제 없음
                   // let m1 = &mut r.1; // 오류: `r`은 공유된 레퍼런스이므로, 변경할 수 있는 레퍼런스를 빌려올 수 없음
    println!("{}", r0); // 여기서 r0를 사용함.
}

fn reference_rule3() {
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0; // 변경할 수 있는 레퍼런스에게서 다시 변경할 수 있는 레퍼런스를 빌려오는건 문제 없음

    *m0 = 137;
    let r1 = &m.1; // 변경할 수 있는 레퍼런스에게서 다시 공유된 레퍼런스를 빌려 오는 건 문제 없음. m0와도 겹치지 않음

    v.1; // 오류: 금지되어있는 다른 경로를 통한 접근이다.
    println!("{}", r1); // 여기서 r1을 사용함.
}
