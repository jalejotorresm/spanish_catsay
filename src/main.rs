use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::Read;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Miau!")]
    ///Como dice el gato?
    message: String,

    #[clap(short = 'd', long = "dead")]
    ///El gato dice el mensaje desde el mas alla.
    dead: bool,

    #[clap(short = 'f', long = "file")]
    ///Envia un archivo .txt de un gatito ASCII para obtener un mensaje personalizado.
    catfile: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    ///Permite leer el mensaje desde el stdin en vez del argumento directo del comando
    stdin: bool,
    //TODO = Se podra modificar el mensaje de las flasg h/help que viene por defecto a un mensaje en español?
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();

    if options.stdin {
        std::io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" || message.to_lowercase() == "guau" {
        eprintln!("No estoy seguro que nuestro gatito deberia decir esto:")
    };

    let eye = if options.dead {
        'x'.to_string().red().bold()
    } else {
        'o'.to_string().green().bold()
    };

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("No se pudo leer el contenido de {:?}", path))?;
            let eye = if options.dead {
                format!("{}", 'x'.to_string().red().bold())
            } else {
                format!("{}", 'o'.to_string().green().bold())
            };
            let cat_picture = cat_template.replace("{eye}", &eye);

            println!("{}", message.bright_white().bold().on_cyan());
            println!("{}", &cat_picture);
            std::process::exit(0)
        }
        None => {}
    }

    println!("{}", message.bright_white().bold().on_cyan());
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )");
    println!("   =( I )=");

    Ok(())
}
