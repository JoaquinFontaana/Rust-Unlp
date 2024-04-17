pub fn sumar_arreglos(arreglo1: &[f32; 3], arreglo2: &[f32; 3]) -> [f32; 3] {
    let mut suma_arreglos: [f32; 3] = [0.0; 3];
    for i in 0..arreglo1.len() {
        suma_arreglos[i] = arreglo1[i] + arreglo2[i];
        suma_arreglos[i] = (suma_arreglos[i] * 10.0).round() / 10.0;
    }
    suma_arreglos
}

#[test]
fn test_sumar_arreglos() {
    let arreglo1: [f32; 3] = [2.3, 1.0, 0.5];
    let arreglo2: [f32; 3] = [5.3, 0.0, 0.5];
    assert_eq!([7.6, 1.0, 1.0], sumar_arreglos(&arreglo1, &arreglo2));
}
