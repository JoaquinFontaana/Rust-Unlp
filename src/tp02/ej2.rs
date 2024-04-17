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
    return false;
}
}

#[test]
fn test_es_primo(){
    let number = 23;
    assert_eq!(true,es_primo(number));
}