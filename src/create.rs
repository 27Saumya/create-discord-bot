use crate::boilerplates::*;
use crate::ui::*;
use std::{
    fs::{create_dir, File, OpenOptions},
    io::Write,
    path::Path,
};

fn check_cog(cog: usize) -> bool {
    if cog == 0 {
        return true;
    } else {
        return false;
    };
}

pub fn generate_python_boilerplate(cog: usize) {
    let chose_cog = check_cog(cog);

    if Path::new("main.py").exists() {
        println!("{}", error_message("\nmain.py file already exists!"));
        return;
    }

    if chose_cog {
        if Path::new("./cogs").exists() {
            println!("{}", error_message("Cogs directory already exists!"));
            return;
        }

        let _ = create_dir("./cogs/").expect(&error_message("Failed to create cogs/ directory"));

        let mut cog_example_file = File::create("./cogs/example.py")
            .expect(&error_message("Failed to create cogs example file"));
        let _ = cog_example_file.write_all(PYTHON_COG_EXAMPLE_CONTENT.as_bytes());

        let mut main_file =
            File::create("main.py").expect(&error_message("Failed to create main.py file"));
        let _ = main_file.write_all(PYTHON_MAINFILE_COG_CONTENT.as_bytes());
    } else {
        let mut main_file =
            File::create("main.py").expect(&error_message("Failed to create main.py file"));
        let _ = main_file.write_all(PYTHON_MAINFILE_CONTENT.as_bytes());
    };

    println!(
        "{}",
        success_message("\nSuccessfully created your discord.py project, happy coding!")
    )
}

pub fn generate_js_boilerplate(cog: usize) {
    let chose_cog = check_cog(cog);
    const REQUIRED_FILES: [&str; 2] = ["main.js", "config.js"];

    for required_file in REQUIRED_FILES {
        if Path::new(required_file).exists() {
            let err_message = &format!("\n{} file already exists!", required_file);
            println!("{}", error_message(err_message));
            return;
        }
    }

    let mut config_file =
        File::create("config.json").expect(&error_message("Failed to create config.json"));
    let _ = config_file.write("{\n\t'token': 'your-token'\n}".as_bytes());

    let mut gitignore_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(".gitignore")
        .unwrap();

    if let Err(e) = writeln!(gitignore_file, "\nnode_modules\nconfig.json") {
        eprintln!("Couldnt't write line! {}", e);
    }

    if chose_cog {
        if Path::new("./events").exists() {
            println!("{}", error_message("\nevents directory already exists!"));
            return;
        };
        if Path::new("./commands").exists() {
            println!("{}", error_message("\ncommands directory already exists!"));
            return;
        };

        let mut main_file =
            File::create("main.js").expect(&error_message("Failed to create main.js file"));
        let _ = main_file.write_all(JS_MAINFILE_COG_CONTENT.as_bytes());
        const DIRS: [&str; 2] = ["events/", "commands/"];
        for dir in DIRS {
            let _ = create_dir(dir).expect(&error_message(&format!(
                "Failed to create {} directory",
                dir
            )));
        }
    } else {
        let mut main_file =
            File::create("main.js").expect(&error_message("Failed to create main.js file"));
        let _ = main_file.write_all(JS_MAINFILE_CONTENT.as_bytes());
    }

    println!(
        "{}",
        success_message("\nSuccessfully created your discord.js project, happy coding!")
    )
}

pub fn generate_rust_boilerplate(cog: usize) {
    let chose_cog = check_cog(cog);
    const REQUIRED_FILES: [&str; 3] = ["src/", "Makefile.toml", "Cargo.toml"];

    for required_file in REQUIRED_FILES {
        if Path::new(required_file).exists() {
            let err_message = &format!("\n{} file already exists!", required_file);
            println!("{}", error_message(err_message));
            return;
        }
    }

    let _ = create_dir("src/").expect(&error_message("Failed to create cogs/ directory"));

    let mut dotenv_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(".env")
        .unwrap();

    if let Err(e) = writeln!(
        dotenv_file,
        "\nDISCORD_TOKEN=your token here\n\nRUST_LOG=debug"
    ) {
        eprintln!("Couldnt't write line! {}", e);
    }

    let makefile_content = "extend = '../../Makefile.toml'\n\n[tasks.examples_build]\nalias = 'build'\n\n[tasks.examples_build_release]\nalias = 'build_release'\n\n[tasks.examples_run]\nalias = 'run'\n\n[tasks.examples_run_release]\nalias = 'run_release'";
    let mut makefile = File::create("Makefile.toml")
        .expect(&error_message("Failed to create Makefile.toml file!"));
    let _ = makefile.write_all(makefile_content.as_bytes());

    if chose_cog {
        let cargo_toml_content = "[package]\nname = 'my-bot'\nversion = '0.1.0'\nauthors = ['my name <my@email.address>']\nedition = '2018'\n\n[dependencies]\ndotenv = '0.15'\ntracing = '0.1.23'\ntracing-subscriber = '0.2'\n\n[dependencies.tokio]\nversion = '1.0'\nfeatures = ['macros', 'signal', 'rt-multi-thread']\n\n[dependencies.serenity]\nfeatures = ['cache', 'framework', 'standard_framework', 'rustls_backend']\npath = '../../'";
        let mut cargo_toml_file =
            File::create("Cargo.toml").expect(&error_message("Failed to create Cargo.toml file!"));
        let _ = cargo_toml_file.write_all(cargo_toml_content.as_bytes());

        let _ = create_dir("src/commands")
            .expect(&error_message("Failed ot create cogs/commands directory"));

        let mut main_file =
            File::create("src/main.rs").expect(&error_message("Failed to create src/main.rs file"));
        let _ = main_file.write_all(RUST_MAINFILE_COG_CONTENT.as_bytes());

        let mut example_cog_file = File::create("src/commands/math.rs")
            .expect(&error_message("Failed to create src/commands/math.rs file"));
        let _ = example_cog_file.write_all(RUST_EXAMPLE_COG_CONTENT.as_bytes());
    } else {
        let mut cargo_toml_file =
            File::create("Cargo.toml").expect(&error_message("Failed to create Cargo.toml file!"));
        let _ = cargo_toml_file.write_all(CARGO_TOML_CONTENT.as_bytes());

        let mut main_file =
            File::create("src/main.rs").expect(&error_message("Failed to create src/main.rs file"));
        let _ = main_file.write_all(RUST_MAINFILE_CONTENT.as_bytes());
    }

    println!(
        "{}",
        success_message("\nSuccessfully created your serenity project, happy coding!")
    )
}
