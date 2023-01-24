mod unknown;

use rustc_codegen_ssa::{mono_item::MonoItemExt, traits::BackendTypes};
use rustc_middle::{
    mir::mono::CodegenUnit,
    ty::{layout::HasTyCtxt, TyCtxt},
};

use inkwell::{
    basic_block::BasicBlock, context::Context, module::Module, types::AnyTypeEnum, values,
};
use rustc_target::spec::HasTargetSpec;

use crate::builder::QirBuilder;

pub(crate) struct QirCodegenCompiler<'tcx: 'ir, 'ir> {
    pub tcx: TyCtxt<'tcx>,
    codegen_unit: &'tcx CodegenUnit<'tcx>,
    context: &'ir Context,
    module: Module<'ir>,
}

impl<'tcx, 'ir> QirCodegenCompiler<'tcx, 'ir> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        codegen_unit: &'tcx CodegenUnit<'tcx>,
        context: &'ir Context,
    ) -> Self {
        let module = context.create_module(codegen_unit.name().as_str());

        Self {
            tcx,
            codegen_unit,
            context,
            module,
        }
    }

    pub fn compile(self) -> Result<Module<'ir>, String> {
        for &(mono_item, (linkage, visibility)) in self
            .codegen_unit
            .items_in_deterministic_order(self.tcx)
            .iter()
        {
            mono_item.predefine::<QirBuilder<'_, '_, '_>>(&self, linkage, visibility);
        }

        Ok(self.module)
    }
}

impl<'ir> BackendTypes for QirCodegenCompiler<'_, 'ir> {
    type Value = values::BasicValueEnum<'ir>;
    type Function = values::FunctionValue<'ir>;
    type BasicBlock = BasicBlock<'ir>;
    type Type = AnyTypeEnum<'ir>;

    // Funclet: A structure representing an active landing pad for the duration of a basic block. (??)
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_codegen_llvm/common/struct.Funclet.html
    type Funclet = ();

    // Unsure what the below are. DI possibly stands for (D)ebugging (I)nformation?
    type DIScope = ();
    type DIVariable = ();
    type DILocation = ();
}

impl HasTargetSpec for QirCodegenCompiler<'_, '_> {
    fn target_spec(&self) -> &rustc_target::spec::Target {
        self.tcx.target_spec()
    }
}

impl<'tcx> HasTyCtxt<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn tcx(&self) -> rustc_middle::ty::TyCtxt<'tcx> {
        self.tcx
    }
}
