use std::collections::HashMap;
use std::fmt::Display;
use::serde_json;
#[derive(Eq, Debug,PartialEq, Hash,Clone)]
enum TipoDeSuscripcion {
    Basic,
    Clasic,
    Super,
}
#[derive(Eq, PartialEq, Hash,Clone)]
struct MercadoPago {
    email: String,
    id_transaccion: String,
}
#[derive(Eq, PartialEq, Hash,Clone)]
struct TarjetaDeCredito {
    numero_tarjeta: String,
    fecha_vencimiento: String,
    cvv: String,
    nombre_titular: String,
}
#[derive(Eq, PartialEq, Hash,Clone)]
struct TransferenciaBancaria {
    numero_cuenta: String,
    clave_interbancaria: String,
    nombre_banco: String,
    nombre_tiular: String,
    numero_referencia: String,
}
#[derive(Eq, PartialEq, Hash,Clone)]
struct Cripto {
    direccion_billetera: String,
    id_transaccion: String,
    tipo_cripto: String,
}
#[derive(Eq, PartialEq, Hash,Clone)]
enum MedioDePago {
    Efectivo,
    Cripto(Cripto),
    TransferenciaBancaria(TransferenciaBancaria),
    TarjetaDeCredito(TarjetaDeCredito),
    MercadoPago(MercadoPago),
}
struct ErrorCustom(String);
impl Display for ErrorCustom{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
struct Suscripcion {
    estado: bool,
    costo: f64,
    duracion: u8,
    fecha_inicio: String,
    tipo: TipoDeSuscripcion,
    user: Usuario,
}
#[derive(Clone)]
struct Usuario {
    email: String,
    username: String,
    medio_de_pago: MedioDePago,
}
struct StreamingRust{
    usuarios: Vec<Usuario>,
    suscripciones: Vec<Suscripcion>,
}

impl TipoDeSuscripcion {
    fn get_costo(&self) -> f64 {
        match self {
            TipoDeSuscripcion::Basic => 100.0,
            TipoDeSuscripcion::Clasic => 252.5,
            TipoDeSuscripcion::Super => 304.0,
        }
    }
    fn get_duracion(&self) -> u8 {
        match self {
            TipoDeSuscripcion::Basic => 1,
            TipoDeSuscripcion::Clasic => 6,
            TipoDeSuscripcion::Super => 12,
        }
    }
}
impl Suscripcion {
    fn new(fecha_inicio: String, tipo: TipoDeSuscripcion, user: Usuario) -> Suscripcion {
        let costo = tipo.get_costo();
        let duracion = tipo.get_duracion();
        let estado = true;
        let new_suscripcion = Suscripcion {
            costo,
            duracion,
            fecha_inicio,
            tipo,
            user,
            estado,
        };
        new_suscripcion
    }
    fn dar_de_baja(&mut self) {
        self.estado = false;
        self.duracion = 0;
    }
}
impl Usuario {
    fn new(email: String, username: String, medio_de_pago: MedioDePago) -> Usuario {
        let user = Usuario {
            email,
            username,
            medio_de_pago,
        };
        user
    }
}

impl StreamingRust {
    fn new() -> StreamingRust {
        let usuarios: Vec<Usuario> = Vec::new();
        let suscripciones: Vec<Suscripcion> = Vec::new();
        let plataforma = StreamingRust {
            usuarios,
            suscripciones,
        };
        plataforma
    }
    fn agregar_usuario(&mut self, user: Usuario) {
        self.usuarios.push(user);
    }

    fn agregar_suscripcion(&mut self, suscripcion: Suscripcion) {
        self.suscripciones.push(suscripcion);
    }

    fn crear_usuario_con_suscripcion(
        &mut self,
        fecha: String,
        username: String,
        email: String,
        tipo_suscripcion: TipoDeSuscripcion,
        medio_de_pago: MedioDePago,
    ) -> Result<bool,ErrorCustom> {
        if username.is_empty()
            || email.is_empty()
            || self
                .usuarios
                .iter()
                .any(|u| u.email == email || u.username == username)
        {
            return Err(ErrorCustom("Faltan campos requeridos o no validos".to_string()));
        }
        let new_user = Usuario::new(email, username, medio_de_pago);
        self.agregar_usuario(new_user.clone());
        let suscripcion = Suscripcion::new(fecha, tipo_suscripcion, new_user);
        self.agregar_suscripcion(suscripcion);
        return Ok(true);
    }

    fn upgrade_suscripcion(&mut self, email: &String) -> Result<bool,ErrorCustom> {
        if let Some(suscripcion) = self
            .suscripciones
            .iter_mut()
            .find(|sus| sus.user.email == *email)
        {
            let nuevo_tipo = match suscripcion.tipo {
                TipoDeSuscripcion::Basic => TipoDeSuscripcion::Clasic,
                TipoDeSuscripcion::Clasic => TipoDeSuscripcion::Super,
                TipoDeSuscripcion::Super => return Err(ErrorCustom("No puedes mejorar su suscripcion porque esta en la mejor calidad".to_string())),
            };
            suscripcion.costo = nuevo_tipo.get_costo();
            suscripcion.duracion=  nuevo_tipo.get_duracion();
            suscripcion.tipo = nuevo_tipo;
            return Ok(true);
        }
        return Err(ErrorCustom("Usuario no encontrado, o no posee una suscripcion".to_string()));
    }
    fn downgrade_suscripcion(&mut self, email: &String) -> Result<bool,ErrorCustom> {
        if let Some(suscripcion) = self
            .suscripciones
            .iter_mut()
            .find(|sus| sus.user.email == *email)
        {
            let nuevo_tipo = match suscripcion.tipo {
                TipoDeSuscripcion::Basic => {
                    suscripcion.dar_de_baja();
                    return Ok(true);
                }
                TipoDeSuscripcion::Clasic => TipoDeSuscripcion::Basic,
                TipoDeSuscripcion::Super => TipoDeSuscripcion::Clasic,
            };
            suscripcion.costo = nuevo_tipo.get_costo();
            suscripcion.duracion=  nuevo_tipo.get_duracion();
            suscripcion.tipo = nuevo_tipo;
            return Ok(true);
        } else {
            return Err(ErrorCustom("Usuario no encontrado, o no posee una suscripcion".to_string()));
        }
    }
    fn cancelar_suscripcion_usuario(&mut self, user: &Usuario) -> Result<bool,ErrorCustom> {
        if let Some(suscripcion) = self
            .suscripciones
            .iter_mut()
            .find(|sus| sus.user.email == user.email)
        {
            if !suscripcion.estado{
                return Err(ErrorCustom("Su suscripcion no esta activa".to_string()))
            }
            suscripcion.dar_de_baja();
            Ok(true)
        } else {
            Err(ErrorCustom("Usuario no existente o no posee una suscripcion".to_string()))
        }
    }
    fn medio_de_pago_mas_usado_activo(&self) -> Result<MedioDePago,ErrorCustom> {
        let mut contador: HashMap<&MedioDePago, u32> = HashMap::new();
        self.suscripciones.iter().for_each(|sus| {
            if sus.estado {
                let entry = contador.entry(&sus.user.medio_de_pago).or_insert(0);
                *entry += 1;
            }
        });
        if let Some((key, _)) = contador.into_iter().max_by_key(|&(_, count)| count) {
            return Ok(key.clone());
        } else {
            return Err(ErrorCustom("No hay suscripciones activas o no hay suscripciones".to_string()));
        }
    }
    fn medio_de_pago_mas_usado_all_time(&self) -> Result<MedioDePago,ErrorCustom> {
        let mut contador: HashMap<&MedioDePago, u32> = HashMap::new();

        self.suscripciones.iter().for_each(|sus| {
            let entry = contador.entry(&sus.user.medio_de_pago).or_insert(0);
            *entry += 1;
        });
        if let Some((key, _)) = contador.into_iter().max_by_key(|&(_, count)| count) {
            return Ok(key.clone());
        } else {
            return Err(ErrorCustom("No hay suscripciones".to_string()));
        }
    }
    fn suscripcion_mas_contratada_activa(&self) ->Result<TipoDeSuscripcion,ErrorCustom> {
        let mut contador: HashMap<&TipoDeSuscripcion, u32> = HashMap::new();

        self.suscripciones.iter().for_each(|sus| {
            if sus.estado {
                let entry = contador.entry(&sus.tipo).or_insert(0);
                *entry += 1;
            }
        });
        if let Some((key, _)) = contador.into_iter().max_by_key(|&(_, count)| count) {
            return Ok(key.clone());
        } else {
            return Err(ErrorCustom("No hay suscripciones activas o no hay suscripciones".to_string()));
        }
    }
    fn suscripcion_mas_contratada_all_time(&self) -> Result<TipoDeSuscripcion,ErrorCustom> {
        let mut contador: HashMap<&TipoDeSuscripcion, u32> = HashMap::new();

        self.suscripciones.iter().for_each(|sus| {
            let entry = contador.entry(&sus.tipo).or_insert(0);
            *entry += 1;
        });
        if let Some((key, _)) = contador.into_iter().max_by_key(|&(_, count)| count) {
            return Ok(key.clone());
        } else {
            return Err(ErrorCustom("No hay suscripciones".to_string()));
        }
    }
    fn default() -> StreamingRust {
        let mut plataforma = StreamingRust::new();
        let _ =plataforma.crear_usuario_con_suscripcion(
            "2024-03-20".to_string(),
            "user1".to_string(),
            "user1@gmail.com".to_string(),
            TipoDeSuscripcion::Basic,
            MedioDePago::Efectivo,
        );
        let _ =plataforma.crear_usuario_con_suscripcion(
            "2024-03-20".to_string(),
            "user6".to_string(),
            "user6@gmail.com".to_string(),
            TipoDeSuscripcion::Basic,
            MedioDePago::Efectivo,
        );
        let _ =plataforma.crear_usuario_con_suscripcion(
            "2024-03-21".to_string(),
            "user2".to_string(),
            "user2@gmail.com".to_string(),
            TipoDeSuscripcion::Clasic,
            MedioDePago::Cripto(Cripto {
                direccion_billetera: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
                id_transaccion: "abc123".to_string(),
                tipo_cripto: "Bitcoin".to_string(),
            }),
        );
        let _ =plataforma.crear_usuario_con_suscripcion(
            "2024-03-22".to_string(),
            "user3".to_string(),
            "user3@gmail.com".to_string(),
            TipoDeSuscripcion::Super,
            MedioDePago::TransferenciaBancaria(TransferenciaBancaria {
                numero_cuenta: "12345678".to_string(),
                clave_interbancaria: "CL123456".to_string(),
                nombre_banco: "Banco Rust".to_string(),
                nombre_tiular: "User Three".to_string(),
                numero_referencia: "REF123456".to_string(),
            }),
        );
        let _ =plataforma.crear_usuario_con_suscripcion(
            "2024-03-23".to_string(),
            "user4".to_string(),
            "user4@gmail.com".to_string(),
            TipoDeSuscripcion::Basic,
            MedioDePago::TarjetaDeCredito(TarjetaDeCredito {
                numero_tarjeta: "4111111111111111".to_string(),
                fecha_vencimiento: "12/25".to_string(),
                cvv: "123".to_string(),
                nombre_titular: "User Four".to_string(),
            }),
        );
        let _ =plataforma.crear_usuario_con_suscripcion(
            "2024-03-24".to_string(),
            "user5".to_string(),
            "user5@gmail.com".to_string(),
            TipoDeSuscripcion::Clasic,
            MedioDePago::MercadoPago(MercadoPago {
                email: "user5@mercadopago.com".to_string(),
                id_transaccion: "mp123456".to_string(),
            }),
        );
        plataforma
    }
}
#[test]
    fn test_cancelar_suscripcion(){
        let mut plataforma =StreamingRust::default();

        if let Some(suscripcion) =plataforma.suscripciones.iter_mut().find(|s| s.user.email == "user1@gmail.com".to_string()){
            suscripcion.estado = false;
            
        }
    }
#[test]
    fn test_medio_de_pago_mas_usado_activo() {
        let plataforma = StreamingRust::default();

        // Verificar que el medio de pago más usado activo sea el esperado
        assert!(plataforma.medio_de_pago_mas_usado_activo().is_ok_and(|m| m == MedioDePago::Efectivo));
    }

    #[test]
    fn test_medio_de_pago_mas_usado_all_time() {
        let plataforma = StreamingRust::default();

        // Verificar que el medio de pago más usado de todos los tiempos sea el esperado
        assert!(plataforma.medio_de_pago_mas_usado_all_time().is_ok_and(|m| m == MedioDePago::Efectivo));
    }

    #[test]
    fn test_suscripcion_mas_contratada_activa() {
        let plataforma = StreamingRust::default();

        // Verificar que la suscripción más contratada activa sea la esperada
        assert!(plataforma.suscripcion_mas_contratada_activa().is_ok_and(|t| t == TipoDeSuscripcion::Basic));

    }

    #[test]
    fn test_suscripcion_mas_contratada_all_time() {
        let plataforma = StreamingRust::default();

        // Verificar que la suscripción más contratada de todos los tiempos sea la esperada
        assert!(plataforma.suscripcion_mas_contratada_all_time().is_ok_and(|t| t == TipoDeSuscripcion::Basic));
    }

    #[test]
    fn test_upgrade_suscripcion() {
        let mut plataforma = StreamingRust::default();
        
        // Upgrade de una suscripción
        let email = "user1@gmail.com".to_string();
        assert!(plataforma.upgrade_suscripcion(&email).is_ok());
        let suscripcion = plataforma
            .suscripciones
            .iter()
            .find(|sus| sus.user.email == email && sus.estado)
            .unwrap();
        assert_eq!(suscripcion.tipo, TipoDeSuscripcion::Clasic);
    }
    #[test]
    fn test_downgrade_suscripcion() {
        let mut plataforma = StreamingRust::default();
        let email = "user1@gmail.com".to_string();
        // Downgrade de una suscripción
        assert!(plataforma.downgrade_suscripcion(&email).is_ok());
        let suscripcion = plataforma
            .suscripciones
            .iter()
            .find(|sus| sus.user.email == email)
            .unwrap();
        assert_eq!(suscripcion.estado, false);
    }
