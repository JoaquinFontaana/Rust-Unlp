pub fn longitud_cadenas(cadenas: &[String;3]) -> [i32;3]{
    let mut longitud: [i32;3] = [0;3];
    for i in 0..cadenas.len(){
        longitud[i] = cadenas[i].len() as i32;
    }
    longitud
}

#[test]
fn test_logitud_cadenas(){
    let cadenas = ["hola".to_string(),"pelotudo".to_string(),"jaja ".to_string()];
    assert_eq!([4,8,5],longitud_cadenas(&cadenas));
}   