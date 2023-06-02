fn main() {
    let a = 12;
    let b = 3;

    let calc_add = add::add(a, b);
    let calc_sub = sub::sub(a, b);
    let calc_mul = mul::mul(a, b);
    let calc_div = div::div(a, b);

    println!("{}, {}", a, b);
    println!("Add: {}", calc_add);
    println!("Sub: {}", calc_sub);
    println!("Mul: {}", calc_mul);
    println!("Div: {}", calc_div);
}
