struct Fecha{
dia:i32,
mes:i32,
año:i32,
}
impl Fecha{
    fn new(dia:i32,mes:i32,año:i32) -> Fecha{
        let fecha = Fecha{
            dia,
            año,
            mes
        };
        fecha
    }
    fn es_fecha_valida(&self) -> bool{
        let mes = self.mes;
        let dia =  self.dia;
        if mes < 1 || mes > 12 {
            return false;
        }
        let bisiesto = self.es_bisiesto();
        let dias_validos = 
        match mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 =>{
                if bisiesto{
                    29
                }
                else{
                    28
                }
            }
            _=> return false,
        };
        dia >= 1 && dia <= dias_validos
    }
    fn es_bisiesto(&self) -> bool{
        if self.año % 4 == 0 && (self.año % 100 != 0 || self.año % 400 == 0){
            return true;
        }
        else{
            return false;
        }
    }
    fn sumar_dias(&mut self,dias:i32){
        let mut dias_restantes = dias;
        while dias_restantes > 0{
            let bisiesto = self.es_bisiesto();
            let dias_maximos_mes = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 =>{
                    if bisiesto{
                        29
                    }
                    else{
                        28
                    }
                }
                _=> return 
            };
            let dias_restantes_mes = dias_maximos_mes - self.dia+1; 
            if dias_restantes_mes <= dias_restantes{
                dias_restantes-= dias_restantes_mes;    
                self.dia = 1;
                if self.mes == 12{
                    self.mes = 1;
                    self.año += 1;
                }
                else{
                    self.mes+=1;
                }
            }
            else{
                self.dia += dias_restantes;
                dias_restantes= 0;
            }
        }
    }
    fn es_mayor(&self, fecha: &Fecha) -> bool {
        if self.año == fecha.año {
            if self.mes == fecha.mes {
                return self.dia < fecha.dia;
            } else {
                return self.mes < fecha.mes;
            }
        } else {
            return self.año < fecha.año;
        }
    }
}

#[test]
fn test_sumar_dias() {
    let mut fecha = Fecha::new(14, 3, 2005);
    fecha.sumar_dias(365); // Por ejemplo, sumar 10 días a la fecha inicial

    // Crear manualmente una instancia de Fecha con los valores esperados
    let fecha_esperada = Fecha::new(14, 3, 2006);

    // Comprobar si las fechas son iguales
    assert_eq!(fecha.dia, fecha_esperada.dia);
    assert_eq!(fecha.mes, fecha_esperada.mes);
    assert_eq!(fecha.año, fecha_esperada.año);
}
#[test]
fn test_sumar_dias_en_bisiesto() {
    let mut fecha = Fecha::new(28, 2, 2020);
    fecha.sumar_dias(1); // Sumar un día al 28 de febrero de 2020

    // Crear manualmente una instancia de Fecha con los valores esperados
    let fecha_esperada = Fecha::new(29, 2, 2020);

    // Comprobar si las fechas son iguales
    assert_eq!(fecha.dia, fecha_esperada.dia);
    assert_eq!(fecha.mes, fecha_esperada.mes);
    assert_eq!(fecha.año, fecha_esperada.año);
}
#[test]
fn test_es_bisiesto(){
    let fecha=  Fecha::new(1, 1, 2020);
    assert_eq!(true,fecha.es_bisiesto());
}
#[test]
fn test_es_mayor(){
    let fecha = Fecha::new(3, 2, 2002);
    let fecha_comparar = Fecha::new(4,2,2002);
    assert_eq!(fecha.es_mayor(&fecha_comparar),true);
}