use std::iter;

/*
Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones.x1
 */
struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero
}
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Others
}
struct Playlist{
    canciones:Vec<Cancion>,
    nombre:String
}
impl Cancion {
    fn new(titulo:String,artista:String,genero:Genero) -> Cancion{
        let cancion = Cancion{
            titulo,
            artista,
            genero
        };
        return cancion;
    }
}

impl Genero{
    fn to_string(&self) -> String{
        let str_genero = match self{
            Genero::Jazz => "Jazz".to_string(),
            Genero::Others => "Otro".to_string(),
            Genero::Pop => "Pop".to_string(),
            Genero::Rap => "Rap".to_string(),
            Genero::Rock => "Rock".to_string()
        };
        str_genero
    }
}
impl PartialEq for Cancion{
    fn eq(&self,other: &Self) -> bool{
        self.artista == other.artista && self.titulo == other.titulo  && self.genero.to_string() == other.genero.to_string()
    }
}

impl Playlist {
    fn new(nombre:String,canciones:Vec<Cancion>) -> Playlist{
        let playlist = Playlist{
            nombre,
            canciones
        };
        return playlist;
    }
    fn agregar_cancion(&mut self,cancion:Cancion){
        self.canciones.push(cancion);
    }
    fn eliminar_cancion(&mut self,cancion:Cancion){
        self.canciones.retain(|a| *a != cancion);
    }
    fn mover_cancion(&mut self,cancion:Cancion,new_position:usize){
        if new_position > 0 && new_position <= self.canciones.len(){ 
            for i in 0..self.canciones.len(){
                if self.canciones[i] == cancion{
                    let cancion_to_move= self.canciones.remove(i);
                    self.canciones.insert(new_position, cancion_to_move);
                }
            }
        }
    }
    fn buscar_cancion_por_nombre(&self,nombre:String) -> Vec<&Cancion>{
        let mut coincidencias: Vec<&Cancion> = Vec::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].titulo == nombre{
                coincidencias.push(&self.canciones[i]);
            }
        }
        coincidencias
    }
    fn canciones_por_genero(&self,genero:Genero) -> Vec<&Cancion>{
        let genero_str = genero.to_string();
        let iter_canciones = self.canciones.iter();
        iter_canciones.filter(|c| *c.genero.to_string() == genero_str).collect()
    }
    fn canciones_artista(&self,artista:String) -> Vec<&Cancion>{
        let iter_canciones = self.canciones.iter();
        iter_canciones.filter(|c| *c.artista == artista).collect()
    }
    fn modificar_titulo(&mut self,nuevo_titulo:String){
        self.nombre = nuevo_titulo;
    }
    fn eliminar_todas_canciones(&mut self){
        self.canciones.clear();
    }
}
