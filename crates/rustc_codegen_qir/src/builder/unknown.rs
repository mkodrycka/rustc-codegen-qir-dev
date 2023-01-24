// This file contains required implementations for Builder
// that are not yet known. As the below methods are investigated and
// implemented, they should be moved to the corresponding source file
// in which the implementation belongs and thoroughly commented.

use rustc_codegen_ssa::traits::{
    AbiBuilderMethods, ArgAbiMethods, AsmBuilderMethods, BuilderMethods,
    CoverageInfoBuilderMethods, DebugInfoBuilderMethods, IntrinsicCallMethods,
    StaticBuilderMethods,
};
use rustc_middle::ty::{
    layout::{FnAbiOfHelpers, LayoutOfHelpers, TyAndLayout},
    Ty,
};
use rustc_target::abi::call::FnAbi;

use super::QirBuilder;

impl<'tcx> LayoutOfHelpers<'tcx> for QirBuilder<'_, 'tcx, '_> {
    type LayoutOfResult = TyAndLayout<'tcx>;

    #[inline]
    fn handle_layout_err(
        &self,
        err: rustc_middle::ty::layout::LayoutError<'tcx>,
        span: rustc_span::Span,
        ty: Ty<'tcx>,
    ) -> ! {
        log::debug!("::QirBuilder::LayoutOfHelpers handle_layout_err");
        todo!()
    }
}

// We ignore FnAbi since QIR does not support interacting with other QIR code through ABIs
// TODO: Do we actually need to support ABIs if we want to call QIR intrinsics?
impl<'tcx> FnAbiOfHelpers<'tcx> for QirBuilder<'_, 'tcx, '_> {
    type FnAbiOfResult = &'tcx FnAbi<'tcx, Ty<'tcx>>;

    #[inline]
    fn handle_fn_abi_err(
        &self,
        err: rustc_middle::ty::layout::FnAbiError<'tcx>,
        span: rustc_span::Span,
        fn_abi_request: rustc_middle::ty::layout::FnAbiRequest<'tcx>,
    ) -> ! {
        self.compiler.handle_fn_abi_err(err, span, fn_abi_request)
    }
}

impl StaticBuilderMethods for QirBuilder<'_, '_, '_> {
    fn get_static(&mut self, def_id: rustc_hir::def_id::DefId) -> Self::Value {
        log::debug!("::QirBuilder::StaticBuilderMethods get_static");
        todo!()
    }
}

impl<'tcx> AsmBuilderMethods<'tcx> for QirBuilder<'_, 'tcx, '_> {
    fn codegen_inline_asm(
        &mut self,
        template: &[rustc_ast::InlineAsmTemplatePiece],
        operands: &[rustc_codegen_ssa::traits::InlineAsmOperandRef<'tcx, Self>],
        options: rustc_ast::InlineAsmOptions,
        line_spans: &[rustc_span::Span],
        instance: rustc_middle::ty::Instance<'_>,
        dest_catch_funclet: Option<(Self::BasicBlock, Self::BasicBlock, Option<&Self::Funclet>)>,
    ) {
        log::debug!("::QirBuilder::AsmBuilderMethods codegen_inline_asm");
        todo!()
    }
}

impl<'tcx> IntrinsicCallMethods<'tcx> for QirBuilder<'_, 'tcx, '_> {
    fn codegen_intrinsic_call(
        &mut self,
        instance: rustc_middle::ty::Instance<'tcx>,
        fn_abi: &FnAbi<'tcx, Ty<'tcx>>,
        args: &[rustc_codegen_ssa::mir::operand::OperandRef<'tcx, Self::Value>],
        llresult: Self::Value,
        span: rustc_span::Span,
    ) {
        log::debug!("::QirBuilder::IntrinsicCallMethods codegen_intrinsic_call");
        todo!()
    }

    fn abort(&mut self) {
        log::debug!("::QirBuilder::IntrinsicCallMethods abort");
        todo!()
    }

    fn assume(&mut self, val: Self::Value) {
        log::debug!("::QirBuilder::IntrinsicCallMethods assume");
        todo!()
    }

    fn expect(&mut self, cond: Self::Value, expected: bool) -> Self::Value {
        log::debug!("::QirBuilder::IntrinsicCallMethods expect");
        todo!()
    }

    fn type_test(&mut self, pointer: Self::Value, typeid: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::IntrinsicCallMethods type_test");
        todo!()
    }

    fn type_checked_load(
        &mut self,
        llvtable: Self::Value,
        vtable_byte_offset: u64,
        typeid: Self::Value,
    ) -> Self::Value {
        log::debug!("::QirBuilder::IntrinsicCallMethods type_checked_load");
        todo!()
    }

    fn va_start(&mut self, val: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::IntrinsicCallMethods va_start");
        todo!()
    }

    fn va_end(&mut self, val: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::IntrinsicCallMethods va_end");
        todo!()
    }
}

impl<'tcx> AbiBuilderMethods<'tcx> for QirBuilder<'_, 'tcx, '_> {
    fn get_param(&mut self, index: usize) -> Self::Value {
        log::debug!("::QirBuilder::AbiBuilderMethods get_param");
        todo!()
    }
}

impl<'tcx> ArgAbiMethods<'tcx> for QirBuilder<'_, 'tcx, '_> {
    fn store_fn_arg(
        &mut self,
        arg_abi: &rustc_target::abi::call::ArgAbi<'tcx, Ty<'tcx>>,
        idx: &mut usize,
        dst: rustc_codegen_ssa::mir::place::PlaceRef<'tcx, Self::Value>,
    ) {
        log::debug!("::QirBuilder::ArgAbiMethods store_fn_arg");
        todo!()
    }

    fn store_arg(
        &mut self,
        arg_abi: &rustc_target::abi::call::ArgAbi<'tcx, Ty<'tcx>>,
        val: Self::Value,
        dst: rustc_codegen_ssa::mir::place::PlaceRef<'tcx, Self::Value>,
    ) {
        log::debug!("::QirBuilder::ArgAbiMethods store_arg");
        todo!()
    }

    fn arg_memory_ty(
        &self,
        arg_abi: &rustc_target::abi::call::ArgAbi<'tcx, Ty<'tcx>>,
    ) -> Self::Type {
        log::debug!("::QirBuilder::ArgAbiMethods arg_memory_ty");
        todo!()
    }
}

impl DebugInfoBuilderMethods for QirBuilder<'_, '_, '_> {
    fn dbg_var_addr(
        &mut self,
        dbg_var: Self::DIVariable,
        dbg_loc: Self::DILocation,
        variable_alloca: Self::Value,
        direct_offset: rustc_abi::Size,
        // NB: each offset implies a deref (i.e. they're steps in a pointer chain).
        indirect_offsets: &[rustc_abi::Size],
        // Byte range in the `dbg_var` covered by this fragment,
        // if this is a fragment of a composite `DIVariable`.
        fragment: Option<std::ops::Range<rustc_abi::Size>>,
    ) {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods dbg_var_addr");
        todo!()
    }

    fn set_dbg_loc(&mut self, dbg_loc: Self::DILocation) {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods set_dbg_loc");
        todo!()
    }

    fn insert_reference_to_gdb_debug_scripts_section_global(&mut self) {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods insert_reference_to_gdb_debug_scripts_section_global");
        todo!()
    }

    fn set_var_name(&mut self, value: Self::Value, name: &str) {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods set_var_name");
        todo!()
    }
}

impl<'tcx> CoverageInfoBuilderMethods<'tcx> for QirBuilder<'_, '_, '_> {
    fn set_function_source_hash(
        &mut self,
        instance: rustc_middle::ty::Instance<'tcx>,
        function_source_hash: u64,
    ) -> bool {
        log::debug!("::QirBuilder::CoverageInfoBuilderMethods set_function_source_hash");
        todo!()
    }

    fn add_coverage_counter(
        &mut self,
        instance: rustc_middle::ty::Instance<'tcx>,
        index: rustc_middle::mir::coverage::CounterValueReference,
        region: rustc_middle::mir::coverage::CodeRegion,
    ) -> bool {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods add_coverage_counter");
        todo!()
    }

    fn add_coverage_counter_expression(
        &mut self,
        instance: rustc_middle::ty::Instance<'tcx>,
        id: rustc_middle::mir::coverage::InjectedExpressionId,
        lhs: rustc_middle::mir::coverage::ExpressionOperandId,
        op: rustc_middle::mir::coverage::Op,
        rhs: rustc_middle::mir::coverage::ExpressionOperandId,
        region: Option<rustc_middle::mir::coverage::CodeRegion>,
    ) -> bool {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods add_coverage_counter_expression");
        todo!()
    }

    fn add_coverage_unreachable(
        &mut self,
        instance: rustc_middle::ty::Instance<'tcx>,
        region: rustc_middle::mir::coverage::CodeRegion,
    ) -> bool {
        log::debug!("::QirBuilder::DebugInfoBuilderMethods add_coverage_unreachable");
        todo!()
    }
}

impl<'a, 'tcx> BuilderMethods<'a, 'tcx> for QirBuilder<'a, 'tcx, '_> {
    fn build(cx: &'a Self::CodegenCx, llbb: Self::BasicBlock) -> Self {
        log::debug!("::QirBuilder::BuilderMethods build");
        todo!()
    }

    fn cx(&self) -> &Self::CodegenCx {
        log::debug!("::QirBuilder::BuilderMethods cx");
        todo!()
    }

    fn llbb(&self) -> Self::BasicBlock {
        log::debug!("::QirBuilder::BuilderMethods libb");
        todo!()
    }

    fn set_span(&mut self, span: rustc_span::Span) {
        log::debug!("::QirBuilder::BuilderMethods set_span");
        todo!()
    }

    fn append_block(cx: &'a Self::CodegenCx, llfn: Self::Function, name: &str) -> Self::BasicBlock {
        log::debug!("::QirBuilder::BuilderMethods append_block");
        todo!()
    }

    fn append_sibling_block(&mut self, name: &str) -> Self::BasicBlock {
        log::debug!("::QirBuilder::BuilderMethods append_sibling_block");
        todo!()
    }

    fn switch_to_block(&mut self, llbb: Self::BasicBlock) {
        log::debug!("::QirBuilder::BuilderMethods switch_to_block");
        todo!()
    }

    fn ret_void(&mut self) {
        log::debug!("::QirBuilder::BuilderMethods ret_void");
        todo!()
    }

    fn ret(&mut self, v: Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods ret");
        todo!()
    }

    fn br(&mut self, dest: Self::BasicBlock) {
        log::debug!("::QirBuilder::BuilderMethods br");
        todo!()
    }

    fn cond_br(
        &mut self,
        cond: Self::Value,
        then_llbb: Self::BasicBlock,
        else_llbb: Self::BasicBlock,
    ) {
        log::debug!("::QirBuilder::BuilderMethods cond_br");
        todo!()
    }

    fn switch(
        &mut self,
        v: Self::Value,
        else_llbb: Self::BasicBlock,
        cases: impl ExactSizeIterator<Item = (u128, Self::BasicBlock)>,
    ) {
        log::debug!("::QirBuilder::BuilderMethods switch");
        todo!()
    }

    fn invoke(
        &mut self,
        llty: Self::Type,
        fn_abi: Option<&FnAbi<'tcx, Ty<'tcx>>>,
        llfn: Self::Value,
        args: &[Self::Value],
        then: Self::BasicBlock,
        catch: Self::BasicBlock,
        funclet: Option<&Self::Funclet>,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods invoke");
        todo!()
    }

    fn unreachable(&mut self) {
        log::debug!("::QirBuilder::BuilderMethods unreachable");
        todo!()
    }

    fn add(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods add");
        todo!()
    }

    fn fadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fadd");
        todo!()
    }

    fn fadd_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fadd_fast");
        todo!()
    }

    fn sub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods sub");
        todo!()
    }

    fn fsub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fsub");
        todo!()
    }

    fn fsub_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fsub_fast");
        todo!()
    }

    fn mul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods mul");
        todo!()
    }

    fn fmul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fmul");
        todo!()
    }

    fn fmul_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fmul_fast");
        todo!()
    }

    fn udiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods udiv");
        todo!()
    }

    fn exactudiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods exactudiv");
        todo!()
    }

    fn sdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods sdiv");
        todo!()
    }

    fn exactsdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods exactsdiv");
        todo!()
    }

    fn fdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fdiv");
        todo!()
    }

    fn fdiv_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fdiv_fast");
        todo!()
    }

    fn urem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods urem");
        todo!()
    }

    fn srem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods srem");
        todo!()
    }

    fn frem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods frem");
        todo!()
    }

    fn frem_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods frem_fast");
        todo!()
    }

    fn shl(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods shl");
        todo!()
    }

    fn lshr(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods lshr");
        todo!()
    }

    fn ashr(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods ashr");
        todo!()
    }

    fn unchecked_sadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods unchecked_sadd");
        todo!()
    }

    fn unchecked_uadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods unchecked_uadd");
        todo!()
    }

    fn unchecked_ssub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods unchecked_ssub");
        todo!()
    }

    fn unchecked_usub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods unchecked_usub");
        todo!()
    }

    fn unchecked_smul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods unchecked_smul");
        todo!()
    }

    fn unchecked_umul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods unchecked_umul");
        todo!()
    }

    fn and(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods and");
        todo!()
    }

    fn or(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods or");
        todo!()
    }

    fn xor(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods xor");
        todo!()
    }

    fn neg(&mut self, v: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods neg");
        todo!()
    }

    fn fneg(&mut self, v: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fneg");
        todo!()
    }

    fn not(&mut self, v: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods not");
        todo!()
    }

    fn checked_binop(
        &mut self,
        oop: rustc_codegen_ssa::traits::OverflowOp,
        ty: Ty<'_>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> (Self::Value, Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods checked_binop");
        todo!()
    }

    fn from_immediate(&mut self, val: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods from_immediate");
        todo!()
    }

    fn to_immediate_scalar(&mut self, val: Self::Value, scalar: rustc_abi::Scalar) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods to_immediate_scalar");
        todo!()
    }

    fn alloca(&mut self, ty: Self::Type, align: rustc_abi::Align) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods alloca");
        todo!()
    }

    fn byte_array_alloca(&mut self, len: Self::Value, align: rustc_abi::Align) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods byte_array_alloca");
        todo!()
    }

    fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: rustc_abi::Align) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods load");
        todo!()
    }

    fn volatile_load(&mut self, ty: Self::Type, ptr: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods volatile_load");
        todo!()
    }

    fn atomic_load(
        &mut self,
        ty: Self::Type,
        ptr: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        size: rustc_abi::Size,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods atomic_load");
        todo!()
    }

    fn load_operand(
        &mut self,
        place: rustc_codegen_ssa::mir::place::PlaceRef<'tcx, Self::Value>,
    ) -> rustc_codegen_ssa::mir::operand::OperandRef<'tcx, Self::Value> {
        log::debug!("::QirBuilder::BuilderMethods load_operand");
        todo!()
    }

    fn write_operand_repeatedly(
        &mut self,
        elem: rustc_codegen_ssa::mir::operand::OperandRef<'tcx, Self::Value>,
        count: u64,
        dest: rustc_codegen_ssa::mir::place::PlaceRef<'tcx, Self::Value>,
    ) {
        log::debug!("::QirBuilder::BuilderMethods write_operand_repeatedly");
        todo!()
    }

    fn range_metadata(&mut self, load: Self::Value, range: rustc_abi::WrappingRange) {
        log::debug!("::QirBuilder::BuilderMethods range_metadata");
        todo!()
    }

    fn nonnull_metadata(&mut self, load: Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods nonnull_metadata");
        todo!()
    }

    fn store(
        &mut self,
        val: Self::Value,
        ptr: Self::Value,
        align: rustc_abi::Align,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods store");
        todo!()
    }

    fn store_with_flags(
        &mut self,
        val: Self::Value,
        ptr: Self::Value,
        align: rustc_abi::Align,
        flags: rustc_codegen_ssa::MemFlags,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods store_with_flags");
        todo!()
    }

    fn atomic_store(
        &mut self,
        val: Self::Value,
        ptr: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        size: rustc_abi::Size,
    ) {
        log::debug!("::QirBuilder::BuilderMethods atomic_store");
        todo!()
    }

    fn gep(&mut self, ty: Self::Type, ptr: Self::Value, indices: &[Self::Value]) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods gep");
        todo!()
    }

    fn inbounds_gep(
        &mut self,
        ty: Self::Type,
        ptr: Self::Value,
        indices: &[Self::Value],
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods inbounds_gep");
        todo!()
    }

    fn struct_gep(&mut self, ty: Self::Type, ptr: Self::Value, idx: u64) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods struct_gep");
        todo!()
    }

    fn trunc(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods trunc");
        todo!()
    }

    fn sext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods sext");
        todo!()
    }

    fn fptoui_sat(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fptoui_sat");
        todo!()
    }

    fn fptosi_sat(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fptosi_sat");
        todo!()
    }

    fn fptoui(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fptoui");
        todo!()
    }

    fn fptosi(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fptosi");
        todo!()
    }

    fn uitofp(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods uitofp");
        todo!()
    }

    fn sitofp(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods sitofp");
        todo!()
    }

    fn fptrunc(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fptrunc");
        todo!()
    }

    fn fpext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fpext");
        todo!()
    }

    fn ptrtoint(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods ptrtoint");
        todo!()
    }

    fn inttoptr(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods inttoptr");
        todo!()
    }

    fn bitcast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods bitcast");
        todo!()
    }

    fn intcast(&mut self, val: Self::Value, dest_ty: Self::Type, is_signed: bool) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods intcast");
        todo!()
    }

    fn pointercast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods pointercast");
        todo!()
    }

    fn icmp(
        &mut self,
        op: rustc_codegen_ssa::common::IntPredicate,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods icmp");
        todo!()
    }

    fn fcmp(
        &mut self,
        op: rustc_codegen_ssa::common::RealPredicate,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods fcmp");
        todo!()
    }

    fn memcpy(
        &mut self,
        dst: Self::Value,
        dst_align: rustc_abi::Align,
        src: Self::Value,
        src_align: rustc_abi::Align,
        size: Self::Value,
        flags: rustc_codegen_ssa::MemFlags,
    ) {
        log::debug!("::QirBuilder::BuilderMethods memcpy");
        todo!()
    }

    fn memmove(
        &mut self,
        dst: Self::Value,
        dst_align: rustc_abi::Align,
        src: Self::Value,
        src_align: rustc_abi::Align,
        size: Self::Value,
        flags: rustc_codegen_ssa::MemFlags,
    ) {
        log::debug!("::QirBuilder::BuilderMethods memmove");
        todo!()
    }

    fn memset(
        &mut self,
        ptr: Self::Value,
        fill_byte: Self::Value,
        size: Self::Value,
        align: rustc_abi::Align,
        flags: rustc_codegen_ssa::MemFlags,
    ) {
        log::debug!("::QirBuilder::BuilderMethods memset");
        todo!()
    }

    fn select(
        &mut self,
        cond: Self::Value,
        then_val: Self::Value,
        else_val: Self::Value,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods select");
        todo!()
    }

    fn va_arg(&mut self, list: Self::Value, ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods va_arg");
        todo!()
    }

    fn extract_element(&mut self, vec: Self::Value, idx: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods extract_element");
        todo!()
    }

    fn vector_splat(&mut self, num_elts: usize, elt: Self::Value) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods vector_splat");
        todo!()
    }

    fn extract_value(&mut self, agg_val: Self::Value, idx: u64) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods extract_value");
        todo!()
    }

    fn insert_value(&mut self, agg_val: Self::Value, elt: Self::Value, idx: u64) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods insert_value");
        todo!()
    }

    fn set_personality_fn(&mut self, personality: Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods set_personality_fn");
        todo!()
    }

    fn cleanup_landing_pad(&mut self, pers_fn: Self::Value) -> (Self::Value, Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods cleanup_landing_pad");
        todo!()
    }

    fn resume(&mut self, exn0: Self::Value, exn1: Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods resume");
        todo!()
    }

    fn cleanup_pad(&mut self, parent: Option<Self::Value>, args: &[Self::Value]) -> Self::Funclet {
        log::debug!("::QirBuilder::BuilderMethods cleanip_pad");
        todo!()
    }

    fn cleanup_ret(&mut self, funclet: &Self::Funclet, unwind: Option<Self::BasicBlock>) {
        log::debug!("::QirBuilder::BuilderMethods cleanup_ret");
        todo!()
    }

    fn catch_pad(&mut self, parent: Self::Value, args: &[Self::Value]) -> Self::Funclet {
        log::debug!("::QirBuilder::BuilderMethods catch_pad");
        todo!()
    }

    fn catch_switch(
        &mut self,
        parent: Option<Self::Value>,
        unwind: Option<Self::BasicBlock>,
        handlers: &[Self::BasicBlock],
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods catch_switch");
        todo!()
    }

    fn atomic_cmpxchg(
        &mut self,
        dst: Self::Value,
        cmp: Self::Value,
        src: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        failure_order: rustc_codegen_ssa::common::AtomicOrdering,
        weak: bool,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods atomic_cmpxchg");
        todo!()
    }

    fn atomic_rmw(
        &mut self,
        op: rustc_codegen_ssa::common::AtomicRmwBinOp,
        dst: Self::Value,
        src: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods atomic_raw");
        todo!()
    }

    fn atomic_fence(
        &mut self,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        scope: rustc_codegen_ssa::common::SynchronizationScope,
    ) {
        log::debug!("::QirBuilder::BuilderMethods atomic_fence");
        todo!()
    }

    fn set_invariant_load(&mut self, load: Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods set_invariant_load");
        todo!()
    }

    fn lifetime_start(&mut self, ptr: Self::Value, size: rustc_abi::Size) {
        log::debug!("::QirBuilder::BuilderMethods lifetime_start");
        todo!()
    }

    fn lifetime_end(&mut self, ptr: Self::Value, size: rustc_abi::Size) {
        log::debug!("::QirBuilder::BuilderMethods lifetime_end");
        todo!()
    }

    fn instrprof_increment(
        &mut self,
        fn_name: Self::Value,
        hash: Self::Value,
        num_counters: Self::Value,
        index: Self::Value,
    ) {
        log::debug!("::QirBuilder::BuilderMethods instrprof_increment");
        todo!()
    }

    fn call(
        &mut self,
        llty: Self::Type,
        fn_abi: Option<&FnAbi<'tcx, Ty<'tcx>>>,
        llfn: Self::Value,
        args: &[Self::Value],
        funclet: Option<&Self::Funclet>,
    ) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods call");
        todo!()
    }

    fn zext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        log::debug!("::QirBuilder::BuilderMethods zext");
        todo!()
    }

    fn do_not_inline(&mut self, llret: Self::Value) {
        log::debug!("::QirBuilder::BuilderMethods do_not_inline");
        todo!()
    }
}
