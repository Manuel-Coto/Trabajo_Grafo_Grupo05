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

#[derive(Debug)]
pub struct RutaEncontrada {
    pub ciudades: Vec<String>,
    pub saltos: usize,
    pub distancia_total: u32,
}

impl RutaEncontrada {
    pub fn mostrar(&self) {
        println!("Ruta encontrada:");
        println!("{}", self.ciudades.join(" -> "));
        println!("Cantidad de saltos: {}", self.saltos);
        println!("Distancia aproximada: {} km", self.distancia_total);
    }
}