use roast_vm_core::vm::Vm;
use roast_vm_core::error::VmError;
use libloading::Library;
use log::{error, LevelFilter};
use colored::Colorize;
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
	match vm.run("org/example/Main") {
		Ok(_) => {}
		Err(VmError::Exception { message, stack_trace }) => {
			let thread = vm.threads.get(&vm.main_thread_id).unwrap();
			let objs = thread.gc.read().unwrap()
				.objects
				.iter()
				.map(|(x, y)| format!("{x} : {y}"))
				.collect::<Vec<_>>();
			let len = objs.len().clone();
			error!("Heap dump: len: {len} objs:\n{objs:#?}");
			eprintln!("{}: {}", "Exception".red().bold(), message);
			for elem in &stack_trace {
				let class_name = elem.class.replace('/', ".");
				let at = "\tat".dimmed();
				let location = match (&elem.file, elem.line) {
					(Some(f), Some(l)) => format!("({}:{})", f, l),
					(Some(f), None) => format!("({})", f),
					_ => "(Unknown Source)".to_string(),
				}.blue().dimmed();
				eprintln!("{} {}.{}{}", at, class_name, elem.method, location);
			}
			/*error!("Exception: {}", message);
			for elem in &stack_trace {
				let class_name = elem.class.replace('/', ".");
				match (&elem.file, elem.line) {
					(Some(f), Some(l)) => eprintln!("\tat {}.{}({}:{})", class_name, elem.method, f, l),
					(Some(f), None) => eprintln!("\tat {}.{}({})", class_name, elem.method, f),
					_ => eprintln!("\tat {}.{}(Unknown Source)", class_name, elem.method),
				}
			}*/
		}
		Err(e) => {
			error!("VM Error: {:?}", e);
		}
	}
}

fn load(filename: &str) -> Library {
	let exe_path = std::env::current_exe().expect("get exe path");
	let dll_path = exe_path.parent().unwrap().join(filename);

	let leeb = unsafe { libloading::os::windows::Library::new(&dll_path) }.expect("load dll");

	Library::from(leeb)
}
