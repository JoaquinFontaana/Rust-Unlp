/*

a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad.
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario.
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario.
e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.
Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure
 */
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}
fn salario_mayor_que(personas:Vec<Persona>,salario:f64) -> Vec<Persona>{
    let iter_personas= personas.into_iter();
    iter_personas.filter(|p|p.salario > salario).collect()
}
//Pedir que me expliquen como funciona el lifetime en esta funcion
fn ciudadanos_de_x_mayores_que_x<'a>(personas:Vec<Persona<'a>>,ciudad:&'a str, edad:u8) -> Vec<Persona<'a>>{
    personas.into_iter().filter(|p|*p.ciudad == *ciudad && p.edad > edad).collect()
}
fn todos_viven_en_ciudad_x(personas:Vec<Persona>,ciudad:& str) -> bool{
    personas.iter().all(|p|p.ciudad == ciudad)
}
fn alguien_vive_en_ciudad_x(personas:Vec<Persona>,ciudad:& str) -> bool{
    personas.iter().any(|p|p.ciudad == ciudad)
}
fn existe_persona(personas:Vec<Persona>,persona:Persona) -> bool{
    personas.contains(&persona)
}
fn obtener_edades(personas:Vec<Persona>) -> Vec<u8>{
    let mut edades = Vec::new();
    let iter = personas.iter();
    iter.for_each(|p| edades.push(p.edad.clone()));
    return edades;
}
fn salario_minimo_maximo(persona:Vec<Persona>){
    let iter = persona.iter();
    let obtener_max = |p:Persona,mut max:Persona| {
        if p.salario > max.salario{
            max= p.clone();
        }
        else{
            if p.salario == max.salario{
                if p.edad > max.edad{
                    max = p.clone();
                }
            }
        }
    };
    iter.map(obtener_max());
}
impl<'a> Persona<'a>{
    fn new(salario:f64,edad:u8,nombre:&str,apellido:&str,direccion:&str,ciudad:&str) -> Persona<'a>{
        let persona = Persona{
            edad,
            ciudad,
            salario,
            nombre,
            apellido,
            direccion
        };
        return persona;
    }
}
impl<'a> PartialEq for Persona<'a>{
    fn eq(&self, other: &Self) -> bool {
        self.apellido == other.apellido && self.ciudad == other.ciudad && self.direccion == other.direccion && self.edad == other.edad && self.nombre == other.nombre && self.salario == other.salario
    }
}
impl<'a> Clone for Persona<'a>{
    fn clone(&self) -> Persona<'a>{
        Persona::new(self.salario, self.edad, self.nombre, self.apellido, self.direccion, self.ciudad)
    }
}