mod unknown;

use std::ops::Deref;

use rustc_codegen_ssa::traits::{BackendTypes, HasCodegen};
use rustc_middle::ty::{
    layout::{HasParamEnv, HasTyCtxt},
    ParamEnv, TyCtxt,
};
use rustc_target::{abi::HasDataLayout, spec::HasTargetSpec};

use crate::codegen::QirCodegenCompiler;

pub(crate) struct QirBuilder<'a, 'tcx: 'ir, 'ir> {
    compiler: &'a QirCodegenCompiler<'tcx, 'ir>,
}

/// Allow calling the inner [QirCodegenUnit] methods on the builder.
///
/// Note: This is needed for [HasCodegen], which assumes that it can
/// interact with the codegen unit through the builder.
impl<'tcx, 'ir> Deref for QirBuilder<'_, 'tcx, 'ir> {
    type Target = QirCodegenCompiler<'tcx, 'ir>;

    fn deref(&self) -> &Self::Target {
        self.compiler
    }
}

// Since we allow Deref for the inner QirCodegenCompiler, we expose its BackendTypes here.
impl<'tcx, 'ir> BackendTypes for QirBuilder<'_, 'tcx, 'ir> {
    type Value = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::Value;
    type Function = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::Function;
    type BasicBlock = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::BasicBlock;
    type Type = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::Type;
    type Funclet = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::Funclet;

    type DIScope = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::DIScope;
    type DIVariable = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::DIVariable;
    type DILocation = <QirCodegenCompiler<'tcx, 'ir> as BackendTypes>::DILocation;
}

impl<'tcx, 'ir> HasCodegen<'tcx> for QirBuilder<'_, 'tcx, 'ir> {
    type CodegenCx = QirCodegenCompiler<'tcx, 'ir>;
}

impl<'tcx> HasDataLayout for QirBuilder<'_, 'tcx, '_> {
    fn data_layout(&self) -> &rustc_target::abi::TargetDataLayout {
        self.compiler.data_layout()
    }
}

impl<'tcx> HasParamEnv<'tcx> for QirBuilder<'_, 'tcx, '_> {
    fn param_env(&self) -> ParamEnv<'tcx> {
        self.compiler.param_env()
    }
}

impl HasTargetSpec for QirBuilder<'_, '_, '_> {
    fn target_spec(&self) -> &rustc_target::spec::Target {
        self.compiler.target_spec()
    }
}

impl<'tcx> HasTyCtxt<'tcx> for QirBuilder<'_, 'tcx, '_> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.compiler.tcx
    }
}
