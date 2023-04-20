#![feature(lazy_cell)]
#![feature(naked_functions)]

mod dl;
mod dxgi_shim;
mod hooks;
mod modules;

pub fn main() -> Result<(), anyhow::Error> {
    static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if INITIALIZED.fetch_or(true, std::sync::atomic::Ordering::SeqCst) {
        return Ok(());
    }

    unsafe {
        winapi::um::consoleapi::AllocConsole();
    }

    env_logger::Builder::from_default_env()
        .filter(Some("dxgi"), log::LevelFilter::Info)
        .init();
    log::info!("mod loader ready!");

    unsafe {
        hooks::install()?;
    }

    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _module: winapi::shared::minwindef::HINSTANCE,
    call_reason: winapi::shared::minwindef::DWORD,
    _reserved: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::BOOL {
    if call_reason != winapi::um::winnt::DLL_PROCESS_ATTACH {
        return winapi::shared::minwindef::TRUE;
    }
    main().unwrap();
    winapi::shared::minwindef::TRUE
}
