use std::collections::HashMap;

use crate::ciudad::{Ciudad, Conexion};

pub struct GrafoRutas {
    ciudades: Vec<Ciudad>,
    indice_ciudades: HashMap<String, usize>,
    adyacencia: Vec<Vec<Conexion>>,
}

impl GrafoRutas {
    pub fn new() -> Self {
        Self {
            ciudades: Vec::new(),
            indice_ciudades: HashMap::new(),
            adyacencia: Vec::new(),
        }
    }

    pub fn agregar_ciudad(&mut self, ciudad: Ciudad) -> Result<(), String> {
        if self.indice_ciudades.contains_key(&ciudad.nombre) {
            return Err(format!("La ciudad '{}' ya existe.", ciudad.nombre));
        }

        let indice = self.ciudades.len();

        self.indice_ciudades
            .insert(ciudad.nombre.clone(), indice);

        self.ciudades.push(ciudad);
        self.adyacencia.push(Vec::new());

        Ok(())
    }

    pub fn agregar_ruta(
        &mut self,
        origen: &str,
        destino: &str,
        distancia_km: u32,
    ) -> Result<(), String> {
        let indice_origen = self
            .indice_de(origen)
            .ok_or(format!("La ciudad '{}' no existe.", origen))?;

        let indice_destino = self
            .indice_de(destino)
            .ok_or(format!("La ciudad '{}' no existe.", destino))?;

        // Como es un grafo no dirigido, se agrega la ruta en ambos sentidos.
        self.adyacencia[indice_origen]
            .push(Conexion::new(indice_destino, distancia_km));

        self.adyacencia[indice_destino]
            .push(Conexion::new(indice_origen, distancia_km));

        Ok(())
    }

    pub fn indice_de(&self, nombre: &str) -> Option<usize> {
        self.indice_ciudades.get(nombre).copied()
    }

    pub fn ciudad_por_indice(&self, indice: usize) -> Option<&Ciudad> {
        self.ciudades.get(indice)
    }

    pub fn vecinos(&self, indice: usize) -> Option<&Vec<Conexion>> {
        self.adyacencia.get(indice)
    }

    pub fn distancia_entre(&self, origen: usize, destino: usize) -> Option<u32> {
        self.adyacencia
            .get(origen)?
            .iter()
            .find(|conexion| conexion.destino == destino)
            .map(|conexion| conexion.distancia_km)
    }

    pub fn cantidad_ciudades(&self) -> usize {
        self.ciudades.len()
    }

    pub fn mostrar_grafo(&self) {
        println!("--- Grafo de rutas entre ciudades ---");

        for (indice, ciudad) in self.ciudades.iter().enumerate() {
            print!("{} ({}) -> ", ciudad.nombre, ciudad.departamento);

            if let Some(vecinos) = self.vecinos(indice) {
                for conexion in vecinos {
                    if let Some(destino) = self.ciudad_por_indice(conexion.destino) {
                        print!("{} [{} km]  ", destino.nombre, conexion.distancia_km);
                    }
                }
            }

            println!();
        }
    }
}