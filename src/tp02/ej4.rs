pub fn cantidad_impares(numeros: &[i32]) -> i32{
    let mut contador = 0;
    for &numero in numeros.iter(){
        if numero % 2 == 1{
            contador+= 1;
        }
    }
    return contador;
}

#[test]
fn test_cantidad_impares(){
    let numeros = [2,41,3,4,6,7];
    assert_eq!(3,cantidad_impares(&numeros));
}