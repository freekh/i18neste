use swc_core::common::chain;
use swc_core::common::Mark;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::CallExpr;
use swc_core::ecma::ast::Decl;
use swc_core::ecma::ast::ExportDecl;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::ExprOrSpread;
use swc_core::ecma::ast::Ident;
use swc_core::ecma::ast::ImportDecl;
use swc_core::ecma::ast::ImportDefaultSpecifier;
use swc_core::ecma::ast::ImportSpecifier;
use swc_core::ecma::ast::MemberExpr;
use swc_core::ecma::ast::MemberProp;
use swc_core::ecma::ast::Module;
use swc_core::ecma::ast::ModuleDecl;
use swc_core::ecma::ast::ModuleItem;
use swc_core::ecma::ast::Pat;
use swc_core::ecma::ast::Str;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::testing::test;
use swc_core::ecma::utils::ExprFactory;
use swc_core::ecma::visit::as_folder;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_core::{
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use tracing::info;

const PACKAGE_NAME: &str = "i18neste";
const IMPORT_SPECIFIER: &str = "__plugin_i18neste_module_";
const SET_STATIC_STATE_DECORATOR: &str = "setServerSideI18NesteState";
const GET_SERVER_SIDE_PROPS_FUNCTION: &str = "getServerSideProps";
const GET_STATIC_PROPS_FUNCTION: &str = "getStaticProps";

pub struct I18NesteVisitor;

impl VisitMut for I18NesteVisitor {
    fn visit_mut_module(&mut self, n: &mut Module) {
        info!("Visiting module");
        n.visit_mut_children_with(self);
        let i18neste_package_name: Str = PACKAGE_NAME.into();
        let import_decl = ImportDecl {
            span: DUMMY_SP,
            specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                span: DUMMY_SP,
                local: Ident::new(IMPORT_SPECIFIER.into(), DUMMY_SP),
            })],
            src: i18neste_package_name,
            type_only: false,
            asserts: None,
        };
        n.body
            .insert(0, ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)));
    }

    fn visit_mut_export_decl(&mut self, n: &mut ExportDecl) {
        n.visit_mut_children_with(self);
        if let Decl::Var(decl) = &mut n.decl {
            for var_declarator in &mut decl.decls {
                if let Some(current_expr) = &var_declarator.init {
                    if let Pat::Ident(maybe_name) = &var_declarator.name {
                        if &*maybe_name.id.sym == GET_SERVER_SIDE_PROPS_FUNCTION
                            || &*maybe_name.id.sym == GET_STATIC_PROPS_FUNCTION
                        {
                            var_declarator.init =
                                create_decorator(SET_STATIC_STATE_DECORATOR, current_expr);
                        }
                    }
                }
            }
        }
    }
}

fn create_decorator(name: &str, current_expr: &Box<Expr>) -> Option<Box<Expr>> {
    let neste_decorator = Some(Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new(IMPORT_SPECIFIER.into(), DUMMY_SP))),
            prop: MemberProp::Ident(Ident::new(name.into(), DUMMY_SP)),
        })
        .as_callee(),
        args: vec![ExprOrSpread {
            spread: None,
            expr: current_expr.clone(),
        }],
        type_args: None,
    })));
    neste_decorator
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(I18NesteVisitor))
}
// fn tr() -> impl Fold {
//     chain!(
//         resolver(Mark::new(), Mark::new(), false),
//         // Most of transform does not care about globals so it does not need `SyntaxContext`
//         your_transform()
//     )
// }

// test!(
//     Default::default(),
//     |_| as_folder(I18NesteVisitor),
//     inject_set_neste_static_state_into_get_server_side_props,
//     // Input codes
//     r#"
//     import React from 'react';
//     import { useTranslation } from 'i18neste';

//     export const getServerSideProps = async () => {
//         return {
//             props: {
//                 foo: 'bar',
//             },
//         };
//     };
//         "#,
//     // Output codes after transformed with plugin
//     r#"import __plugin_i18neste_module_ from "i18neste";
//     import React from 'react';
//     import { useTranslation } from 'i18neste';

//     export const getServerSideProps = __plugin_i18neste_module_.setServerSideI18NesteState(async () => {
//         return {
//             props: {
//                 foo: 'bar',
//             },
//         };
//     });
//     "#
// );

// test!(
//     Default::default(),
//     |_| as_folder(I18NesteVisitor),
//     inject_set_neste_static_state_into_get_static_props,
//     // Input codes
//     r#"
//     import React from 'react';
//     import { useTranslation } from 'i18neste';

//     export const getStaticProps = async () => {
//         return {
//             foo: 'bar',
//         };
//     };
//         "#,
//     // Output codes after transformed with plugin
//     r#"import __plugin_i18neste_module_ from "i18neste";
//     import React from 'react';
//     import { useTranslation } from 'i18neste';

//     export const getStaticProps = __plugin_i18neste_module_.setServerSideI18nesteState(async () => {
//         return {
//             foo: 'bar',
//         };
//     });
//     "#
// );
