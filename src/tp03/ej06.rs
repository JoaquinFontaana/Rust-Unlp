struct Estudiante{
    nombre:String,
    id:i32,
    calificaciones: Vec<Examen>
}
struct Examen{
    nota:f64,
    nombre:String
}
impl Estudiante{
    fn new(id:i32,nombre:String,calificaciones:Vec<Examen>) -> Estudiante{
        let estudiante = Estudiante{
            nombre,
            id,
            calificaciones
        };
        estudiante
    }
    fn obtener_promedio(&self) -> f64{
        let mut suma = 0.0;
        for examen in &self.calificaciones{
            suma+=examen.nota;
        }
        return suma / self.calificaciones.len() as f64;
    }
    fn obtener_calificacion_mas_alta(&self)->f64{
        if self.calificaciones.len() > 0{
            let mut max = 0.0;
            for examen in &self.calificaciones{
                if examen.nota > max{
                    max=examen.nota;
                }
            }
            max
        }
        else{
            return -1 as f64;
        }
    }
    fn obtener_calificacion_mas_baja(&self)->f64{
        if self.calificaciones.len() > 0 { 
            let mut min = 99.0;
            for examen in &self.calificaciones{
                if examen.nota < min{
                    min=examen.nota;
                }
            }
            min
        }
        else{
            return -1 as f64;
        }
    }
}
impl Examen{
    fn new(nombre:String,nota:f64) -> Examen{
        let examen = Examen{
            nombre,
            nota
        };
        examen
    }
}
#[test]
fn test_obtener_calificacion_mas_baja(){
    let examen1 =Examen::new("Matematica".to_string(), 10.0);
    let examen2 = Examen::new("Lengua".to_string(), 4.0);
    let mut calificaciones = Vec::new();
    calificaciones.push(examen1);
    calificaciones.push(examen2);
    let estudiante = Estudiante::new(1233, "Jeremias".to_string(),calificaciones);
    assert_eq!(4.0,estudiante.obtener_calificacion_mas_baja());
}
#[test]
fn test_calificaciones_vacio_obtener_calificacion_mas_baja(){
    let calificaciones = Vec::new();
    let estudiante = Estudiante::new(1233, "Jeremias".to_string(),calificaciones);
    assert_eq!(-1.0,estudiante.obtener_calificacion_mas_baja());
}
#[test]
fn test_obtener_calificacion_mas_alta(){
    let examen1 =Examen::new("Matematica".to_string(), 10.0);
    let examen2 = Examen::new("Lengua".to_string(), 4.0);
    let mut calificaciones = Vec::new();
    calificaciones.push(examen1);
    calificaciones.push(examen2);
    let estudiante = Estudiante::new(1233, "Jeremias".to_string(),calificaciones);
    assert_eq!(10.0,estudiante.obtener_calificacion_mas_alta());
}
#[test]
fn test_obtener_promedio(){
    let examen1 =Examen::new("Matematica".to_string(), 10.0);
    let examen2 = Examen::new("Lengua".to_string(), 4.0);
    let mut calificaciones = Vec::new();
    calificaciones.push(examen1);
    calificaciones.push(examen2);
    let estudiante = Estudiante::new(1233, "Jeremias".to_string(),calificaciones);
    assert_eq!(7.0,estudiante.obtener_promedio());
}