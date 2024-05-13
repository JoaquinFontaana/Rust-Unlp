struct Rectangulo{
longitud:f64,
ancho:f64,
}

impl Rectangulo{
    fn new(ancho:f64,longitud: f64) -> Rectangulo{
        let rectangulo = Rectangulo{
            ancho,
            longitud
        };
        rectangulo
    }
    fn calcular_area(&self) -> f64{
        return self.ancho * self.longitud;
    }
    fn calcular_perimetro(&self) -> f64{
        return self.ancho * 2.0 + self.longitud * 2.0;
    }
    fn es_cuadrado(&self) -> bool{
        if self.longitud == self.ancho{
            true
        }
        else{
            false
        }
    }
}
#[test]
fn test_calcular_area(){
    let rectangulo = Rectangulo::new(10.0, 20.0);
    assert_eq!(200.0,rectangulo.calcular_area());
}
#[test]
fn test_es_cuadrado(){
    let rectangulo = Rectangulo::new(10.0, 10.0);
    assert_eq!(true,rectangulo.es_cuadrado());
}
#[test]
fn test_calcular_perimetro(){
    let rectangulo = Rectangulo::new(10.0, 10.0);
    assert_eq!(40.0,rectangulo.calcular_perimetro());
}