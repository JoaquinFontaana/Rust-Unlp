struct Persona{
    nombre:String,
    edad:i32,
    direccion:String
}
impl Persona{
    fn new(nombre:String,edad:i32,direccion:String) -> Persona{
        let persona = Persona{
            nombre,
            edad,
            direccion
        };
        return persona
    }
    fn to_string(&self)-> String{
        let text = format!("{} tiene {} años y vive en {}",self.nombre,self.edad,self.direccion);
        return text;
    }
    fn obtener_edad(&self) -> i32{
        return self.edad;
    }
    fn actualizar_direccion(&mut self,nueva_direccion:String){
        self.direccion = nueva_direccion;
    }
}
#[test]
fn test_to_string(){
    let persona = Persona::new("Joaquin".to_string(),19,"La plata".to_string());
    assert_eq!(persona.to_string(),"Joaquin tiene 19 años y vive en La plata");
}
#[test]
fn test_actualizar_direccion(){
    let nueva_direccion = "Trelew".to_string();
    let mut persona = Persona::new("Joaquin".to_string(),19,"La plata".to_string());
    persona.actualizar_direccion(nueva_direccion);
    assert_eq!(persona.direccion,"Trelew".to_string());
}
#[test]
fn test_obtener_edad(){
    let persona = Persona::new("Joaquin".to_string(),19,"La plata".to_string());
    assert_eq!(persona.obtener_edad(),19);
}