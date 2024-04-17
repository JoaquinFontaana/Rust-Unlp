pub fn cantidad_de_mayores(numeros:&[i32;4],limite:i32) -> i32{
    let mut contador = 0;
    for i in 0..numeros.len(){
        if numeros[i] > limite{
            contador+= 1;
        }
    }
    return contador;
}

#[test]
fn test_cantidad_de_mayores(){
    let numeros = [2,4,56,6];
    let limite = 3;
    assert_eq!(3,cantidad_de_mayores(&numeros, limite));
}