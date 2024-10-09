use std::io;

#[derive(Debug)]
enum Estado {
    Pendiente,
    EnProgreso,
    Completada,
}

#[derive(Debug)]
struct Tarea {
    titulo: String,
    descripcion: String,
    estado: Estado,
}

fn main() {
    let mut tareas: Vec<Tarea> = Vec::new(); // Estructura que alojara todas nuestras tareas

    // Menu para nuestro programa de Todo List
    loop {
        println!("\n==== Gesto de Tareas ====");
        println!("1. Nueva Tarea");
        println!("2. Listar tareas");
        println!("3. Actualizar estado tarea");
        println!("4. Eliminar tarea");
        println!("5. Salir");

        //creamos nuestra eleccion (Opcion a utilzar)
        let mut eleccion: String = String::new();

        println!("Por favor, ingrese su selección:");
        io::stdin()
            .read_line(&mut eleccion)
            .expect("Error al leer dato de entrada");

        let eleccion: u32 = match eleccion.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                // usamos _ para ignorar el valor de error
                println!("Por favor ingrese una opcion valida");
                continue;
            }
        };
        match eleccion {
            // que haremos segun la tarea seleccionada
            1 => agregar_tarea(&mut tareas),
            2 => listar_tareas(&tareas),
            3 => actualizar_estado(&mut tareas),
            4 => eliminar_tarea(&mut tareas),
            5 => {
                println!("!Saliendo del gestor de tareas!");
                break;
            }
            _ => println!("Opcion no valida, favor intentar nuevamente!"), // _ => dentro de un match se utiliza como "catch-all" o patrón comodín
        }
    }
}

// Agregar Tarea
fn agregar_tarea(tareas: &mut Vec<Tarea>) {
    let mut titulo: String = String::new();
    let mut descripcion: String = String::new();

    // nuevo titulo
    println!("Ingrese el titulo de la tarea");
    io::stdin()
        .read_line(&mut titulo)
        .expect("Error al leer  titulo");

    // nueva descripcion
    println!("Ingrese la descripcion de la tarea");
    io::stdin()
        .read_line(&mut descripcion)
        .expect("Error al leer descripcion");

    let tarea = Tarea {
        titulo: titulo.trim().to_string(),
        descripcion: descripcion.trim().to_string(),
        estado: Estado::Pendiente,
    };

    tareas.push(tarea);
    println!("Tarea nueva agregada");
}

// Listar Tareas
fn listar_tareas(tareas: &Vec<Tarea>) {
    if tareas.is_empty() {
        println!("No hay tareas")
    } else {
        for (index, tarea) in tareas.iter().enumerate() {
            println!(
                // {:?}: visualizaremos de esta formaTarea
                // { titulo: "Aprender Rust", descripcion: "Repasar conceptos básicos de Rust", estado: Pendiente }
                // por usar #[derive(Debug)]
                "{}. {} - Estado: {:?}",
                index + 1,
                tarea.titulo,
                tarea.estado,
            );
        }
    }
}

// Actualizar estado
fn actualizar_estado(tareas: &mut Vec<Tarea>) {
    listar_tareas(tareas); // listamos tareas

    println!("Ingrese el numero de la tarea a actualizar");
    let mut indice: String = String::new();
    io::stdin()
        .read_line(&mut indice)
        .expect("Error al leer numero");

    let indice: usize = match indice.trim().parse::<usize>() {
        // queremos convertir la entrada en un tipo usize
        Ok(num) => num - 1,
        Err(_) => {
            println!("Numero de tarea no valido");
            return;
        }
    };

    if indice >= tareas.len() {
        println!("Numero de tareas fuera de rango");
        return;
    }

    println!("Seleccione el nuevo estado:");
    println!("1. Pendiente");
    println!("2. En Progreso");
    println!("3. Completada");

    let mut nuevo_estado = String::new();
    println!("Ingrese seleccion");
    io::stdin()
        .read_line(&mut nuevo_estado)
        .expect("Error al leer estado");

    let nuevo_estado = match nuevo_estado.trim().parse::<u32>() {
        Ok(1) => Estado::Pendiente,
        Ok(2) => Estado::EnProgreso,
        Ok(3) => Estado::Completada,
        _ => {
            println!("Estado no valido.");
            return;
        }
    };

    tareas[indice].estado = nuevo_estado;
    println!("Estado actualizado exitosamente")
}

// Eliminar tarea
fn eliminar_tarea(tareas: &mut Vec<Tarea>) {
    listar_tareas(&tareas);

    println!("Ingrese el numero de las tareas a eliminar");

    let mut indice: String = String::new();
    io::stdin()
        .read_line(&mut indice)
        .expect("Error al leer numero");

    let indice: usize = match indice.trim().parse::<usize>() {
        Ok(num) => num - 1,
        Err(_) => {
            println!("Numero de tareas no validas");
            return;
        }
    };

    if indice >= tareas.len(){
        println!("Numero de tareas fuera de rango");
        return;
    }

    tareas.remove(indice);
    println!("Tarea eliminada exitosamente");
}