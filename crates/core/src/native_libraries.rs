use crate::class_file::ConstantPoolEntry;
use dashmap::DashMap;

use libloading::Library;
use std::collections::HashMap;

// impl NativeExt for NativeLibraries {}
//
// trait NativeExt: AsRef<[..]> {
// 	fn find(&self, name: String) -> () {
// 		for lib in self.iter() {
// 			lib
// 		}
// 	}
// }
