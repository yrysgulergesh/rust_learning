fn main() {
    let int_num: i32 = 42;
    let float_num: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    let array: [i32; 3] = [1, 2, 3];

    println!("Бүтүн сан: {}", int_num);
    println!("Чыныгы сан: {}", float_num);
    println!("Буул: {}", boolean);
    println!("Тамга: {}", character);
    println!("Кортеж: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    println!("Массив: [{}]", array[0]);
}
