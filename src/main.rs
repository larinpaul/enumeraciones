//// 2023/10/01 // 6:50 //

// Enumeraciones

// Las enumeraciones permiten definir un tipo enumerando sus posibles variantes,
// de ahí su nombre.

// Por ejemplo, actualmente podemos usar direcciones IP de la versión 4 o de la
// versión 6. De este modo, nosotros podríamos definir un tipo de dato
// personalizado que sólo admitese estos valores mediante el uso de las
// enumeraciones.

enum tipoDireccionIP {
    // V4,
    // V6,

    // V4(String),
    // V6(String),

    V4(u8, u8, u8, u8),
    V6(String),

}

// fn ruta(tipoIP: tipoDireccionIP) {

// }

// struct direccionIP {
//     tipo: tipoDireccionIP,
//     direccion: String,
// }

enum Mensaje {
    Quitar,
    Mover { x: i32, y: i32},
    Escribir(String),
    CambiarColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

struct QuitarMensaje;
struct MoverMensaje {
    x: i32,
    y: i32,
}
struct EscribitMensaje(String);
struct CambiarColor(i32, i32, i32);

impl Mensaje {
    fn llamar(&self) {
        // los métodos deberian ser definidos aqui
    }
}

fn main() {

    // let cuatro = tipoDireccionIP::V4;
    // let seis = tipoDireccionIP::V6;
    // ruta(cuatro);
    // ruta(seis);
    // let loopback_v4 = direccionIP {
    //     tipo: tipoDireccionIP::V4,
    //     direccion: String::from("127.0.0.1"),
    // };

    // let loopback_v6 = direccionIP {
    //     tipo: tipoDireccionIP::V6,
    //     direccion: String::from("127.0.0.1::1"),
    // };


    // let loopback_v4 = tipoDireccionIP::V4(String::from("127.0.0.1"));
    let loopback_v4 = tipoDireccionIP::V4(127, 0, 0, 1);

    let loopback_v6 = tipoDireccionIP::V6(String::from("127.0.0.1::1"));


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

}




