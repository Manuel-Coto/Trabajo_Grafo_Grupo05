#[derive(Clone, Debug)]
pub struct Ciudad {
    pub nombre: String,
    pub departamento: String,
}

impl Ciudad {
    pub fn new(nombre: &str, departamento: &str) -> Self {
        Self {
            nombre: nombre.to_string(),
            departamento: departamento.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Conexion {
    pub destino: usize,
    pub distancia_km: u32,
}

impl Conexion {
    pub fn new(destino: usize, distancia_km: u32) -> Self {
        Self {
            destino,
            distancia_km,
        }
    }
}