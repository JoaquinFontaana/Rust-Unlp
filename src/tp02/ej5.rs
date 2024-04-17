pub fn duplicar_valores(numeros:&[f32;3]) -> [f32;3] {
    let mut numeros_duplicados: [f32;3] = [0.0;3];
    for i in 0..numeros.len(){
        numeros_duplicados[i] = numeros[i]*2.0;
    }
    numeros_duplicados
}

#[test]
fn test_duplicar_valores(){
    let numeros: [f32;3] =[2.0,5.1,3.0];
    assert_eq!([4.0,10.2,6.0],duplicar_valores(&numeros));
}
