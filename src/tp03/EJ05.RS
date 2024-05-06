struct Producto{
    nombre:string,
    precio_bruto:f64,
    id:i32
}
impl Producto{
    fn new(nombre:string,precio_bruto:f64,id:i32) -> Producto{
        let producto = Producto{
            nombre,
            id,
            precio_bruto
        };
        return producto
    }
}