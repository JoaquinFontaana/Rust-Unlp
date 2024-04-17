pub fn suma_pares(numeros: &[i32]) -> i32{
    let mut suma = 0;
    for &numero in numeros.iter(){
        if numero % 2 ==0{
            suma+=numero;
        }
    }
    return suma;
}