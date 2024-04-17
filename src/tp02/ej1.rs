pub fn es_par(number:i32) -> bool{
    return if number % 2 == 0{
        true
    } else{
        false
    }
}

#[test]
fn test_es_par(){
    let number = 3;
    assert_eq!(false,es_par(number));
}