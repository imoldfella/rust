#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;


struct HirPrinter;


impl rustc_driver::Callbacks for HirPrinter {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let crate_items = tcx.hir_crate_items(());
        crate_items
            .par_items(|item_id| {
                let def_id = item_id.owner_id.def_id;
                let item = tcx.hir_expect_item(def_id);
                println!("Item: {:?}", item);
                Ok(())
            })
            .unwrap();
        rustc_driver::Compilation::Continue
    }
}

pub fn main_with_args(args: Vec<String>) {
    let mut callbacks = HirPrinter;
    rustc_driver::run_compiler(&args, &mut callbacks);
}
