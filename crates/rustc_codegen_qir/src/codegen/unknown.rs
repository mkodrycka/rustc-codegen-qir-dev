// This file contains required implementations for Codegen
// that are not yet known. As the below methods are investigated and
// implemented, they should be moved to the corresponding source file
// in which the implementation belongs and thoroughly commented.

use rustc_abi::HasDataLayout;
use rustc_codegen_ssa::traits::{
    AsmMethods, BaseTypeMethods, ConstMethods, CoverageInfoMethods, DebugInfoMethods,
    LayoutTypeMethods, MiscMethods, PreDefineMethods, StaticMethods, TypeMembershipMethods,
};
use rustc_middle::ty::{
    layout::{FnAbiOfHelpers, HasParamEnv, LayoutOfHelpers, TyAndLayout},
    Ty,
};
use rustc_target::abi::call::FnAbi;

use super::QirCodegenCompiler;

impl<'tcx> PreDefineMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn predefine_static(
        &self,
        def_id: rustc_hir::def_id::DefId,
        linkage: rustc_middle::mir::mono::Linkage,
        visibility: rustc_middle::mir::mono::Visibility,
        symbol_name: &str,
    ) {
        log::debug!("::QirCodegenCompiler::PreDefineMethods predefine_static");
        todo!()
    }

    fn predefine_fn(
        &self,
        instance: rustc_middle::ty::Instance<'tcx>,
        linkage: rustc_middle::mir::mono::Linkage,
        visibility: rustc_middle::mir::mono::Visibility,
        symbol_name: &str,
    ) {
        log::debug!("::QirCodegenCompiler::PreDefineMethods predefine_fn");
        todo!()
    }
}

impl<'tcx> AsmMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn codegen_global_asm(
        &self,
        template: &[rustc_ast::InlineAsmTemplatePiece],
        operands: &[rustc_codegen_ssa::traits::GlobalAsmOperandRef<'tcx>],
        options: rustc_ast::InlineAsmOptions,
        line_spans: &[rustc_span::Span],
    ) {
        log::debug!("::QirCodegenCompiler::AsmMethods codegen_global_asm");
        todo!()
    }
}

impl<'tcx> DebugInfoMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn create_vtable_debuginfo(
        &self,
        ty: rustc_middle::ty::Ty<'tcx>,
        trait_ref: Option<rustc_middle::ty::PolyExistentialTraitRef<'tcx>>,
        vtable: Self::Value,
    ) {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods create_vtable_debuginfo");
        todo!()
    }

    fn create_function_debug_context(
        &self,
        instance: rustc_middle::ty::Instance<'tcx>,
        fn_abi: &rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
        llfn: Self::Function,
        mir: &rustc_middle::mir::Body<'tcx>,
    ) -> Option<
        rustc_codegen_ssa::mir::debuginfo::FunctionDebugContext<Self::DIScope, Self::DILocation>,
    > {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods create_function_debug_context");
        todo!()
    }

    fn dbg_scope_fn(
        &self,
        instance: rustc_middle::ty::Instance<'tcx>,
        fn_abi: &rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
        maybe_definition_llfn: Option<Self::Function>,
    ) -> Self::DIScope {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods dbg_scope_fn");
        todo!()
    }

    fn dbg_loc(
        &self,
        scope: Self::DIScope,
        inlined_at: Option<Self::DILocation>,
        span: rustc_span::Span,
    ) -> Self::DILocation {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods dbg_loc");
        todo!()
    }

    fn extend_scope_to_file(
        &self,
        scope_metadata: Self::DIScope,
        file: &rustc_span::SourceFile,
    ) -> Self::DIScope {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods extend_scope_to_file");
        todo!()
    }

    fn debuginfo_finalize(&self) {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods debuginfo_finalize");
        todo!()
    }

    fn create_dbg_var(
        &self,
        variable_name: rustc_span::Symbol,
        variable_type: rustc_middle::ty::Ty<'tcx>,
        scope_metadata: Self::DIScope,
        variable_kind: rustc_codegen_ssa::mir::debuginfo::VariableKind,
        span: rustc_span::Span,
    ) -> Self::DIVariable {
        log::debug!("::QirCodegenCompiler::DebugInfoMethods create_dbg_var");
        todo!()
    }
}

impl<'tcx> CoverageInfoMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn coverageinfo_finalize(&self) {
        log::debug!("::QirCodegenCompiler::CoverageInfoMethods coverageinfo_finalize");
        todo!()
    }

    fn define_unused_fn(&self, def_id: rustc_hir::def_id::DefId) {
        log::debug!("::QirCodegenCompiler::CoverageInfoMethods define_unused_fn");
        todo!()
    }

    fn get_pgo_func_name_var(&self, instance: rustc_middle::ty::Instance<'tcx>) -> Self::Value {
        log::debug!("::QirCodegenCompiler::CoverageInfoMethods get_pgo_func_name_var");
        todo!()
    }
}

impl StaticMethods for QirCodegenCompiler<'_, '_> {
    fn static_addr_of(
        &self,
        cv: Self::Value,
        align: rustc_abi::Align,
        kind: Option<&str>,
    ) -> Self::Value {
        log::debug!("::QirCodegenCompiler::StaticMethods static_addr_of");
        todo!()
    }

    fn codegen_static(&self, def_id: rustc_hir::def_id::DefId, is_mutable: bool) {
        log::debug!("::QirCodegenCompiler::StaticMethods codegen_static");
        todo!()
    }

    fn add_used_global(&self, global: Self::Value) {
        log::debug!("::QirCodegenCompiler::StaticMethods add_used_global");
        todo!()
    }

    fn add_compiler_used_global(&self, global: Self::Value) {
        log::debug!("::QirCodegenCompiler::StaticMethods add_compiler_used_global");
        todo!()
    }
}

impl<'tcx> ConstMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn const_null(&self, t: Self::Type) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_null");
        todo!()
    }

    fn const_undef(&self, t: Self::Type) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_undef");
        todo!()
    }

    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_int");
        todo!()
    }

    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_uint");
        todo!()
    }

    fn const_uint_big(&self, t: Self::Type, u: u128) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_uint_big");
        todo!()
    }

    fn const_bool(&self, val: bool) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_bool");
        todo!()
    }

    fn const_i16(&self, i: i16) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_i16");
        todo!()
    }

    fn const_i32(&self, i: i32) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_i32");
        todo!()
    }

    fn const_u32(&self, i: u32) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_u32");
        todo!()
    }

    fn const_u64(&self, i: u64) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_u64");
        todo!()
    }

    fn const_usize(&self, i: u64) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_usize");
        todo!()
    }

    fn const_u8(&self, i: u8) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_u8");
        todo!()
    }

    fn const_real(&self, t: Self::Type, val: f64) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_real");
        todo!()
    }

    fn const_str(&self, s: &str) -> (Self::Value, Self::Value) {
        log::debug!("::QirCodegenCompiler::ConstMethods const_str");
        todo!()
    }

    fn const_struct(&self, elts: &[Self::Value], packed: bool) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_struct");
        todo!()
    }

    fn const_to_opt_uint(&self, v: Self::Value) -> Option<u64> {
        log::debug!("::QirCodegenCompiler::ConstMethods const_to_opt_uint");
        todo!()
    }

    fn const_to_opt_u128(&self, v: Self::Value, sign_ext: bool) -> Option<u128> {
        log::debug!("::QirCodegenCompiler::ConstMethods const_to_opt_u128");
        todo!()
    }

    fn const_data_from_alloc(
        &self,
        alloc: rustc_const_eval::interpret::ConstAllocation<'tcx>,
    ) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_data_from_alloc");
        todo!()
    }

    fn scalar_to_backend(
        &self,
        cv: rustc_const_eval::interpret::Scalar,
        layout: rustc_target::abi::Scalar,
        llty: Self::Type,
    ) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods scalar_to_backend");
        todo!()
    }

    fn from_const_alloc(
        &self,
        layout: rustc_middle::ty::layout::TyAndLayout<'tcx>,
        alloc: rustc_const_eval::interpret::ConstAllocation<'tcx>,
        offset: rustc_abi::Size,
    ) -> rustc_codegen_ssa::mir::place::PlaceRef<'tcx, Self::Value> {
        log::debug!("::QirCodegenCompiler::ConstMethods from_const_alloc");
        todo!()
    }

    fn const_ptrcast(&self, val: Self::Value, ty: Self::Type) -> Self::Value {
        log::debug!("::QirCodegenCompiler::ConstMethods const_ptrcast");
        todo!()
    }
}

impl<'tcx> MiscMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn vtables(
        &self,
    ) -> &std::cell::RefCell<
        rustc_hash::FxHashMap<
            (
                rustc_middle::ty::Ty<'tcx>,
                Option<rustc_middle::ty::PolyExistentialTraitRef<'tcx>>,
            ),
            Self::Value,
        >,
    > {
        log::debug!("::QirCodegenCompiler::MiscMethods vtables");
        todo!()
    }

    fn check_overflow(&self) -> bool {
        log::debug!("::QirCodegenCompiler::MiscMethods check_overflow");
        todo!()
    }

    fn get_fn(&self, instance: rustc_middle::ty::Instance<'tcx>) -> Self::Function {
        log::debug!("::QirCodegenCompiler::MiscMethods get_fn");
        todo!()
    }

    fn get_fn_addr(&self, instance: rustc_middle::ty::Instance<'tcx>) -> Self::Value {
        log::debug!("::QirCodegenCompiler::MiscMethods get_fn_addr");
        todo!()
    }

    fn eh_personality(&self) -> Self::Value {
        log::debug!("::QirCodegenCompiler::MiscMethods eh_personality");
        todo!()
    }

    fn sess(&self) -> &rustc_session::Session {
        log::debug!("::QirCodegenCompiler::MiscMethods sess");
        todo!()
    }

    fn codegen_unit(&self) -> &'tcx rustc_middle::mir::mono::CodegenUnit<'tcx> {
        log::debug!("::QirCodegenCompiler::MiscMethods codegen_unit");
        todo!()
    }

    fn set_frame_pointer_type(&self, llfn: Self::Function) {
        log::debug!("::QirCodegenCompiler::MiscMethods set_frame_pointer_type");
        todo!()
    }

    fn apply_target_cpu_attr(&self, llfn: Self::Function) {
        log::debug!("::QirCodegenCompiler::MiscMethods apply_target_cpu_attr");
        todo!()
    }

    fn declare_c_main(&self, fn_type: Self::Type) -> Option<Self::Function> {
        log::debug!("::QirCodegenCompiler::MiscMethods declare_c_main");
        todo!()
    }
}

impl<'tcx> TypeMembershipMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn set_type_metadata(&self, function: Self::Function, typeid: String) {
        log::debug!("::QirCodegenCompiler::TypeMembershipMethods set_type_metadata");
        todo!()
    }

    fn typeid_metadata(&self, typeid: String) -> Self::Value {
        log::debug!("::QirCodegenCompiler::TypeMembershipMethods typeid_metadata");
        todo!()
    }

    fn set_kcfi_type_metadata(&self, function: Self::Function, typeid: u32) {
        log::debug!("::QirCodegenCompiler::TypeMembershipMethods set_kcfi_type_metadata");
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for QirCodegenCompiler<'tcx, '_> {
    type FnAbiOfResult = &'tcx FnAbi<'tcx, Ty<'tcx>>;

    fn handle_fn_abi_err(
        &self,
        err: rustc_middle::ty::layout::FnAbiError<'tcx>,
        span: rustc_span::Span,
        fn_abi_request: rustc_middle::ty::layout::FnAbiRequest<'tcx>,
    ) -> ! {
        log::debug!("::QirCodegenCompiler::FnAbiOfHelpers handle_fn_abi_err");
        todo!()
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for QirCodegenCompiler<'tcx, '_> {
    type LayoutOfResult = TyAndLayout<'tcx>;

    fn handle_layout_err(
        &self,
        err: rustc_middle::ty::layout::LayoutError<'tcx>,
        span: rustc_span::Span,
        ty: Ty<'tcx>,
    ) -> ! {
        log::debug!("::QirCodegenCompiler::LayoutOfHelpers handle_layout_err");
        todo!()
    }
}

impl<'tcx> HasParamEnv<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn param_env(&self) -> rustc_middle::ty::ParamEnv<'tcx> {
        log::debug!("::QirCodegenCompiler::HasParamEnv param_env");
        todo!()
    }
}

impl HasDataLayout for QirCodegenCompiler<'_, '_> {
    fn data_layout(&self) -> &rustc_abi::TargetDataLayout {
        log::debug!("::QirCodegenCompiler::HasDataLayout data_layout");
        todo!()
    }
}

impl<'tcx> LayoutTypeMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods backend_type");
        todo!()
    }

    fn cast_backend_type(&self, ty: &rustc_target::abi::call::CastTarget) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods cast_backend_type");
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods fn_decl_backend_type");
        todo!()
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods fn_ptr_backend_type");
        todo!()
    }

    fn reg_backend_type(&self, ty: &rustc_target::abi::call::Reg) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods reg_backend_type");
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods immediate_backend_type");
        todo!()
    }

    fn is_backend_immediate(&self, layout: TyAndLayout<'tcx>) -> bool {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods is_backend_immediate");
        todo!()
    }

    fn is_backend_scalar_pair(&self, layout: TyAndLayout<'tcx>) -> bool {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods is_backend_scalar_pair");
        todo!()
    }

    fn backend_field_index(&self, layout: TyAndLayout<'tcx>, index: usize) -> u64 {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods backend_field_index");
        todo!()
    }

    fn scalar_pair_element_backend_type(
        &self,
        layout: TyAndLayout<'tcx>,
        index: usize,
        immediate: bool,
    ) -> Self::Type {
        log::debug!("::QirCodegenCompiler::LayoutTypeMethods scalar_pair_element_backend_type");
        todo!()
    }
}

impl<'tcx> BaseTypeMethods<'tcx> for QirCodegenCompiler<'tcx, '_> {
    fn type_i1(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_i1");
        todo!()
    }

    fn type_i8(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_i8");
        todo!()
    }

    fn type_i16(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_i16");
        todo!()
    }

    fn type_i32(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_i32");
        todo!()
    }

    fn type_i64(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_i64");
        todo!()
    }

    fn type_i128(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_i128");
        todo!()
    }

    fn type_isize(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_isize");
        todo!()
    }

    fn type_f32(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_f32");
        todo!()
    }

    fn type_f64(&self) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_f64");
        todo!()
    }

    fn type_array(&self, ty: Self::Type, len: u64) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_array");
        todo!()
    }

    fn type_func(&self, args: &[Self::Type], ret: Self::Type) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_func");
        todo!()
    }

    fn type_struct(&self, els: &[Self::Type], packed: bool) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_struct");
        todo!()
    }

    fn type_kind(&self, ty: Self::Type) -> rustc_codegen_ssa::common::TypeKind {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_kind");
        todo!()
    }

    fn type_ptr_to(&self, ty: Self::Type) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_ptr_to");
        todo!()
    }

    fn type_ptr_to_ext(
        &self,
        ty: Self::Type,
        address_space: rustc_abi::AddressSpace,
    ) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods type_ptr_to_ext");
        todo!()
    }

    fn element_type(&self, ty: Self::Type) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods element_type");
        todo!()
    }

    fn vector_length(&self, ty: Self::Type) -> usize {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods vector_length");
        todo!()
    }

    fn float_width(&self, ty: Self::Type) -> usize {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods float_width");
        todo!()
    }

    fn int_width(&self, ty: Self::Type) -> u64 {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods int_width");
        todo!()
    }

    fn val_ty(&self, v: Self::Value) -> Self::Type {
        log::debug!("::QirCodegenCompiler::BaseTypeMethods val_ty");
        todo!()
    }
}
