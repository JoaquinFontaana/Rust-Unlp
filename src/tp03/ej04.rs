struct Triangulo{
    base:f64,
    lado1:f64,
    lado2:f64
}
enum Tipo{
    Isosceles,
    Escaleno,
    Equilatero
}
impl Triangulo{
    fn new(base:f64,lado1:f64,lado2:f64) -> Triangulo{
        let triangulo = Triangulo{
            base,
            lado1,
            lado2,
        };
        return triangulo;
    }
    fn determinar_tipo(&self) -> Tipo {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            Tipo::Equilatero
        } else if self.lado1 == self.lado2 || self.lado1 == self.lado3 || self.lado2 == self.lado3 {
            Tipo::Isosceles
        } else {
            Tipo::Escaleno
        }
    }
    fn calcular_area(&self){

    }
}