# Maquina expendedora en Solana

Este proyecto consiste en un programa desarrollado para la blockchain de Solana utilizando el framework Anchor. Permite la creación y gestión de una máquina expendedora digital mediante el backend, permitiendo un control total sobre el inventario de productos (snacks).

## Características
* Gestión de Inventario (CRUD): Permite crear una máquina, agregar productos, actualizar el stock tras una venta y eliminar productos.
* Seguridad con PDAs: El acceso y modificación de la máquina está restringido únicamente al propietario (owner) mediante semillas derivadas.
* Optimización de Espacio: Implementa InitSpace para un manejo eficiente del almacenamiento on-chain.

## Requisitos Técnico-Académicos
* Lenguaje: Rust.
* Framework: Anchor (Solana).
* Entorno de Desarrollo: Solana Playground / VS Code con Anchor CLI.

## Instrucciones de Uso (Flujo CRUD)
1. Inicializar Máquina (Create)
Crea la cuenta de la máquina expendedora en la blockchain.
Función: inicializar_maquina
Parámetros: nombre_maquina (String).

2. Agregar Productos (Create/Update)
Añade snacks al vector de inventario.
Función: agregar_snack
Parámetros: nombre (String), precio (u64), stock (u8).

4. Ver Inventario (Read)
Consulta los productos registrados. Nota: Los resultados se visualizan en los Program Logs.
Función: ver_snacks

4. Vender Producto (Update)
Simula una compra reduciendo el stock en una unidad.
Función: vender_snack
Parámetros: nombre (String).

5. Retirar Producto (Delete)
Elimina permanentemente un producto del inventario.
Función: retirar_snack
Parámetros: nombre (String).



![banner](./images/banner-biblioteca.jpg)

CRUD básico de un Solana Program desarrollado con Rust y Anchor desde el Solana Playground. 

Puedes comenzar dándole Fork a este repositorio (abajo te explicamos como 👇), **hemos preparado un entorno de codespaces listo para que no tengas que instalar nada**, solo déjate llevar por la fluidez de los ejercicios y temas desarrollados especialmente para ti. 

Asegúrate de clonar este repositorio a tu cuenta usando el botón **`Fork`**.

![fork](./images/fork.png)

## Importando el proyecto 

Ya con el repositorio en tu cuenta lo siguiente que debes hacer copiar el `enlace de tu repositorio`, lo que se puede hacer directamente desdel navegador:

![repo](./images/repo.png)
Posteriormente, lo uniremos con el siguiente enlace en nuestro navegador de preferencia:

```url
https://beta.solpg.io/
```

Lo que nos dará algo parecido a:

![url](./images/url.png)

Al pulsar enter seremos enviados al `Solana Playground` con nuestro proyecto abierto:

![pg](./images/pg.png)

Para guardarlo solo damos clic en el boton `import` y asignamos un nombre:

![import](./images/import.png)

## Preparacion del entorno

Primero conectaremos el entorno con la devnet, lo que tambien procederá a la creación de una wallet. Para eso daremos clic en donde dice **Not Conected**:

![playground1](./images/playground1.png)

Saldrá la siguiente ventana donde daremos en el botón **Continue**:

![wallet](./images/wallet.png)

Como resultado se mostrará la siguiente información:

![status](./images/status.png)

* En verde: el estado de la conexión y el entorno al que se encuentra conectado

* En amarillo: la la dirección de la wallet conectada

* En azul: la cantidad de tokens en la wallet

> ℹ️ ¿Quieres ver el ejemplo de un "Hola Mundo" en Solana?. Da clic aquí: 👉 [Ver Ejemplo](https://github.com/WayLearnLatam/Solana-starter-kit/tree/1fc6349ba63375a3fe223d8d56911bc64765459b/build-deploy)

> ℹ️ ¿Cuentas con una Wallet de [Phantom](https://phantom.com/) que deseas importar?, Da clic aquí para ver como hacerlo: 

👉 [Como Importar una Wallet](https://github.com/WayLearnLatam/Solana-starter-kit/tree/1fc6349ba63375a3fe223d8d56911bc64765459b/import-key-a-playground)
