use rustc_middle::mir::mono::CodegenUnit;
use rustc_middle::ty::TyCtxt;

use std::arch::asm;

pub struct QirCodecgenUnit<'tcx> {
    tcx: TyCtxt<'tcx>,
    codegen_unit: &'tcx CodegenUnit<'tcx>,
}

impl<'tcx> QirCodecgenUnit<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, codegen_unit: &'tcx CodegenUnit<'tcx>) -> Self {
        Self { tcx, codegen_unit }
    }

    pub fn assemble(self) -> Vec<u32> {
        let x: u64;
        unsafe {
            asm!("mov {}, 5", out(reg) x);
        }

        vec![2, 3, 3]
    }
}
