#![allow(dead_code)]
use std::{collections::HashMap, hash::Hash};
use rand::Rng;
use serde::ser::SerializeMap;
use std::fmt::Display;
use serde::{Serialize,Serializer, Deserialize};
use serde_json::Error as SerdeError;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Clone, Eq, PartialEq, Hash, Debug,Deserialize)]
struct CriptoMoneda {
    nombre: String,
    prefijo: String,
    blockchains: Vec<Blockchain>,
}
impl Serialize for CriptoMoneda {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializar solo el nombre de la criptomoneda
        self.nombre.serialize(serializer)
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Serialize, Deserialize,Clone)]
struct Usuario {
    email: String,
    dni: String,
    nombre: String,
    apellido: String,
    verificacion: bool,
    #[serde(serialize_with = "serialize_balance_cripto")]
    balance_cripto: HashMap<CriptoMoneda, f64>,
    balance_fiat: f64,
}

fn serialize_balance_cripto<S>(balance: &HashMap<CriptoMoneda, f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(balance.len()))?;

    for (cripto_moneda, cantidad) in balance {
        // Serializar la clave (nombre de la criptomoneda)
        let key = match serde_json::to_string(cripto_moneda) {
            Ok(key) => key,
            Err(error) => return Err(serde::ser::Error::custom(error)),
        };
        // Serializar el valor (cantidad)
        let value = match serde_json::to_value(cantidad) {
            Ok(value) => value,
            Err(error) => return Err(serde::ser::Error::custom(error)),
        };
        map.serialize_entry(&key, &value)?;
    }

    map.end()
}
#[derive(Serialize, Deserialize)]
struct XYZ {
    usuarios: Vec<Usuario>,
    cripto_monedas: Vec<CriptoMoneda>,
    sistema_cotizaciones: CotizacionCriptos,
    transacciones: Vec<Transaccion>,
}

#[derive(Serialize, Deserialize)]
struct Transaccion {
    fecha: String,
    tipo: TipoTransaccion,
    monto: f64,
    usuario: Usuario,
}

#[derive(Serialize, Deserialize)]
enum TipoTransaccion {
    Fiat(TipoOperacionFiat),
    CriptoMoneda(TipoOperacionCripto),
}

#[derive(Serialize, Deserialize)]
enum TipoOperacionFiat {
    Ingreso,
    Retiro(MedioDePago),
}

#[derive(Serialize, Deserialize)]
enum TipoOperacionCripto {
    Venta(VentaCompraCripto),
    Compra(VentaCompraCripto),
    Retiro(RetiroCripto),
    Recepcion(RecepcionCripto),
}

#[derive(Serialize, Deserialize)]
enum MedioDePago {
    MercadoPago,
    TransferenciaBancaria,
}

#[derive(Serialize, Deserialize)]
struct VentaCompraCripto {
    cotizacion: f64,
    cripto: CriptoMoneda,
}

#[derive(Serialize, Deserialize)]
struct RetiroCripto {
    blockchain: Blockchain,
    hash: String,
    cripto: CriptoMoneda,
    cotizacion: f64,
}

#[derive(Serialize, Deserialize)]
struct RecepcionCripto {
    blockchain: Blockchain,
    cotizacion: f64,
    cripto: CriptoMoneda,
}

#[derive(Serialize, Deserialize)]
struct CotizacionCriptos {
    cotizaciones: HashMap<CriptoMoneda, f64>,
}

impl CotizacionCriptos{
    fn new(cotizaciones:HashMap<CriptoMoneda,f64>) -> CotizacionCriptos{
        let sistema = CotizacionCriptos{
            cotizaciones
        };
        sistema
    }
}
impl VentaCompraCripto{
    fn new(cotizacion:f64,cripto:CriptoMoneda) -> VentaCompraCripto{
        let transaccion = VentaCompraCripto{
            cotizacion,
            cripto
        };
        transaccion
    }
}
impl RecepcionCripto{
    fn new(blockchain:Blockchain,cripto:CriptoMoneda,cotizacion:f64) -> RecepcionCripto{
        let transaccion = RecepcionCripto{
            blockchain,
            cotizacion,
            cripto
        };
        transaccion
    }
}
impl RetiroCripto{
    fn new(
        blockchain:Blockchain,
        hash:String,
        cripto:CriptoMoneda,
        cotizacion:f64) 
        -> RetiroCripto{
        let transaccion = RetiroCripto{
            blockchain,
            hash,
            cripto,
            cotizacion,
        };
        transaccion
    }
}
impl Usuario{
    fn new(email:String,nombre:String,apellido:String,dni:String,verificacion:bool) -> Usuario{
        let balance_cripto:HashMap<CriptoMoneda,f64> = HashMap::new();
        let balance_fiat: f64 = 0.0;
        let user = Usuario{
            email,
            dni,
            apellido,
            nombre,
            verificacion,
            balance_cripto,
            balance_fiat
        };
        user
    }
    fn default() -> Usuario{
        Usuario::new(
            String::from("test@example.com"),
            String::from("John"),
            String::from("Doe"),
            String::from("123456789"),
            true,
        )
    }
    fn default_sin_verficacion() -> Usuario{
        Usuario::new(
            String::from("test@example.com"),
            String::from("John"),
            String::from("Doe"),
            String::from("123456789"),
            false,
        )
    }
}
impl Transaccion{
    fn new(fecha:String,tipo:TipoTransaccion,monto:f64,usuario:Usuario) -> Transaccion{
        let transaccion = Transaccion{
            monto,
            fecha,
            tipo,
            usuario
        };
        transaccion
    }
}
impl Blockchain{
    fn generar_hash(&self) ->String{
        let mut rng = rand::thread_rng();
        let numero_aleatorio: u32 = rng.gen_range(0..10000);
        let input = format!("{}{}", numero_aleatorio, self.nombre);
        input
    }
}
#[derive(Debug)]
struct ErrorCustom(String);
impl Display for ErrorCustom{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
impl From<SerdeError> for ErrorCustom {
    fn from(err: SerdeError) -> ErrorCustom {
        ErrorCustom(err.to_string())
    }
}
impl<'a> XYZ{
    fn new(cripto_monedas: Vec<CriptoMoneda>, sistema_cotizaciones: CotizacionCriptos) -> XYZ {
        let usuarios: Vec<Usuario> = Vec::new();
        let transacciones: Vec<Transaccion> = Vec::new();
        let plataforma = XYZ {
            cripto_monedas,
            sistema_cotizaciones,
            usuarios,
            transacciones,
        };
        plataforma
    }

    fn crear_usuario(&mut self, dni: String, email: String, nombre: String, apellido: String, verificacion: bool) -> Result<bool, ErrorCustom> {
        // Verificacion de dni suponiendo que la plataforma solo opera en Argentina, o la implementacion es para Argentina
        if dni.len() != 9 || email.len() < 1 || nombre.len() < 1 || apellido.len() < 1 {
            return Err(ErrorCustom("Longitud de los campos insuficiente".to_string()));
        }
        if self.usuarios.iter().any(|u| u.dni == dni) {
            return Err(ErrorCustom("DNI ya registrado".to_string()));
        }
        let new_user = Usuario::new(email, nombre, apellido, dni, verificacion);
        self.usuarios.push(new_user);
        self.guardar_datos("usuarios.json", "transacciones.json")?; // Guardar datos
        Ok(true)
    }

    fn ingresar_fiat(&mut self, user: &Usuario, monto: f64, fecha: String) -> Result<bool, ErrorCustom> {
        if monto <= 0.0 {
            return Err(ErrorCustom("Monto invalido. Debe ser mayor a 0.".to_string()));
        }
        if let Some(usuario_base_datos) = self.usuarios.iter_mut().find(|u| u.dni == user.dni && u.verificacion) {
            usuario_base_datos.balance_fiat += monto;
            let new_transaccion = Transaccion::new(fecha, TipoTransaccion::Fiat(TipoOperacionFiat::Ingreso), monto, usuario_base_datos.clone());
            self.transacciones.push(new_transaccion); // Agregar la transacción al registro
            self.guardar_datos("usuarios.json", "transacciones.json")?; // Guardar datos
            Ok(true)
        } else {
            Err(ErrorCustom("Usuario no encontrado o no verificado".to_string()))
        }
    }

    fn comprar_cripto_moneda(&mut self, fecha: String, user: &Usuario, cripto: CriptoMoneda, monto_fiat: f64) -> Result<bool, ErrorCustom> {
        // Buscar al usuario en la base de datos y verificar que esté verificado
        if let Some(usuario_base_datos) = self.usuarios.iter_mut().find(|u| u.dni == user.dni && u.verificacion) {
            // Verificar que el usuario tiene suficiente saldo fiat
            if usuario_base_datos.balance_fiat < monto_fiat {
                return Err(ErrorCustom("Balance de fiat insuficiente".to_string()));
            }
            // Buscar la cotización de la criptomoneda
            if let Some((_, cotizacion)) = self.sistema_cotizaciones.cotizaciones.iter().find(|(c, _)| c.nombre == cripto.nombre) {
                // Calcular el monto a acreditar en criptomonedas
                let monto_acreditar_cripto = monto_fiat / cotizacion;
                // Actualizar el balance de criptomonedas del usuario
                let balance_anterior = usuario_base_datos.balance_cripto.entry(cripto.clone()).or_insert(0.0);
                *balance_anterior += monto_acreditar_cripto;
                // Reducir el balance fiat del usuario
                usuario_base_datos.balance_fiat -= monto_fiat;
                // Crear y registrar la transacción
                let transaccion_cripto = VentaCompraCripto::new(*cotizacion, cripto.clone());
                let new_transaccion = Transaccion::new(fecha, TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Compra(transaccion_cripto)), monto_acreditar_cripto, usuario_base_datos.clone());
                self.transacciones.push(new_transaccion);
                match self.guardar_datos("usuarios.json", "transacciones.json"){
                    Ok(_) => (),
                    Err(e) => return Err(ErrorCustom(e.to_string()))
                } // Guardar datos
                return Ok(true);
            } else {
                return Err(ErrorCustom("No se pudo obtener la cotización de la criptomoneda".to_string()));
            }
        } else {
            return Err(ErrorCustom("Usuario no encontrado o no verificado".to_string()));
        }
    }

    fn vender_cripto_moneda(&mut self, fecha: String, user: &Usuario, cripto: CriptoMoneda, monto_cripto: f64) -> Result<bool, ErrorCustom> {
        if let Some(usuario_base_datos) = self.usuarios.iter_mut().find(|u| u.dni == user.dni && u.verificacion) {
            if let Some((_, balance_actual_cripto)) = usuario_base_datos.balance_cripto.iter_mut().find(|(cripto_base_datos, _)| cripto_base_datos.nombre == cripto.nombre) {
                if *balance_actual_cripto < monto_cripto {
                    return Err(ErrorCustom("Balance de la cripto moneda insuficiente".to_string()));
                    
                } else if let Some((_, cotizacion)) = self.sistema_cotizaciones.cotizaciones.iter().find(|(c, _)| c.nombre == cripto.nombre) {
                    let monto_acreditar_fiat = monto_cripto * cotizacion;
                    usuario_base_datos.balance_fiat += monto_acreditar_fiat;
                    *balance_actual_cripto -= monto_cripto;
                    let transaccion_cripto = VentaCompraCripto::new(*cotizacion, cripto.clone());
                    let new_transaccion = Transaccion::new(fecha, TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Venta(transaccion_cripto)), monto_cripto, usuario_base_datos.clone());
                    self.transacciones.push(new_transaccion);
                    self.guardar_datos("usuarios.json", "transacciones.json")?; // Guardar datos
                    return Ok(true);
                }
                return Err(ErrorCustom("No se pudo obtener la cotización de la criptomoneda".to_string()));
            }
            return Err(ErrorCustom("No se encontró la cripto moneda".to_string()));
        }
        return Err(ErrorCustom("Usuario no encontrado o no verificado".to_string()));
    }

    fn retirar_cripto_a_blockchain(&mut self, monto_cripto: f64, blockchain: Blockchain, cripto: CriptoMoneda, user: &Usuario, fecha: String) -> Result<bool, ErrorCustom> {
        if let Some(user_base_datos) = self.usuarios.iter_mut().find(|u| u.dni == user.dni && u.verificacion) {
            if let Some((cripto_base_datos, balance_actual_cripto)) = user_base_datos.balance_cripto.iter_mut().find(|(c, _)| c.nombre == cripto.nombre) {
                if *balance_actual_cripto < monto_cripto {
                    return Err(ErrorCustom("Balance insuficiente".to_string()));
                }
                if let Some(blockchain_base_datos) = cripto_base_datos.blockchains.iter().find(|b| b.nombre == blockchain.nombre) {
                    if let Some((_, cotizacion)) = self.sistema_cotizaciones.cotizaciones.iter().find(|(c, _)| c.nombre == cripto_base_datos.nombre) {
                        *balance_actual_cripto -= monto_cripto;
                        let info_transaccion = RetiroCripto::new(blockchain_base_datos.clone(), blockchain_base_datos.generar_hash(), cripto, *cotizacion);
                        let new_transaccion = Transaccion::new(fecha, TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Retiro(info_transaccion)), monto_cripto, user_base_datos.clone());
                        self.transacciones.push(new_transaccion);
                        self.guardar_datos("usuarios.json", "transacciones.json")?; // Guardar datos
                        return Ok(true);
                    }
                    return Err(ErrorCustom("Cotización de la cripto moneda no disponible".to_string()));
                }
                return Err(ErrorCustom("BlockChain no disponible en la cripto moneda".to_string()));
            }
            return Err(ErrorCustom("No se encontró la cripto moneda en tu balance".to_string()));
        }
        return Err(ErrorCustom("Usuario no encontrado o no verificado".to_string()));
    }

    fn recibir_cripto_desde_blockchain(&mut self, monto_cripto: f64, blockchain: Blockchain, cripto: CriptoMoneda, user: &Usuario, fecha: String) -> Result<bool, ErrorCustom> {
        if let Some(user_base_datos) = self.usuarios.iter_mut().find(|u| u.dni == user.dni && u.verificacion) {
            if let Some(cripto_base_datos) = self.cripto_monedas.iter().find(|c| c.nombre == cripto.nombre) {
                if let Some(blockchain_base_datos) = cripto_base_datos.blockchains.iter().find(|b| b.nombre == blockchain.nombre) {
                    if let Some((_, cotizacion)) = self.sistema_cotizaciones.cotizaciones.iter().find(|(c, _)| c.nombre == cripto_base_datos.nombre) {
                        let balance_actual = user_base_datos.balance_cripto.entry(cripto_base_datos.clone()).or_insert(0.0);
                        *balance_actual += monto_cripto;
                        let info_transaccion = RecepcionCripto::new(blockchain_base_datos.clone(), cripto, *cotizacion);
                        let new_transaccion = Transaccion::new(fecha, TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Recepcion(info_transaccion)), monto_cripto, user_base_datos.clone());
                        self.transacciones.push(new_transaccion);
                        self.guardar_datos("usuarios.json", "transacciones.json"); // Guardar datos
                        return Ok(true);
                    }
                    return Err(ErrorCustom("Cotización de la cripto moneda no disponible".to_string()));
                }
                return Err(ErrorCustom("BlockChain no disponible en la cripto moneda".to_string()));
            }
            return Err(ErrorCustom("No se encontró la cripto moneda".to_string()));
        }
        return Err(ErrorCustom("Usuario no encontrado o no verificado".to_string()));
    }

    fn retirar_fiat(&mut self, user: &Usuario, monto: f64, fecha: String, medio: MedioDePago) -> Result<bool, ErrorCustom> {
        if let Some(usuario_base_datos) = self.usuarios.iter_mut().find(|u| u.dni == user.dni && u.verificacion) {
            if usuario_base_datos.balance_fiat < monto {
                return Err(ErrorCustom("Balance insuficiente".to_string()));
            }
            usuario_base_datos.balance_fiat -= monto;
            let new_transaccion = Transaccion::new(fecha, TipoTransaccion::Fiat(TipoOperacionFiat::Retiro(medio)), monto, usuario_base_datos.clone());
            self.transacciones.push(new_transaccion);
            self.guardar_datos("usuarios.json", "transacciones.json")?; // Guardar datos
            return Ok(true);
        }
        Err(ErrorCustom("Usuario no encontrado o no verificado".to_string()))
    }

    fn cripto_cantidad_mas_vendida(&self) -> Result<CriptoMoneda, ErrorCustom> {
        let mut contador_ventas: HashMap<CriptoMoneda, u32> = HashMap::new();
        if self.transacciones.len() < 1 {
            return Err(ErrorCustom("No hay transacciones".to_string()));
        }
        self.transacciones.iter().for_each(|transaccion| {
            if let TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Venta(ref venta_info)) = transaccion.tipo {
                let cant = contador_ventas.entry(venta_info.cripto.clone()).or_insert(0);
                *cant += 1;
            }
        });
        let mut max_cripto = self.cripto_monedas[0].clone();
        let mut max_cant: u32 = 0;
        contador_ventas.into_iter().for_each(|(cripto, cant)| {
            if cant > max_cant {
                max_cripto = cripto;
                max_cant = cant;
            }
        });
        Ok(max_cripto)
    }

    fn cripto_cantidad_mas_comprada(&self) -> Result<CriptoMoneda, ErrorCustom> {
        let mut contador_compra: HashMap<CriptoMoneda, u32> = HashMap::new();
        if self.transacciones.is_empty() {
            return Err(ErrorCustom("No hay transacciones".to_string()));
        }
        // Contar la cantidad de compras para cada criptomoneda
        self.transacciones.iter().for_each(|transaccion| {
            if let TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Compra(ref compra_info)) = transaccion.tipo {
                let cant = contador_compra.entry(compra_info.cripto.clone()).or_insert(0);
                *cant += 1;
            }
        });
        // Buscar la criptomoneda con la cantidad máxima de compras
        let mut max_cripto = self.cripto_monedas[0].clone();
        let mut max_cant: u32 = 0;
        contador_compra.into_iter().for_each(|(cripto, cant)| {
            if cant > max_cant {
                max_cripto = cripto;
                max_cant = cant;
            }
        });
        Ok(max_cripto)
    }

    fn cripto_con_mayor_volumen_de_ventas(&self) -> Result<CriptoMoneda, ErrorCustom> {
        let mut contador_ventas: HashMap<CriptoMoneda, f64> = HashMap::new();
        if self.transacciones.is_empty() {
            return Err(ErrorCustom("No hay transacciones suficientes".to_string()));
        }
        self.transacciones.iter().for_each(|transaccion| {
            if let TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Venta(ref venta_info)) = transaccion.tipo {
                let cant = contador_ventas.entry(venta_info.cripto.clone()).or_insert(0.0);
                *cant += transaccion.monto;
            }
        });
        let mut cant_max = 0.0;
        let mut max_cripto: CriptoMoneda = self.cripto_monedas[0].clone();
        contador_ventas.into_iter().for_each(|(cripto, cant)| {
            if cant > cant_max {
                cant_max = cant;
                max_cripto = cripto;
            }
        });
        Ok(max_cripto)
    }

    fn cripto_con_mayor_volumen_de_compras(&self) -> Result<CriptoMoneda, ErrorCustom> {
        let mut contador_compras: HashMap<CriptoMoneda, f64> = HashMap::new();
        if self.transacciones.is_empty() {
            return Err(ErrorCustom("No hay transacciones suficientes".to_string()));
        }
        self.transacciones.iter().for_each(|transaccion| {
            if let TipoTransaccion::CriptoMoneda(TipoOperacionCripto::Compra(ref compra_info)) = transaccion.tipo {
                let cant = contador_compras.entry(compra_info.cripto.clone()).or_insert(0.0);
                *cant += transaccion.monto;
            }
        });
        let mut max_cripto = self.cripto_monedas[0].clone();
        let mut max_count = 0.0;
        contador_compras.into_iter().for_each(|(cripto, count)| {
            if count > max_count {
                max_cripto = cripto;
                max_count = count;
            }
        });
        Ok(max_cripto)
    }
        fn guardar_datos(&self, archivo_usuarios: &str, archivo_transacciones: &str) -> Result<(), SerdeError> {
            // Guardar usuarios
            let file_usuarios = File::create(archivo_usuarios).map_err(|e| SerdeError::io(e))?;
            let writer_usuarios = BufWriter::new(file_usuarios);
            serde_json::to_writer(writer_usuarios, &self.usuarios)?;
    
            // Guardar transacciones
            let file_transacciones = File::create(archivo_transacciones).map_err(|e| SerdeError::io(e))?;
            let writer_transacciones = BufWriter::new(file_transacciones);
            serde_json::to_writer(writer_transacciones, &self.transacciones)?;
    
            Ok(())
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_xyz() -> XYZ {
        // Definir datos de prueba
        let mut cotizaciones: HashMap<CriptoMoneda, f64> = HashMap::new();
        let cripto_moneda_1 = CriptoMoneda {
            nombre: String::from("Bitcoin"),
            prefijo: String::from("BTC"),
            blockchains: vec![
                Blockchain {
                    nombre: String::from("Blockchain1"),
                    prefijo: String::from("BC1"),
                },
                Blockchain {
                    nombre: String::from("Blockchain2"),
                    prefijo: String::from("BC2"),
                },
            ],
        };
        let cripto_moneda_2 = CriptoMoneda {
            nombre: String::from("Ethereum"),
            prefijo: String::from("ETH"),
            blockchains: vec![
                Blockchain {
                    nombre: String::from("Blockchain3"),
                    prefijo: String::from("BC3"),
                },
            ],
        };
        let cripto_moneda_3 = CriptoMoneda {
            nombre: String::from("Thether"),
            prefijo: String::from("USDT"),
            blockchains: vec![
                Blockchain {
                    nombre: String::from("Blockchain3"),
                    prefijo: String::from("BC3"),
                },
                Blockchain {
                    nombre: String::from("Blockchain1"),
                    prefijo: String::from("BC1"),
                },
            ],
        };
        let cripto_moneda_4 = CriptoMoneda {
            nombre: String::from("Thether"),
            prefijo: String::from("USDT"),
            blockchains: vec![
                Blockchain {
                    nombre: String::from("Blockchain3"),
                    prefijo: String::from("BC3"),
                },
                Blockchain {
                    nombre: String::from("Blockchain1"),
                    prefijo: String::from("BC1"),
                },
            ],
        };
        cotizaciones.insert(cripto_moneda_1.clone(), 50000.0);
        cotizaciones.insert(cripto_moneda_2.clone(), 2000.0);
        cotizaciones.insert(cripto_moneda_3.clone(), 1.0);
        cotizaciones.insert(cripto_moneda_4.clone(), 1.0);
        //crear sistema cotizaciones
        let sistema_cotizaciones = CotizacionCriptos::new(cotizaciones);
        let mut cripto_monedas: Vec<CriptoMoneda> = Vec::new();
        cripto_monedas.push(cripto_moneda_1);
        cripto_monedas.push(cripto_moneda_2);
        cripto_monedas.push(cripto_moneda_3);
        cripto_monedas.push(cripto_moneda_4);
        // Crear una instancia de XYZ para pruebas
        XYZ::new(cripto_monedas,sistema_cotizaciones)
    }
    #[test]
    fn test_guardar_datos() {
        let xyz = setup_xyz();

        // Intentar guardar los datos en un archivo
        let result = xyz.guardar_datos("usuarios.json", "transacciones.json");

        // Verificar que no se produjo un error
        assert!(result.is_ok(), "Error al guardar los datos: {:?}", result);
    }
    #[test]
    fn test_crear_usuario_campos_vacios(){
        let mut xyz = setup_xyz();
        assert_eq!(xyz.crear_usuario("".to_string(), "".to_string(), "".to_string(), "".to_string(),false).is_err(),true);
    }
    #[test]
    fn test_crear_usuario_dni_registrado(){
        let mut xyz = setup_xyz();
        assert_eq!(xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true).is_ok(),true);
        assert_eq!(xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true).is_ok(),false);
    }
    #[test]
    fn test_ingresar_fiat_1() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        assert_eq!(xyz.ingresar_fiat(&user, 1000.0, String::from("2024-05-30")).is_ok(), true);
        // Verificar que el saldo de fiat del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_fiat, 1000.0);
    }
    #[test]
    fn test_ingresar_fiat_sin_verificacion() {
        let mut xyz = setup_xyz();
        let user = Usuario::default_sin_verficacion();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), false);
        assert_eq!(xyz.ingresar_fiat(&user, 1000.0, String::from("2024-05-30")).is_err(), true);
        assert_eq!(xyz.usuarios[0].balance_fiat, 0.0);
    }
    #[test]
    fn test_ingresar_fiat_usuario_no_existente() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        assert_eq!(xyz.ingresar_fiat(&user, 1000.0, String::from("2024-05-30")).is_err(), true);
    }
    #[test]
    fn test_ingresar_fiat_monto_negativo() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        assert_eq!(xyz.ingresar_fiat(&user, -1000.0, String::from("2024-05-30")).is_err(), true);
    }
    #[test]
    fn test_comprar_cripto_moneda() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        //Ingresar fiat para la compra
        xyz.ingresar_fiat(&user, 100.0, String::from("2024-05-30"));
        let result = xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 100.0);
    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }
    assert!(result.is_ok(), "Falló la compra de criptomoneda con suficiente saldo fiat: {:?}", result);
        // Verificar que el saldo de fiat del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_fiat, 0.0);
        // Verificar que el saldo de criptomoneda del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_cripto.get(&xyz.cripto_monedas[0]).unwrap(), &0.002);
        // Intentar comprar criptomoneda sin saldo fiat suficiente
        assert_eq!(xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1000000.0).is_err(), true);
    }

    #[test]
    fn test_vender_cripto_moneda() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        //Ingresar fiat para la compra
        xyz.ingresar_fiat(&user, 50000.0, String::from("2024-05-30"));
        // Comprar criptomoneda para luego venderla
        assert_eq!(xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 50000.0).is_ok(), true);
        // Vender criptomoneda
        assert_eq!(xyz.vender_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1.0).is_ok(), true);
        // Verificar que el saldo de fiat del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_fiat, 50000.0);
        // Verificar que el saldo de criptomoneda del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_cripto.get(&xyz.cripto_monedas[0]).unwrap(), &0.0);
    }
    #[test]
    fn test_vender_cripto_moneda_saldo_insuficiente() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Ingresar fiat para la compra
        xyz.ingresar_fiat(&user, 100.0, String::from("2024-05-30"));
        // Comprar criptomoneda con suficiente saldo fiat
        xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1.0);
        // Intentar vender criptomoneda sin saldo suficiente
        assert_eq!(xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[0].clone(), 2.0).is_err(),true);
    }
    #[test]
    fn test_vender_cripto_moneda_usuario_no_verificado() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), false);
        assert_eq!(xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[0].clone(), 2.0).is_err(),true);
    }
    #[test]
    fn test_retirar_cripto_a_blockchain() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        //Ingresar fiat para la compra
        xyz.ingresar_fiat(&user, 50000.0, String::from("2024-05-30"));
        // Comprar criptomoneda para luego retirarla
        assert_eq!(xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 50000.0).is_ok(), true);
        // Retirar criptomoneda a BlockChain no perteneciente a la cripto moneda
        assert_eq!(xyz.retirar_cripto_a_blockchain(1.0, xyz.cripto_monedas[1].blockchains[0].clone(), xyz.cripto_monedas[0].clone(), &user, String::from("2024-05-30")).is_err(), true);
        // Retirar criptomoneda
        assert_eq!(xyz.retirar_cripto_a_blockchain(1.0, xyz.cripto_monedas[0].blockchains[0].clone(), xyz.cripto_monedas[0].clone(), &user, String::from("2024-05-30")).is_ok(), true);
        // Verificar que el saldo de criptomoneda del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_cripto.get(&xyz.cripto_monedas[0]).unwrap(), &0.0);
    }
    #[test]
    fn test_retirar_cripto_a_blockchain_saldo_insuficiente() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        //Ingresar fiat para la compra
        xyz.ingresar_fiat(&user, 50000.0, String::from("2024-05-30"));
        // Comprar criptomoneda para luego retirarla
        xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 500.0).is_ok();
        // Intentar retirar criptomoneda sin saldo suficiente
        assert!(xyz.retirar_cripto_a_blockchain(1.0, xyz.cripto_monedas[0].blockchains[0].clone(), xyz.cripto_monedas[0].clone(), &user, String::from("2024-05-31")).is_err());
    }
    fn test_retirar_cripto_a_blockchain_usuario_no_verificado() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), false);
        assert!(xyz.retirar_cripto_a_blockchain(1.0, xyz.cripto_monedas[0].blockchains[0].clone(), xyz.cripto_monedas[0].clone(), &user, String::from("2024-05-31")).is_err());
    }
    #[test]
    fn test_recibir_cripto_desde_blockchain() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Recibir criptomoneda desde blockchain
        assert_eq!(xyz.recibir_cripto_desde_blockchain(50.0, xyz.cripto_monedas[0].blockchains[0].clone(), xyz.cripto_monedas[0].clone(), &user, String::from("2024-05-30")).is_ok(), true);
        // Verificar que el saldo de criptomoneda del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_cripto.get(&xyz.cripto_monedas[0]).unwrap(), &50.0);
    }
    #[test]
    fn test_recibir_cripto_desde_blockchain_blockchain_no_valida() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Recibir criptomoneda desde blockchain
        assert_eq!(xyz.recibir_cripto_desde_blockchain(50.0, xyz.cripto_monedas[0].blockchains[0].clone(), xyz.cripto_monedas[1].clone(), &user, String::from("2024-05-30")).is_err(), true);
    }
    #[test]
    fn test_recibir_cripto_desde_blockchain_blockchain_usuario_no_valido() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), false);
        // Recibir criptomoneda desde blockchain
        assert_eq!(xyz.recibir_cripto_desde_blockchain(50.0, xyz.cripto_monedas[0].blockchains[0].clone(), xyz.cripto_monedas[0].clone(), &user, String::from("2024-05-30")).is_err(), true);
    }
    #[test]
    fn test_recibir_cripto_desde_blockchain_blockchain_cripto_no_valida() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        let cripto_moneda_no_valida = CriptoMoneda {
            nombre: String::from("Hola"),
            prefijo: String::from("TEST"),
            blockchains: vec![
                Blockchain {
                    nombre: String::from("Blockchain1"),
                    prefijo: String::from("BC1"),
                },
                Blockchain {
                    nombre: String::from("Blockchain2"),
                    prefijo: String::from("BC2"),
                },
            ],
        };
        // Recibir criptomoneda desde blockchain
        assert_eq!(xyz.recibir_cripto_desde_blockchain(50.0, xyz.cripto_monedas[0].blockchains[0].clone(), cripto_moneda_no_valida, &user, String::from("2024-05-30")).is_err(), true);
    }
    #[test]
    fn test_retirar_fiat() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Ingresar saldo fiat
        assert_eq!(xyz.ingresar_fiat(&user, 1000.0, String::from("2024-05-30")).is_ok(), true);
        // Retirar saldo fiat
        assert_eq!(xyz.retirar_fiat(&user, 500.0, String::from("2024-05-31"), MedioDePago::MercadoPago).is_ok(), true);
        // Verificar que el saldo de fiat del usuario se haya actualizado
        assert_eq!(xyz.usuarios[0].balance_fiat, 500.0);
    }
    #[test]
    fn test_retirar_fiat_usuario_no_valido() {
        let mut xyz = setup_xyz();
        let mut user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Ingresar saldo fiat
        user.dni = "22".to_string();
        assert_eq!(xyz.ingresar_fiat(&user, 1000.0, String::from("2024-05-30")).is_err(), true);
    }
    #[test]
    fn test_retirar_fiat_sin_balance() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Intentar retirar más saldo fiat del que se posee
        assert_eq!(xyz.retirar_fiat(&user, 1000.0, String::from("2024-05-31"), MedioDePago::MercadoPago).is_err(), true);
    }
    #[test]
    fn test_cripto_cantidad_mas_vendida() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Ingresar saldo fiat
        assert!(xyz.ingresar_fiat(&user, 10000000.0, String::from("2024-05-30")).is_ok());
        // Comprar y vender criptomonedas
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[2].clone(), 10000.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[3].clone(), 10.0);
        let _ =xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[2].clone(), 1.0);
        let _ =xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[3].clone(), 1.0);
        let _ =xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[3].clone(), 1.0);
        let _ =xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[3].clone(), 1.0);
        // Verificar que la criptomoneda más vendida sea la correcta
        assert!(xyz.cripto_cantidad_mas_vendida().is_ok_and(|c| c == xyz.cripto_monedas[3]));
    }
    #[test]
    fn test_cripto_cantidad_mas_vendida_sin_transacciones() {
        let xyz = setup_xyz();
        assert!(xyz.cripto_cantidad_mas_vendida().is_err());
    }
    #[test]
    fn test_cripto_cantidad_mas_comprada() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Ingresar saldo fiat
        assert_eq!(xyz.ingresar_fiat(&user, 10000000.0, String::from("2024-05-30")).is_ok(), true);
        // Comprar y vender criptomonedas
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1.0);
        let _= xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[0].clone(), 1.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[2].clone(), 1.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[1].clone(), 1.0);
        // Verificar que la criptomoneda más comprada sea la correcta
        assert!(xyz.cripto_cantidad_mas_comprada().is_ok_and(|cripto| cripto == xyz.cripto_monedas[0]));
    }
    #[test]
    fn test_cripto_cantidad_mas_comprada_sin_transacciones() {
        let xyz = setup_xyz();
        assert!(xyz.cripto_cantidad_mas_comprada().is_err());
    }
    #[test]
    fn test_cripto_volumen_mas_vendida() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);
        // Ingresar saldo fiat
        assert_eq!(xyz.ingresar_fiat(&user, 10000000.0, String::from("2024-05-30")).is_ok(), true);
        // Comprar y vender criptomonedas
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[2].clone(), 3.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-06-01"), &user, xyz.cripto_monedas[3].clone(), 3.0);
        let _ =xyz.vender_cripto_moneda(String::from("2024-05-31"), &user, xyz.cripto_monedas[2].clone(), 1.0);
        let _ =xyz.vender_cripto_moneda(String::from("2024-06-02"), &user, xyz.cripto_monedas[3].clone(), 3.0);
        // Verificar que la criptomoneda más vendida sea la correcta
        assert!(xyz.cripto_con_mayor_volumen_de_ventas().is_ok_and(|c| c == xyz.cripto_monedas[3]));
    }
    #[test]
    fn test_cripto_volumen_mas_comprada_sin_transaciones(){
        let xyz = setup_xyz();
        assert!(xyz.cripto_con_mayor_volumen_de_ventas().is_err())
    }
    #[test]
    fn test_cripto_volumen_mas_comprada() {
        let mut xyz = setup_xyz();
        let user = Usuario::default();
        let _ =xyz.crear_usuario(String::from("123456789"), String::from("test@example.com"), String::from("John"), String::from("Doe"), true);

        // Caso donde no hay transacciones suficientes
        assert_eq!(xyz.cripto_con_mayor_volumen_de_compras().is_err(), true);
    
        // Ingresar saldo fiat
        assert_eq!(xyz.ingresar_fiat(&user, 10000000.0, String::from("2024-05-30")).is_ok(), true);
    
        // Comprar y vender criptomonedas
        let _ =xyz.comprar_cripto_moneda(String::from("2024-05-30"), &user, xyz.cripto_monedas[2].clone(), 2.0);
        let _ =xyz.comprar_cripto_moneda(String::from("2024-06-01"), &user, xyz.cripto_monedas[3].clone(), 1.0);
    
        // Verificar que la criptomoneda más comprada sea la correcta
        assert!(xyz.cripto_con_mayor_volumen_de_compras().unwrap() == xyz.cripto_monedas[2]);
    
    }
}

   

