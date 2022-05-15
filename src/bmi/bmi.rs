fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    return s.trim().parse().expect("数値変換エラー");
}

fn main() {
    let height_cm = input("身長(cm)は? ");
    let weight = input("体重(kg)は? ");
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);
    if bmi < 18.5 { println!("低体重"); }
    else if bmi < 25.0 { println!("普通体重"); }
    else if bmi < 30.0 { println!("肥満1度"); }
    else if bmi < 35.0 { println!("肥満2度"); }
    else if bmi < 40.0 { println!("肥満3度"); }
    else { println!("肥満4度"); }
}
