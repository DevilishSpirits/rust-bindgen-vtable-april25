#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
	unsafe{
		let bug_ptr = new_bug_illustration();
		let bug = &mut*bug_ptr;
		let bug_vtable = &*bug.vtable_;
		(bug_vtable.BugIllustration_fn3)(bug_ptr);
		free_bug_illustration(bug_ptr);
	}
}
