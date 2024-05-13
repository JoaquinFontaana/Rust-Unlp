use std::f64;

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
        if self.base == self.lado1 && self.lado1 == self.lado2 {
            Tipo::Equilatero
        } else if self.base == self.lado1 || self.base == self.lado2 || self.lado1 == self.lado2 {
            Tipo::Isosceles
        } else {
            Tipo::Escaleno
        }
    }
    fn calcular_area(&self) -> f64 {
        let semi_perimetro = (self.base + self.lado1 + self.lado2) / 2.0;
        let area = (semi_perimetro * (semi_perimetro - self.base) * (semi_perimetro - self.lado1) * (semi_perimetro - self.lado2)).sqrt();
        return area;
    }
}
#[test]
fn test_calcular_area(){
    let triangulo = Triangulo::new(2.0, 5.0, 7.0);
    assert_eq!(triangulo.calcular_area(),14.6969);
}