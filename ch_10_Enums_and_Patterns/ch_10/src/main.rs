// ****************************************************
//  matchの条件部に参照パターンrefを使う例 (p226)
// ****************************************************
fn ex_referece_pattern1() {
    println!("*********** ex_referece_pattern1 ***********");

    struct Account {
        pub name: String,
        pub language: String,
        pub amount: u32
    }
    
    fn printAmount(a: &Account) {
        println!("amount: {}", a.amount);
    }
    
    let account = Account {
        name: String::from("konao"),
        language: String::from("Japanese"),
        amount: 10000
    };

    // この部分のコードは(★)でエラーになる．
    // ＜理由＞
    // accountにマッチしたときに、(1)でaccountの要素(name, language)がmoveされているので、
    // ★でaccountを参照しようとしたときに、「すでにmoveされている」というエラーになる．
    // match account {
    //     Account { name, language, .. } => {  // (1)
    //         println!("name: {}, language: {}", &name, &language);
    //         printAmount(&account);  // (★)
    //     }
    // }

    // こっちは動く．
    // ＜理由＞
    // (2)でaccountの要素(name, language)にref指定をしているため、
    // これらの値はaccountからmoveされず、name, languageは参照を持っているだけになる．
    // よって(3)でaccountを参照しようとしても、（まだaccountの値は残っているため）エラーにはならない．
    match account {
        Account { ref name, ref language, .. } => { // (2)
            println!("name: {}, language: {}", *name, *language);
            printAmount(&account);  // (3)
        }
    }
}

// ****************************************************
//  matchの条件部に参照パターン&を使う例 (p227)
// ****************************************************
fn ex_referece_pattern2() {
    println!("*********** ex_referece_pattern2 ***********");

    struct Point3d {
        pub x: f64,
        pub y: f64,
        pub z: f64
    }

    struct Sphere {
        center: Point3d,
        radius: f64
    }

    impl Sphere {
        pub fn new(x: f64, y: f64, z: f64, radius: f64) -> Self {
            Self {
                center: Point3d { x, y, z },
                radius
            }
        }

        pub fn center(&self) -> &Point3d {
            &self.center
        }
    }

    let sphere = Sphere::new(100.0, 0.0, 200.0, 50.0);
    match sphere.center() {
        &Point3d { x, y, z } => {   // &がなくてもエラーにならなかった.. (本には&ないとダメなように書いてあるんだけど)
            println!("center: ({}, {}, {})", x, y, z);
        }
    }
}

// ****************************************************
//  matchの条件部に参照パターン&とrefを使う例 (p228)
// ****************************************************
fn ex_referece_pattern3() {
    println!("*********** ex_referece_pattern3 ***********");

    struct Car {
        pub engine: String,
        pub country: String,
    }

    impl Car {
        pub fn get_reference(&self) -> &Self {
            &self
        }
    }

    let car = Car {
        engine: String::from("hybrid"),
        country: String::from("Japan")
    };

    match car.get_reference() {
        &Car { ref engine, .. } => {    // refないとエラーになる(engineはＳtring型なのでcopyableではないため)
            println!("engine: {}", engine);
        }
    }
}

fn main() {
    ex_referece_pattern1();
    ex_referece_pattern2();
    ex_referece_pattern3();
}
