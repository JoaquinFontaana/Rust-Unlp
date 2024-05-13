use std::collections::LinkedList;

struct ConcesionarioAuto{
    direccion:String,
    nombre:String,
    capacidad:i32,
    autos:Vec<Auto>
}
struct Auto{
    marca:String,
    modelo:String,
    año:i32,
    precio_bruto:f64,
    color:Color,
}
impl  Auto {
    fn new(año:i32,marca:String,modelo:String,precio_bruto:f64,color:Color) -> Auto{
        let auto = Auto{
            marca,
            modelo,
            año,
            precio_bruto,
            color
        };
        return auto;
    }
}
impl ConcesionarioAuto{
    fn new(nombre:String,direccion:String,capacidad:i32) -> ConcesionarioAuto{
        let autos: Vec<Auto> = Vec::new();
        let concesionario = ConcesionarioAuto{
            direccion,
            nombre,
            capacidad,
            autos
        };
        concesionario
    }
    fn eliminar_auto(&mut self,auto:Auto){
        self.autos.retain(|a| *a != auto);
    }
    fn agregar_auto(&mut self,auto:Auto) -> bool{
        if self.autos.len() < self.capacidad as usize{
            self.autos.push(auto);
            return true;
        }
        else{
            return false;
        }
    }
    fn buscar_auto(&mut self, auto:Auto) -> Option<&Auto>{
        self.autos.iter().find(|&a| *a == auto)
    }
}
impl PartialEq for Auto {
    fn eq(&self, other: &Self) -> bool {
        self.color.to_string() == other.color.to_string() && self.año == other.año && self.marca == other.marca && self.modelo == other.modelo && self.precio_bruto == other.precio_bruto
    }
}

enum Color{
    Verde,
    Rojo,
    Azul,
    Amarillo,
    Blanco,
    Negro
}
impl Color {
    fn to_string(&self) -> String {
        let color_str = match self {
            Color::Verde => "Verde".to_string(),
            Color::Rojo => "Rojo".to_string(),
            Color::Azul => "Azul".to_string(),
            Color::Amarillo => "Amarillo".to_string(),
            Color::Blanco => "Blanco".to_string(),
            Color::Negro => "Negro".to_string(),
        };
        return color_str
    }
}
#[test]
fn test_agregar_auto(){
    let mut concesionario = ConcesionarioAuto::new("Toyota".to_string(), "Rivadavia 1895".to_string(), 10);
    let auto = Auto::new(2002,"Toyota".to_string(),"Hilux".to_string(),203423.0,Color::Blanco);
    assert_eq!(true,concesionario.agregar_auto(auto));
}
#[test]
fn test_eliminar_auto(){
    let mut concesionario = ConcesionarioAuto::new("Toyota".to_string(), "Rivadavia 1895".to_string(), 10);
    let auto = Auto::new(2002,"Toyota".to_string(),"Hilux".to_string(),203423.0,Color::Blanco);
    concesionario.agregar_auto(auto);
    let auto = Auto::new(2002,"Ford".to_string(),"Raptor".to_string(),203423.0,Color::Blanco);
    concesionario.eliminar_auto(auto);
    assert_eq!(1,concesionario.autos.len());
}
#[test]
fn test_buscar_auto(){
    
}