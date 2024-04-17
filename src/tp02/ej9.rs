pub fn cantidad_en_rango(inferior:i32,superior:i32,numeros: &[i32;3]) -> i32{
    let mut contador = 0;
    for i in 0..numeros.len(){
        if numeros[i] > inferior && numeros[i] < superior  { 
            contador += 1;
        }
    }
    return contador;
}

#[test]
fn test_cantidad_en_rango(){
    let numeros = [2,3,5];
    assert_eq!(1,cantidad_en_rango(2, 5, &numeros));
}

