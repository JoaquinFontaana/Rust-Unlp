pub fn suma_pares(numeros: &[i32]) -> i32{
    let mut suma = 0;
    for &numero in numeros.iter(){
        if numero % 2 ==0{
            suma+=numero;
        }
    }
    return suma;
}
#[test]
fn test_suma_pares(){
    let numeros = [2,5,6,7,3,4];
    assert_eq!(12,suma_pares(&numeros));
}
