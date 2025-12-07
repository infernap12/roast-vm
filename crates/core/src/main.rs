use roast_vm_core::vm::Vm;
use libloading::Library;
use log::LevelFilter;

fn main() {
	env_logger::Builder::from_default_env()
		.filter_level(LevelFilter::Trace)
		.filter_module("deku", LevelFilter::Warn)
		.filter_module("roast_vm_core::class_file::class_file", LevelFilter::Info)
		.filter_module("roast_vm_core::attributes", LevelFilter::Info)
		.filter_module("roast_vm_core::instructions", LevelFilter::Info)
		.init();
	let vm = Vm::new();
	vm.load_native_library("roast_vm.dll", load("roast_vm.dll").into());
	vm.load_native_library("jvm.dll", load("jvm.dll").into());
	vm.load_native_library("java.dll", load("java.dll").into());
	vm.run("org/example/Main");
}

fn load(filename: &str) -> Library {
	let exe_path = std::env::current_exe().expect("get exe path");
	let dll_path = exe_path.parent().unwrap().join(filename);

	let leeb = unsafe { libloading::os::windows::Library::new(&dll_path) }.expect("load dll");

	Library::from(leeb)
}
