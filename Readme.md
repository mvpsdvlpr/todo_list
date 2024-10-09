
# Gestor de Tareas en Rust
> English version 
[English version](README_en.md)  
Este es un proyecto básico de consola para la gestión de tareas (to-do list) en Rust. Permite agregar, listar, actualizar y eliminar tareas, aplicando varios conceptos fundamentales del lenguaje como structs, enums, y más.

## Características

- **Agregar una nueva tarea**: Los usuarios pueden agregar tareas con un título, una descripción y un estado inicial.
- **Listar tareas**: Muestra todas las tareas junto con su estado actual.
- **Actualizar el estado de una tarea**: Permite marcar una tarea como `Pendiente`, `En Progreso`, o `Completada`.
- **Eliminar una tarea**: Elimina una tarea de la lista.
- **Opcional**: Guardar y cargar las tareas desde un archivo.

## Conceptos aplicados

Este proyecto utiliza:
- **Variables, Structs y Enums**: Para representar las tareas y su estado.
- **Funciones**: Para gestionar las operaciones CRUD (Crear, Leer, Actualizar, Eliminar) sobre las tareas.
- **Option y match**: Para manejar tareas inexistentes de manera segura.
- **Ownership y Borrowing**: Para administrar correctamente los datos de las tareas y el acceso concurrente.

## Ejecución del proyecto

### Prerrequisitos

- Tener instalado Rust. Si no lo tienes, instálalo siguiendo las instrucciones en [rust-lang.org](https://www.rust-lang.org/).

### Instalación

1. Clona este repositorio en tu máquina local:
   ```bash
   git clone https://github.com/tu-usuario/gestor-tareas-rust.git
   cd gestor-tareas-rust
   ```

2. Compila y ejecuta el programa:
   ```bash
   cargo run
   ```

## Ejemplo de uso

### Agregar una tarea
Al iniciar el programa, el usuario puede agregar una tarea proporcionando un título y una descripción.

```rust
let tarea = Tarea {
    titulo: String::from("Aprender Rust"),
    descripcion: String::from("Repasar conceptos básicos de Rust"),
    estado: Estado::Pendiente,
};
```

### Listar tareas
El programa muestra todas las tareas almacenadas y su estado actual.

```bash
Tarea: "Aprender Rust", Estado: Pendiente
```

### Actualizar el estado de una tarea
El usuario puede cambiar el estado de una tarea a `En Progreso` o `Completada` según el progreso.

### Eliminar una tarea
El programa permite eliminar tareas de la lista de manera fácil.

## Futuras mejoras

- **Interacción de usuario**: Implementar un sistema de menú que permita al usuario realizar todas las acciones desde la consola.
- **Persistencia de datos**: Guardar y cargar las tareas desde un archivo para no perder los datos cuando se cierre el programa.
- **Interfaz de usuario**: Crear una interfaz gráfica utilizando herramientas como `Yew` o integrando con un frontend en WebAssembly.

## Contribuciones

Las contribuciones son bienvenidas. Si tienes ideas o mejoras, siéntete libre de abrir un issue o enviar un pull request.

## Licencia

Este proyecto está bajo la Licencia MIT. Para más detalles, revisa el archivo [LICENSE](LICENSE).
