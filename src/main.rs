mod ciudad;
mod mapa_rutas;
mod busqueda;

use ciudad::Ciudad;
use mapa_rutas::GrafoRutas;
use busqueda::bfs_ruta_mas_corta;

fn main() {
    let mut mapa = GrafoRutas::new();

    mapa.agregar_ciudad(Ciudad::new("Santa Ana", "Santa Ana")).unwrap();
    mapa.agregar_ciudad(Ciudad::new("Ahuachapan", "Ahuachapan")).unwrap();
    mapa.agregar_ciudad(Ciudad::new("San Salvador", "San Salvador")).unwrap();
    mapa.agregar_ciudad(Ciudad::new("La Libertad", "La Libertad")).unwrap();
    mapa.agregar_ciudad(Ciudad::new("Usulutan", "Usulutan")).unwrap();
    mapa.agregar_ciudad(Ciudad::new("San Miguel", "San Miguel")).unwrap();

    mapa.agregar_ruta("Santa Ana", "Ahuachapan", 35).unwrap();
    mapa.agregar_ruta("Santa Ana", "San Salvador", 65).unwrap();
    mapa.agregar_ruta("San Salvador", "La Libertad", 32).unwrap();
    mapa.agregar_ruta("San Salvador", "Usulutan", 110).unwrap();
    mapa.agregar_ruta("La Libertad", "Usulutan", 120).unwrap();
    mapa.agregar_ruta("Usulutan", "San Miguel", 45).unwrap();

    mapa.mostrar_grafo();

    let origen = "Santa Ana";
    let destino = "San Miguel";

    println!();
    println!("--- Buscando ruta con BFS ---");
    println!("Origen: {}", origen);
    println!("Destino: {}", destino);
    println!();

    match bfs_ruta_mas_corta(&mapa, origen, destino) {
        Some(ruta) => ruta.mostrar(),
        None => println!("No se encontró una ruta entre {} y {}.", origen, destino),
    }
}
