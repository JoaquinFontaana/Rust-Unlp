pub fn cantidad_de_cadenas_mayor_a(limite: i32, cadenas: &[String; 3]) -> i32 {
    let mut contador = 0;
    for i in 0..cadenas.len() {
        if cadenas[i].len() as i32 > limite {
            contador += 1;
        }
    }
    contador
}

#[test]
fn test_cantidad_de_cadenas_mayor_a() {
    let cadenas = [
        "hola".to_string(),
        "jfjfdfjjf".to_string(),
        "si soy".to_string(),
    ];
    assert_eq!(2, cantidad_de_cadenas_mayor_a(4, &cadenas));
}
