
use rustc::lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};

declare_lint!(VERIFY_WEBIDL, Deny, "Verify inheritance of WebIDL objects");

pub struct WebIdlPass;

impl WebIdlPass{
	pub fn new() -> WebIdlPass{
		WebIdlPass
	}
}

impl LintPass for WebIdlParser{
	get_lints(&self) -> LintArray {
		lint_array!(VERIFY_WEBIDL)
	}
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for WebIdlPass {
	
	check_struct_def(&mut self,
                        cx: &LateContext,
                        def: &hir::VariantData,
                        _n: ast::Name,
                        _gen: &hir::Generics,
                        id: ast::NodeId) {
		//check for the existance of webidl attribute

		//if the struct does not have this attribute we should not check
		if !cx.tcx.has_attr(did.did, "webidl") {
        	return;   
        }

		//open the corresponding webidl file


		//parse the webidl file

		//determine the parent interface of the matching interface that has been parsed

		//verify that the first field of the rust structure matches that interface name
		//if it does not, use span_lint to give a compiler warning
		//cx.lint_span(VERIFY_WEBIDL, it.span, "First field of dom_struct does not match parent interface name");
	}

	check_struct_field() -> {

	}

}