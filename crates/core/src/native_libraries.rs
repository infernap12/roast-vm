use crate::class_file::ConstantPoolEntry;
use dashmap::DashMap;
use libloading::os::windows::Library;
use std::collections::HashMap;

pub type NativeLibraries = HashMap<String, Library>;
// impl NativeExt for NativeLibraries {}
//
// trait NativeExt: AsRef<[..]> {
// 	fn find(&self, name: String) -> () {
// 		for lib in self.iter() {
// 			lib
// 		}
// 	}
// }
