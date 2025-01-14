// Verifies that KCFI type metadata for functions are emitted.
//
// revisions: aarch64 x86_64
// [aarch64] compile-flags: --target aarch64-unknown-none
// [aarch64] needs-llvm-components: aarch64
// [x86_64] compile-flags: --target x86_64-unknown-none
// [x86_64] needs-llvm-components:
// compile-flags: -Cno-prepopulate-passes -Zsanitizer=kcfi

#![crate_type="lib"]
#![feature(no_core, lang_items)]
#![no_core]

#[lang="sized"]
trait Sized { }
#[lang="copy"]
trait Copy { }

impl Copy for i32 {}

pub fn foo(f: fn(i32) -> i32, arg: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}foo
    // FIXME(rcvalle): Change <unknown kind #36> to !kcfi_type when Rust is updated to LLVM 16
    // CHECK-SAME: {{.*}}!<unknown kind #36> ![[TYPE1:[0-9]+]]
    // CHECK: call i32 %f(i32 %arg){{.*}}[ "kcfi"(i32 -1666898348) ]
    f(arg)
}

pub fn bar(f: fn(i32, i32) -> i32, arg1: i32, arg2: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}bar
    // FIXME(rcvalle): Change <unknown kind #36> to !kcfi_type when Rust is updated to LLVM 16
    // CHECK-SAME: {{.*}}!<unknown kind #36> ![[TYPE2:[0-9]+]]
    // CHECK: call i32 %f(i32 %arg1, i32 %arg2){{.*}}[ "kcfi"(i32 -1789026986) ]
    f(arg1, arg2)
}

pub fn baz(f: fn(i32, i32, i32) -> i32, arg1: i32, arg2: i32, arg3: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}baz
    // FIXME(rcvalle): Change <unknown kind #36> to !kcfi_type when Rust is updated to LLVM 16
    // CHECK-SAME: {{.*}}!<unknown kind #36> ![[TYPE3:[0-9]+]]
    // CHECK: call i32 %f(i32 %arg1, i32 %arg2, i32 %arg3){{.*}}[ "kcfi"(i32 1248878270) ]
    f(arg1, arg2, arg3)
}

// CHECK: ![[TYPE1]] = !{i32 653723426}
// CHECK: ![[TYPE2]] = !{i32 412174924}
// CHECK: ![[TYPE3]] = !{i32 -636668840}
