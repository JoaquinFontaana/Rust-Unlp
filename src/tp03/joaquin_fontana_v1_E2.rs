//Joaquin Fontana, 45380413, Fonta
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
struct Reporte{
    info:Vec<RegistroReporte>,
    total_canciones:i32
}
struct RegistroReporte{
    cancion:Cancion,
    posicion:i32
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
impl Clone for Cancion {
    fn clone(&self) -> Self {
        Cancion {
            titulo: self.titulo.clone(),
            artista: self.artista.clone(),
            genero: match self.genero{
                Genero::Rock => Genero::Rock,
                Genero::Pop => Genero::Pop,
                Genero::Rap => Genero::Rap,
                Genero::Jazz => Genero::Jazz,
                Genero::Others => Genero::Others,
            },
        }
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
        if new_position >= 0 && new_position <= self.canciones.len()-1{ 
            for i in 0..self.canciones.len(){
                if self.canciones[i] == cancion{
                    let cancion_to_move= self.canciones.remove(i);
                    self.canciones.insert(new_position, cancion_to_move);
                }
            }
        }
    }
    fn buscar_cancion_por_nombre(&self,nombre:String) -> Option<&Cancion>{
        for i in 0..self.canciones.len(){
            if self.canciones[i].titulo == nombre{
                return Some(&self.canciones[i]);
            }
        }
        return None;
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
    fn generar_reporte_genero(&self,genero:Genero) -> Reporte{
        let mut reporte = Reporte::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].genero.to_string() == genero.to_string(){
                let registro_reporte = RegistroReporte::new(i as i32, self.canciones[i].clone());
                reporte.info.push(registro_reporte);
                reporte.total_canciones+=1;
            }
        }
        reporte
    }
}
impl Reporte{
    fn new() -> Reporte{
        let info = Vec::new();
        let total_canciones = 0;
        let reporte = Reporte{
            info,
            total_canciones
        };
        reporte
    }
}
impl RegistroReporte{
    fn new(posicion:i32,cancion:Cancion) -> RegistroReporte{
        let registro = RegistroReporte{
            cancion,
            posicion
        };
        registro
    }
}
fn generar_playlist() -> Playlist{
    let cancion = Cancion::new("Hello coto".to_string(), "Duki".to_string(),Genero::Rap);
    let cancion2 = Cancion::new("Iba a llamarte".to_string(), "Cro".to_string(),Genero::Rock);
    let cancion3 = Cancion::new("Everlong".to_string(), "palo".to_string(),Genero::Rock);
    let mut canciones = Vec::new();
    canciones.push(cancion);
    canciones.push(cancion2);
    canciones.push(cancion3);
    Playlist::new("Fontana".to_string(), canciones)
}
#[test]
fn test_eliminar_todas_canciones(){
    let mut playlist = generar_playlist();
    playlist.eliminar_todas_canciones();
    assert_eq!(playlist.canciones.len(),0);
}
#[test]
fn test_canciones_por_genero() {
    let playlist = generar_playlist();
    let canciones_rock = playlist.canciones_por_genero(Genero::Rock);
    assert_eq!(canciones_rock.len(), 2);
    for cancion in canciones_rock {
        assert_eq!(cancion.genero.to_string(), Genero::Rock.to_string());
    }
}
#[test]
fn test_agregar_cancion(){
    let mut playlist = generar_playlist();
    let cancion = Cancion::new("Mori".to_string(),"Flor".to_string(),Genero::Pop);
    playlist.agregar_cancion(cancion);
    assert_eq!(playlist.canciones.len(),4);
    assert_eq!(playlist.canciones[3].artista,"Flor".to_string());
}
#[test]
fn test_eliminar_cancion(){
    let mut playlist = generar_playlist();
    let cancion = Cancion::new("Hello coto".to_string(), "Duki".to_string(),Genero::Rap);
    playlist.eliminar_cancion(cancion);
    assert_eq!(playlist.canciones.len(),2);
}
#[test]
fn test_modificar_titulo(){
    let mut playlist = generar_playlist();
    playlist.modificar_titulo("all".to_string());
    assert_eq!(playlist.nombre,"all".to_string());
}
#[test]
fn test_mover_cancion(){
    let mut playlist = generar_playlist();
    let cancion = Cancion::new("Hello coto".to_string(), "Duki".to_string(),Genero::Rap);
    playlist.mover_cancion(cancion, 2);
    assert_eq!(playlist.canciones[2].titulo,"Hello coto".to_string());
}
#[test]
fn test_buscar_cancion_por_nombre(){
    let playlist = generar_playlist();
    let result = playlist.buscar_cancion_por_nombre("Hello Coto".to_string());
    let cancion = match result{
        Some(cancion) => cancion,
        None=> return  
    };
    assert_eq!("Hello coto".to_string(),cancion.titulo);
}
#[test]
fn test_canciones_por_artista(){
    let playlist = generar_playlist();
    let canciones = playlist.canciones_artista("Duki".to_string());
    assert_eq!(1,canciones.len());
}
#[test]
fn test_generar_reporte_genero(){
    let playlist = generar_playlist();
    let reporte =playlist.generar_reporte_genero(Genero::Rock);
}