    use std::collections::VecDeque;
    use crate::tp03::ej03::Fecha;

    struct Veterinaria<'a> {
        nombre: String,
        direccion: String,
        id: String,
        cola_atencion: VecDeque<Mascota<'a>>,
        atenciones:Vec<Atencion<'a>>
    }

    struct Mascota<'a> {
        nombre: String,
        edad: u8,
        tipo: TipoAnimal,
        duenio: &'a Duenio,
    }

    struct Duenio {
        nombre: String,
        direccion: String,
        telefono: String,
    }

    struct Atencion<'a> {
        datos_mascota: Mascota<'a>,
        diagnostico: String,
        tratamiento: String,
        fecha_proxima: Option<Fecha>,
    }

    struct Cadena<'a> {
        veterinarias: Vec<Veterinaria<'a>>,
    }

    enum TipoAnimal {
        Perro,
        Gato,
        Caballo,
        Otro,
    }
    impl<'a> Mascota<'a>{
        fn new(nombre:String,duenio:&Duenio,edad:u8,tipo:TipoAnimal) -> Mascota{
            let animal = Mascota{
                nombre,
                duenio,
                edad,
                tipo
            };
            animal
        }
    }
    impl<'a> Atencion<'a>{
        fn new(datos_mascota:Mascota,diagnostico:String,tratamiento:String,fecha_proxima:Option<Fecha>) -> Atencion{
            let at = Atencion{
                datos_mascota,
                diagnostico,
                tratamiento,
                fecha_proxima
            };
            at
        }
    }
    // Implementación de métodos para la estructura Veterinaria
    impl<'a> Veterinaria<'a> {
        fn new(nombre: String, direccion: String, id: String) -> Veterinaria<'a> {
            let cola_atencion: VecDeque<Mascota> = VecDeque::new();
            let atenciones: Vec<Atencion> = Vec::new();
            Veterinaria {
                nombre,
                direccion,
                id,
                cola_atencion,
                atenciones
            }
        }

        fn agregar_mascota_cola(&mut self, mascota: Mascota<'a>) {
            self.cola_atencion.push_back(mascota);
        }

        fn agregar_mascota_prioridad(&mut self, mascota: Mascota<'a>) {
            self.cola_atencion.push_front(mascota);
        }

        fn atender_proxima_mascota(&mut self) -> Option<Mascota<'a>> {
            self.cola_atencion.pop_front()
        }
        fn eliminar_mascota_cola(&mut self, mascota: Mascota<'a>) -> Option<Mascota<'a>> {
            // Recorremos la cola de atención utilizando un bucle for sobre los índices
            for i in 0..self.cola_atencion.len() {
                // Verificamos si la mascota actual coincide con los criterios proporcionados
                if self.cola_atencion[i].edad == mascota.edad &&
                self.cola_atencion[i].nombre == mascota.nombre &&
                self.cola_atencion[i].duenio.telefono == mascota.duenio.telefono {
                    return Some(self.cola_atencion.remove(i).unwrap());
                }
            }
            None
        }
        fn registrar_atencion(&mut self,mascota:Mascota<'a>,diagnostico:String,tratamiento:String,fecha_proxima:Option<Fecha>){
            let datos_mascota = Mascota::new(mascota.nombre, mascota.duenio, mascota.edad, mascota.tipo);
            let atencion = Atencion::new(datos_mascota, diagnostico, tratamiento, fecha_proxima);
            self.atenciones.push(atencion);
        }

    }
    impl<'a> Cadena<'a> {
        fn new() -> Cadena<'a>{
            let veterinarias: Vec<Veterinaria> = Vec::new();
            let cadena = Cadena{
                veterinarias
            };
            cadena
        }
        fn agregar_veterinaria(&mut self,vet:Veterinaria<'a>){
            self.veterinarias.push(vet);
        }
        fn buscar_atencion(&'a mut self, nombre_mascota: &str, nombre_duenio: &'a str, telefono: &str) -> Option<&'a mut Atencion<'a>> {
            for vet in &mut self.veterinarias {
                for atencion in &mut vet.atenciones {
                    if atencion.datos_mascota.duenio.nombre == nombre_duenio &&
                    atencion.datos_mascota.duenio.telefono == telefono &&
                    atencion.datos_mascota.nombre == nombre_mascota {
                        return Some(atencion);
                    }
                }
            }
            None
        }

        fn modificar_diagnostico(&'a mut self, nombre_mascota: &'a str, nombre_duenio: &'a str, telefono: &'a str, nuevo_diagnostico: String) {
            if let Some(atencion) = self.buscar_atencion(nombre_mascota, nombre_duenio, telefono) {
                atencion.diagnostico = nuevo_diagnostico;
            }
        }

        fn modificar_fecha(&'a mut self, nombre_mascota: &'a str, nombre_duenio: &'a str, telefono: &'a str, nueva_fecha: Fecha) {
            if nueva_fecha.es_fecha_valida(){
                if let Some(atencion) = self.buscar_atencion(nombre_mascota, nombre_duenio, telefono) {
                atencion.fecha_proxima = Some(nueva_fecha) ;
                }
            }
        }

        fn eliminar_atencion(&mut self, atencion_eliminar: Atencion<'a>) {
            for vet in &mut self.veterinarias {
                let mut i = 0;
                while i < vet.atenciones.len() {
                    let atencion = &vet.atenciones[i];
                    if atencion.datos_mascota.duenio.nombre == atencion_eliminar.datos_mascota.duenio.nombre &&
                    atencion.datos_mascota.nombre == atencion_eliminar.datos_mascota.nombre &&
                    atencion.datos_mascota.duenio.telefono == atencion_eliminar.datos_mascota.duenio.telefono {
                        vet.atenciones.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_agregar_mascota_cola() {
            // Crear estructuras necesarias
            let  duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
    
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
    
            // Realizar la prueba
            veterinaria.agregar_mascota_cola(mascota);
    
            // Verificar que la mascota se ha agregado correctamente a la cola de atención
            assert_eq!(veterinaria.cola_atencion.len(), 1);
        }

        #[test]
        fn test_agregar_mascota_prioridad() {
            // Crear estructuras necesarias
            let  duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let copia_mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));

            // Realizar la prueba
            veterinaria.agregar_mascota_prioridad(mascota);

            // Verificar que la mascota se ha agregado correctamente a la cola de atención con prioridad
            assert_eq!(veterinaria.cola_atencion.len(), 1);
            assert_eq!(veterinaria.cola_atencion.front().unwrap().nombre, copia_mascota.nombre);
        }

        #[test]
        fn test_atender_proxima_mascota() {
            // Crear estructuras necesarias
            let  duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let copia_mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
            veterinaria.agregar_mascota_cola(mascota);

            // Realizar la prueba
            let mascota_atendida = veterinaria.atender_proxima_mascota();

            // Verificar que se ha atendido la próxima mascota correctamente
            assert_eq!(mascota_atendida.unwrap().nombre, copia_mascota.nombre);
            assert_eq!(veterinaria.cola_atencion.len(), 0);
        }

        #[test]
        fn test_eliminar_mascota_cola_existente() {
            // Crear estructuras necesarias
            let  duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let copia_mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let copia2_mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
            veterinaria.agregar_mascota_cola(mascota);

            // Realizar la prueba
            let mascota_eliminada = veterinaria.eliminar_mascota_cola(copia_mascota);

            // Verificar que la mascota se ha eliminado correctamente de la cola de atención
            assert_eq!(mascota_eliminada.unwrap().nombre, copia2_mascota.nombre);
            assert_eq!(veterinaria.cola_atencion.len(), 0);
        }

        #[test]
        fn test_eliminar_mascota_cola_inexistente() {
            // Crear estructuras necesarias
            let duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
            veterinaria.agregar_mascota_cola(mascota);

            // Crear otra mascota que no se agregará a la cola
            let otra_mascota = Mascota::new(String::from("Tom"), &duenio, 3, TipoAnimal::Gato);

            // Realizar la prueba
            veterinaria.eliminar_mascota_cola(otra_mascota);

            assert_eq!(veterinaria.cola_atencion.len(), 1);
        }

        #[test]
        fn test_registrar_atencion() {
            // Crear estructuras necesarias
            let duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));

            // Realizar la prueba
            veterinaria.registrar_atencion(mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None);

            // Verificar que la atención se ha registrado correctamente
            assert_eq!(veterinaria.atenciones.len(), 1);
        }

/*         #[test]
        fn test_modificar_diagnostico() {
            // Crear estructuras necesarias
            let mut duenio = Duenio {
                nombre: String::from("Juan"),
                direccion: String::from("Calle 123"),
                telefono: String::from("123456789"),
            };
            let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
            let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
            let mut cadena =Cadena::new();
            cadena.agregar_veterinaria(veterinaria);
            cadena.veterinarias[0].registrar_atencion(mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None);

            // Realizar la prueba modificando el diagnóstico de la atención registrada
            cadena.modificar_diagnostico("Bobby", "Juan", "123456789", String::from("Nuevo Diagnóstico"));

            // Verificar que el diagnóstico ha sido modificado correctamente
            let atencion_encontrada = cadena.buscar_atencion("Bobby", "Juan", "123456789");
            let atencion = match atencion_encontrada {
                Some(atencion) => atencion,
                None => return
            };
            assert_eq!(atencion.diagnostico, "Nuevo Diagnóstico");
        }
*/
        #[test]
    fn test_buscar_atencion_existente() {
        // Crear estructuras necesarias
        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456789"),
        };
        let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
        let mut cadena = Cadena::new();
        let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
        veterinaria.registrar_atencion(mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None);
        cadena.veterinarias.push(veterinaria);

        // Realizar la prueba
        let atencion_encontrada = cadena.buscar_atencion("Bobby", "Juan", "123456789");

        // Verificar que la atención existente ha sido encontrada
        assert!(atencion_encontrada.is_some());
    }

    #[test]
    fn test_buscar_atencion_inexistente() {
        // Crear estructuras necesarias
        let  duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456789"),
        };
        let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
        let mut cadena = Cadena { veterinarias: vec![] };
        let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
        veterinaria.registrar_atencion(mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None);
        cadena.veterinarias.push(veterinaria);

        // Realizar la prueba buscando una atención inexistente
        let atencion_encontrada = cadena.buscar_atencion("Tom", "Juan", "123456789");

        // Verificar que la atención inexistente no ha sido encontrada
        assert!(atencion_encontrada.is_none());
    }

    #[test]
    fn test_eliminar_atencion_existente() {
        // Crear estructuras necesarias
        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456789"),
        };
        let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
        let copia_mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
        let mut cadena = Cadena { veterinarias: vec![] };
        let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
        veterinaria.registrar_atencion(mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None);
        cadena.veterinarias.push(veterinaria);

        // Realizar la prueba eliminando la atención registrada
        cadena.eliminar_atencion(Atencion::new(copia_mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None));

        // Verificar que la atención ha sido eliminada correctamente
        assert_eq!(cadena.veterinarias[0].atenciones.len(), 0);
    }

    #[test]
    fn test_eliminar_atencion_inexistente() {
        // Crear estructuras necesarias
        let mut duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456789"),
        };
        let mascota = Mascota::new(String::from("Bobby"), &duenio, 5, TipoAnimal::Perro);
        let mut cadena = Cadena { veterinarias: vec![] };
        let mut veterinaria = Veterinaria::new(String::from("Veterinaria A"), String::from("Dirección 123"), String::from("001"));
        veterinaria.registrar_atencion(mascota, String::from("Diagnóstico A"), String::from("Tratamiento A"), None);
        cadena.veterinarias.push(veterinaria);
            // Crear otra atención que no se eliminará
    let otra_mascota = Mascota::new(String::from("Tom"), &duenio, 3, TipoAnimal::Gato);
    cadena.veterinarias[0].registrar_atencion(otra_mascota, String::from("Diagnóstico B"), String::from("Tratamiento B"), None);

    // Realizar la prueba eliminando una atención inexistente
    let atencion_a_eliminar = Atencion {
        datos_mascota: Mascota::new(String::from("Lassie"), &duenio, 4, TipoAnimal::Perro),
        diagnostico: String::from("Diagnóstico C"),
        tratamiento: String::from("Tratamiento C"),
        fecha_proxima: None,
    };
    cadena.eliminar_atencion(atencion_a_eliminar);

    // Verificar que la atención inexistente no ha sido eliminada
    assert_eq!(cadena.veterinarias[0].atenciones.len(), 2);
}

    }


