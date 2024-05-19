/*Escriba una función que reciba un vector de números enteros y retorna la cantidad de
números primos. Cree un trait para la determinación del número primo e impleméntelo
según corresponda. Utilice la función iter sobre el vector y aplique un closure para
resolverlo. */
fn primos(numeros:Vec<i32>) -> i32{
    let mut iter_vec = numeros.iter();
    let mut cantidad = 0;
    while let Some(number) = iter_vec.next(){
        if number.determinar(){
            cantidad+= 1;
        }
    }
    cantidad
}
pub trait NumeroPrimo {
    fn determinar(&self)-> bool;
}
impl NumeroPrimo for i32{
    fn determinar(&self)-> bool {
        let n = *self;
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}
#[test]
fn test_primos(){
    let mut numeros = Vec::new();
    numeros.push(2);
    numeros.push(11);
    numeros.push(5);
    numeros.push(3);
    assert_eq!(4,primos(numeros));
}