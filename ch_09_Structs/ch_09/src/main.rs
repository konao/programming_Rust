// ***********************************************
//  .. EXPRのサンプル (p195)
// ***********************************************

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }  // Copy, Cloneを自動導出してコピーできるようにする

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height/2, .. b };
        // heightメンバ以外はbから引き継ぐ
        // b.nameの所有権はbroom1に移る（StringはCopyトレイトを持っていないため）
    let mut broom2 = Broom { name: broom1.name.clone(), .. broom1};
        // StringはCopyトレイトを持っていないので、明示的にclone()で名前のコピーを作る．
        // nameメンバ以外はbroom1から引き継ぐ
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    
    (broom1, broom2)    // 返り値
}

fn ex1() {
    println!("*********** ex1 ***********");

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    println!("name: {}, height: {}, health: {}", hokey1.name, hokey1.height, hokey1.health);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    println!("name: {}, height: {}, health: {}", hokey2.name, hokey2.height, hokey2.health);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);
}

// ------------------------------------------------------
use std::fmt;

// 下の★でprintln!を使っているため、Tにfmt::Debugトレイト制約が必要
pub struct Queue<T> where T: fmt::Debug {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Queue<T> where T: fmt::Debug {
    pub fn new() -> Self {  // Selfは、このメソッドの定義対象の構造体を指す（ここではQueue<T>と同じ）
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.younger.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // ★
    pub fn print(&self) {
        for x in self.younger.iter() {
            println!("{:?}", *x);
        }
    }
}

// ***********************************************
//  構造体でGenericを使うサンプル (p202)
// ***********************************************
fn ex2() {
    println!("*********** ex2 ***********");

    let mut q = Queue::<char>::new();
    q.push('A');
    q.print();

    let mut r = Queue::new();
    r.push(3.141592);
    r.push(2.71828);
    r.print();
}

// ------------------------------------------------------

// ***********************************************
//  構造体のメンバに外部への参照を持たせる (p204)
// ***********************************************
fn ex3() {
    println!("*********** ex3 ***********");

    struct Extrema<'elt> {
        greatest: &'elt i32,
        least: &'elt i32
    }

    // 引数sliceの生存期間と、返り値Extremaの生存期間が同じ
    // --> 返り値には、引数sliceへの参照が入っている、と言いたい
    fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {

        // greatest, leastは共に、引数sliceへの参照を保持
        let mut greatest = &slice[0];
        let mut least = &slice[0];

        for i in 1..slice.len() {
            if slice[i] < *least { least = &slice[i]; }
            if slice[i] > *greatest { greatest = &slice[i]; }
        }

        Extrema{ greatest, least }
    }

    let a = [0, -3, 0, 15, 48];

    let e = find_extrema(&a);   // コールする側にはlifetimeパラメータは要らない
        // eにはaの要素への参照が入っている

    println!("greatest = {}", *e.greatest);
    println!("least = {}", *e.least);
}

fn main() {
    ex1();
    ex2();
    ex3();
}

/* 実行結果

    Finished dev [unoptimized + debuginfo] target(s) in 1.34s
     Running `target\debug\ch_09.exe`
*********** ex1 ***********
name: Hokey I, height: 30, health: 100
name: Hokey II, height: 30, health: 100
*********** ex2 ***********
*********** ex3 ***********
greatest = 48
least = -3

*/