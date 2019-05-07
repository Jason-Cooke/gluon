use support::*;

mod support;

use gluon::{self, query::Compilation, vm::core::tests::check_expr_eq, ThreadExt};

#[test]
fn inline_cross_module() {
    let _ = env_logger::try_init();

    let thread = make_vm();
    thread.get_database_mut().set_implicit_prelude(false);

    gluon::Compiler::new()
        .implicit_prelude(false)
        .load_script(
            &thread,
            "test",
            r#"
        let { (+) } = import! std.num
        let { ? } = import! std.int
        1 + 2
    "#,
        )
        .unwrap_or_else(|err| panic!("{}", err));

    let db = thread.get_database();
    let core_expr = db
        .core_expr("test".into())
        .unwrap_or_else(|err| panic!("{}", err));
    let expected_str = r#"
        match std_num with
        | { (+) } ->
            let implicit = std_int
            in
            match std_int with
            | { } ->
                (#Int+) 1 2
            end
        end
    "#;
    check_expr_eq(core_expr.expr(), expected_str);
}
