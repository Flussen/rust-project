fn main() {
   println!("Elige la azul o roja");
   let mut eleccion = String::new();
   std::io::stdin().read_line(&mut eleccion).unwrap();
   let elecc:String = eleccion.trim().to_lowercase().to_string();

    if elecc == "azul" {
        println!("enviado al mundo matrix")
    }else{
        println!("no sucedió nada")
    }
}
