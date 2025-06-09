# 🔗 DLL Loader

Rust로 작성된 간단한 DLL 로더입니다. 지정한 DLL을 로드하고 언로드되지 않도록 유지합니다. 주로 디버깅, 테스트 또는 프로세스 환경에서 특정 DLL을 강제로 로드할 때 사용할 수 있습니다.

## 📦 Features

- 사용자 지정 경로의 DLL 파일 로드
- 로딩된 DLL을 언로드되지 않도록 유지
- 에러 메시지 출력 지원

## 🔧 사용 예시

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

fn dll_load_and_wait(args: &Args) -> Result<(), DllLoaderError> {
    let dll_path = DllPath::new(&args.dll_path)?;
    DllLoader::run_with_loop(dll_path)
}

fn main() {
    println!("[+] Dll-Loader started...");
    for (i, arg) in std::env::args().enumerate() {
        println!(" - argv[{i}]: {arg}");
    }

    let args = Args::parse();

    if let Err(e) = dll_load_and_wait(&args) {
        println!("[-] {}", e);
    }
}
```
