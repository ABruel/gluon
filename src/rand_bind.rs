//! Module containing bindings to the `rand` library.

extern crate rand;
extern crate rand_xorshift;

use self::rand::{Rng, SeedableRng};

use crate::vm::api::{RuntimeResult, IO};
use crate::vm::thread::Thread;
use crate::vm::types::VmInt;
use crate::vm::{self, ExternModule};

#[derive(Clone, Debug, Userdata)]
#[gluon(crate_name = "::vm")]
struct XorShiftRng(self::rand_xorshift::XorShiftRng);

field_decl! { value, gen }

fn next_int(_: ()) -> IO<VmInt> {
    IO::Value(rand::thread_rng().gen())
}

fn next_float(_: ()) -> IO<f64> {
    IO::Value(rand::thread_rng().gen())
}

fn gen_int_range(low: VmInt, high: VmInt) -> IO<VmInt> {
    IO::Value(rand::thread_rng().gen_range(low, high))
}

type RngNext<G> = record_type! {
    value => VmInt,
    gen => G
};

fn xor_shift_new(seed: &[u8]) -> RuntimeResult<XorShiftRng, String> {
    if seed.len() == 16 {
        let seed = unsafe { *(seed.as_ptr() as *const [u8; 16]) };
        RuntimeResult::Return(XorShiftRng(self::rand_xorshift::XorShiftRng::from_seed(
            seed,
        )))
    } else {
        RuntimeResult::Panic("Expected xorshift seed to have 4 elements".to_string())
    }
}

fn xor_shift_next(gen: &XorShiftRng) -> RngNext<XorShiftRng> {
    let mut gen = gen.clone();
    record_no_decl! {
        value => gen.0.gen(),
        gen => gen
    }
}

mod std {
    pub mod random {
        pub use crate::rand_bind as prim;
    }
}

pub fn load(vm: &Thread) -> vm::Result<ExternModule> {
    use self::std;

    vm.register_type::<XorShiftRng>("XorShiftRng", &[])?;

    ExternModule::new(
        vm,
        record! {
            type XorShiftRng => XorShiftRng,
            next_int => primitive!(1, std::random::prim::next_int),
            next_float => primitive!(1, std::random::prim::next_float),
            gen_int_range => primitive!(2, std::random::prim::gen_int_range),
            xor_shift_new => primitive!(1, std::random::prim::xor_shift_new),
            xor_shift_next => primitive!(1, std::random::prim::xor_shift_next)
        },
    )
}
