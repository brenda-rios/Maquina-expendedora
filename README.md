# Maquina expendedora en Solana

Este proyecto consiste en un programa desarrollado para la blockchain de Solana utilizando el framework Anchor. Permite la creación y gestión de una máquina expendedora digital mediante el backend, permitiendo un control total sobre el inventario de productos (snacks).

## Características
* Gestión de Inventario (CRUD): Permite crear una máquina, agregar productos, actualizar el stock tras una venta y eliminar productos.
* Seguridad con PDAs: El acceso y modificación de la máquina está restringido únicamente al propietario (owner) mediante semillas derivadas.
* Optimización de Espacio: Implementa InitSpace para un manejo eficiente del almacenamiento on-chain.

## Requisitos Técnico-Académicos
* Lenguaje: Rust.
* Framework: Anchor (Solana).
* Entorno de Desarrollo: Solana Playground.


## Instrucciones de Uso (Flujo CRUD)
**1. Crear Máquina**

Crea la cuenta de la máquina expendedora en la blockchain.
Función: inicializar_maquina
Parámetros: nombre_maquina (String).

**2. Agregar Productos**

Añade snacks al vector de inventario.
Función: agregar_snack
Parámetros: nombre (String), precio (u64), stock (u8).

**3. Ver Inventario**

Consulta los productos registrados. Nota: Los resultados se visualizan en los Program Logs.
Función: ver_snacks

**4. Vender Producto**

Simula una compra reduciendo el stock en una unidad.
Función: vender_snack
Parámetros: nombre (String).

**5. Eliminar Producto**

Elimina permanentemente un producto del inventario.
Función: retirar_snack
Parámetros: nombre (String).

## Estructura de Datos
El programa utiliza direcciones derivadas (PDA) para garantizar que cada usuario tenga su propia máquina única.
Las semillas (seeds) utilizadas son:
* El prefijo estático: "expendedora"
* La llave pública del usuario: `owner_pubkey`

## Créditos
Desarrollador: Brenda Rios Vargas

Institución: Universidad Tecnológica del Sur del Estado de México

Proyecto: Certificación en Desarrollo de programas (contratos inteligentes) en la blockchain de Solana usando el lenguiaje Rust.
