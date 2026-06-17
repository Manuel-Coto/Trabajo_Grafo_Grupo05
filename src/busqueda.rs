use std::collections::VecDeque;

use crate::ciudad::RutaEncontrada;
use crate::mapa_rutas::GrafoRutas;

pub fn bfs_ruta_mas_corta(
    grafo: &GrafoRutas,
    origen: &str,
    destino: &str,
) -> Option<RutaEncontrada> {
    let origen_idx = grafo.indice_de(origen)?;
    let destino_idx = grafo.indice_de(destino)?;

    let cantidad = grafo.cantidad_ciudades();

    let mut visitado = vec![false; cantidad];
    let mut anterior: Vec<Option<usize>> = vec![None; cantidad];
    let mut cola = VecDeque::new();

    visitado[origen_idx] = true;
    cola.push_back(origen_idx);

    while let Some(actual) = cola.pop_front() {
        if actual == destino_idx {
            break;
        }

        if let Some(vecinos) = grafo.vecinos(actual) {
            for conexion in vecinos {
                let vecino = conexion.destino;

                if !visitado[vecino] {
                    visitado[vecino] = true;
                    anterior[vecino] = Some(actual);
                    cola.push_back(vecino);
                }
            }
        }
    }

    if !visitado[destino_idx] {
        return None;
    }

    let mut ruta_indices = Vec::new();
    let mut actual = destino_idx;

    ruta_indices.push(actual);

    while let Some(previo) = anterior[actual] {
        ruta_indices.push(previo);
        actual = previo;
    }

    ruta_indices.reverse();

    let mut nombres_ciudades = Vec::new();

    for indice in &ruta_indices {
        if let Some(ciudad) = grafo.ciudad_por_indice(*indice) {
            nombres_ciudades.push(ciudad.nombre.clone());
        }
    }

    let mut distancia_total = 0;

    for par in ruta_indices.windows(2) {
        if let Some(distancia) = grafo.distancia_entre(par[0], par[1]) {
            distancia_total += distancia;
        }
    }

    Some(RutaEncontrada {
        ciudades: nombres_ciudades,
        saltos: ruta_indices.len().saturating_sub(1),
        distancia_total,
    })
}