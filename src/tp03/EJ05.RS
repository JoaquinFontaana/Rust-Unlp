struct Producto{
    nombre:String,
    precio_bruto:f64,
    id:i32
}
impl Producto{
    fn new(nombre:String,precio_bruto:f64,id:i32) -> Producto{
        let producto = Producto{
            nombre,
            id,
            precio_bruto
        };
        return producto
    }
    fn calcular_impuesto(&self,impuesto:f64) -> f64{
        return self.precio_bruto + (self.precio_bruto * impuesto / 100.0);
    }
    fn aplicar_descuento(&self,descuento:f64) -> f64{
        return self.precio_bruto - (self.precio_bruto * descuento / 100.0);
    }
    fn calcular_precio_total(&self,descuento:Option<f64>,impuesto:Option<f64>)-> f64{
        let mut precio_total = self.precio_bruto;
        precio_total = match descuento{ 
            Some(valor) => precio_total * (1.0 - valor / 100.0),
            None => precio_total
        };
        precio_total = match impuesto{
            Some(valor) => precio_total * (1.0 + valor / 100.0),
            None=>precio_total
        };
        return precio_total;
    }
}

#[test]
fn test_calcular_precio_total(){
    let producto = Producto::new("Yerba".to_string(),200.0,293894);
    let descuento = Some(10.0);
    let impuesto = Some(20.0);
    assert_eq!(producto.calcular_precio_total(descuento,impuesto),216.0);
}