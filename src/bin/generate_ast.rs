use ::std::env::args;
use std::{
    fs::{File, OpenOptions},
    io::{LineWriter, Write},
};

fn main() {
    let ouput_directory: String = match args().count() {
        2 => args().nth(1).unwrap(),
        _ => {
            println!("Usage: generate_ast <output directory>");
            std::process::exit(64);
        }
    };

    define_ast(
        ouput_directory,
        "Expr",
        vec![
            "Unary      : Token operator, Expr right",
            "Binary     : Expr left, Token operator, Expr right",
            "Grouping   : Expr Expression",
            "Literal    : Object value",
        ],
    );
}

fn define_ast(_output_directory: String, base_name: &str, types: Vec<&str>) {
    // let path: String = output_directory + "/" + base_name + ".rs";
    let path: String = base_name.to_owned() + ".rs";
    // println!("path: {}", path);

    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut writer = LineWriter::new(file);

    // Base Expr trait
    let _ = writer.write_all(
        b"trait Expr<R> {
            fn accept<T: Visitor<R>>(&mut self, visitor: &mut T);
          }\n",
    );

    // Visitor trait
    let _ = writer.write_all(b"trait Visitor<R> {\n");

    for expr_type in types {
        let type_name = expr_type.split(':').nth(0).unwrap().trim();
        let _ = writer.write_fmt(format_args!(
            "fn visit_{}_expr(&mut self, expr: &mut {}) -> R;\n",
            type_name.to_lowercase(),
            type_name
        ));

    }

    // Types
    // for expr_type in types {
    //     let type_name = expr_type.split(':').nth(0).unwrap().trim();
        // let _ = writer.write(format_args!(
        //         "struct {}",
        //         type_name
        // ));
    // }

    let _ = writer.write(b"}\n");

    writer.flush().unwrap();
}
