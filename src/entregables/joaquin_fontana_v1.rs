//Joaquin Fontana, DNI 45380413, Fonta
pub fn calcular_precio_con_impuestos(cantidades: &[i32],precios: &[f64],impuesto: f64) -> f64{
    let mut total = 0.0;
    for i in 0..cantidades.len(){
        total += cantidades[i] as f64*precios[i];
    }
    return total - total * (impuesto / 100.0)
}

#[test]
pub fn test_calcular_precios_con_impuestos(){
    let cantidades = [1,2,3];
    let precios = [20.0,15.0,10.2];
    let impuesto = 10.0;
    assert_eq!(80.6- (80.6 * 10.0 / 100.0),calcular_precio_con_impuestos(&cantidades, &precios, impuesto));
}
