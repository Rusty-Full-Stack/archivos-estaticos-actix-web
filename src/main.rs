use actix_files::NamedFile;
use actix_web::{get, App, Error, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

// la ruta base sera /static/ luego el filename es un parametro de tipo url
// el parametro "filename" puede contener cualquier string como por ejemplo js/, css/, img/ etc
// agregarle el .* quiere decir que el parametro permitira cualquier tipo de extension como .css,
// .js, .html, etc. Se puede restringir el tipo de archivo que se quira leer, pero para nuestro
// caso de uso, .* es suficiente.
#[get("/static/{filename:.*}")]
async fn archivo_estatico(req: HttpRequest) -> Result<NamedFile, Error> {
    // Obteniendo un la ruta o pah del archivo.
    let ruta: PathBuf = req.match_info().query("filename").parse().unwrap();
    // Con el path del archivo, sera necesario agregarle el folder base de los archivos estaticos
    // para ello debemos pasar el path a un String al cual podamos agregarle el folder base.
    let mut ruta_string = ruta.into_os_string().into_string().unwrap();
    // Agregando el folder base, que le hemos llamado "static", es importando agregar "./" porque
    // la ruta base esta en el root del proyecto.
    ruta_string = format!("./static/{}", ruta_string);
    // Ahora con la ruta vamos a abrir el archivo en modo solo lectura con el metodo "open"

    // Notar el "?" al final de la linea, esto permitira regresar un Error
    // si no se puede abrir o encontrar el archivo entonces se traduce en un error
    // con status HTTP 404 (archivo no encontrado)
    let archivo = NamedFile::open(ruta_string)?;

    // Si el archivo es encontrado, entonces devolvemos la ultima version
    Ok(archivo.use_last_modified(true))
}

// Es importante notar que main devuelve un Result pero de std, no the actic_web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(archivo_estatico))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
