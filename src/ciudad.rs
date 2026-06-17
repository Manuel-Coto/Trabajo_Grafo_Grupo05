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