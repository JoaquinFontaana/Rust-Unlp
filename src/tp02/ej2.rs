pub fn es_primo(number:i32) -> bool{
if number > 1{
    let raiz_n = (number as f64).sqrt() as i32;
    for i in 2..=raiz_n{
        if number % i == 0{
            return false;
        }
    }
    return true;
}
else{
    println!("El numero ingresado debe ser mayor a 1");
    return false;
}
}