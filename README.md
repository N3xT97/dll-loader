### 사용 예시

```rust
use clap::Parser;
use dll_loader_lib::{DllLoader, DllLoaderError, DllPath};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dll_path: String,
    //#[arg(short, long)]
    //export_function_name: Option<String>,
}

fn app(args: &Args) -> Result<(), DllLoaderError> {
    let dll_path = DllPath::new(&args.dll_path)?;
    DllLoader::run_with_loop(dll_path)
}

fn main() {
    println!("[+] Dll-Loader started...");
    for (i, arg) in std::env::args().enumerate() {
        println!(" - argv[{i}]: {arg}");
    }

    let args = Args::parse();

    if let Err(e) = app(&args) {
        println!("[-] {}", e);
    }
}

```
