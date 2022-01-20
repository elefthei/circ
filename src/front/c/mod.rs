//! The C front-end

mod ast_utils;
mod parser;
mod term;
mod types;

use super::{FrontEnd, Mode};
use crate::circify::{Circify, Loc, Val};
use crate::front::c::ast_utils::*;
use crate::front::c::term::*;
use crate::front::c::types::*;
use crate::ir::opt::cfold::fold;
use crate::ir::proof::{self, ConstraintMetadata};
use crate::ir::term::*;
use lang_c::ast::*;
use lang_c::span::Node;
use log::debug;

// use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

/// The prover visibility
const PROVER_VIS: Option<PartyId> = Some(proof::PROVER_ID);
/// Public visibility
const PUBLIC_VIS: Option<PartyId> = None;

/// Inputs to the C compiler
pub struct Inputs {
    /// The file to look for `main` in.
    pub file: PathBuf,
    /// The file to look for concrete arguments to main in. Optional.
    ///
    /// ## Examples
    ///
    /// If main takes `x: u64, y: field`, this file might contain
    ///
    /// ```ignore
    /// x 4
    /// y -1
    /// ```
    pub inputs: Option<PathBuf>,
    /// The mode to generate for (MPC or proof). Effects visibility.
    pub mode: Mode,
}

/// The C front-end. Implements [FrontEnd].
pub struct C;

impl FrontEnd for C {
    type Inputs = Inputs;
    fn gen(i: Inputs) -> Computation {
        let parser = parser::CParser::new();
        let p = parser.parse_file(&i.file).unwrap();
        let mut g = CGen::new(i.inputs, i.mode, p.unit);
        g.gen();
        g.circ.consume().borrow().clone()
    }
}

struct CGen {
    circ: Circify<Ct>,
    mode: Mode,
    tu: TranslationUnit,
}

enum CLoc {
    Var(Loc),
    Idx(Box<CLoc>, CTerm),
}

impl CLoc {
    fn loc(&self) -> &Loc {
        match self {
            CLoc::Var(l) => l,
            CLoc::Idx(i, _) => i.loc(),
        }
    }
}

impl CGen {
    fn new(inputs: Option<PathBuf>, mode: Mode, tu: TranslationUnit) -> Self {
        let this = Self {
<<<<<<< HEAD
            circ: Circify::new(Ct::new(inputs.map(|i| parser::parse_inputs(i)))),
=======
            circ: Circify::new(Ct::new(inputs.map(parser::parse_inputs))),
>>>>>>> 75572c6... C Frontend (#22)
            mode,
            tu,
        };
        this.circ
            .cir_ctx()
            .cs
            .borrow_mut()
            .metadata
            .add_prover_and_verifier();
        this
    }

    /// Unwrap a result of an error and abort
    fn err<E: Display>(&self, e: E) -> ! {
        println!("Error: {}", e);
        std::process::exit(1)
    }

    /// Unwrap result of a computation
    fn unwrap<CTerm, E: Display>(&self, r: Result<CTerm, E>) -> CTerm {
        r.unwrap_or_else(|e| self.err(e))
    }

    fn array_select(&self, array: CTerm, idx: CTerm) -> Result<CTerm, String> {
        match (array.clone().term, idx.term) {
            (CTermData::CArray(ty, id), CTermData::CInt(_, _, idx)) => {
                let i = id.unwrap_or_else(|| panic!("Unknown AllocID: {:#?}", array));
<<<<<<< HEAD
                Ok(CTerm {
                    term: match ty {
                        Ty::Bool => {
                            CTermData::CBool(self.circ.load(i, idx))
                        }
                        Ty::Int(s,w) => {
                            CTermData::CInt(s, w, self.circ.load(i, idx))
                        } 
                        // TODO: Flatten array so this case doesn't occur
                        // Ty::Array(_,t) => {
                        //     CTermData::CArray(*t, id)
                        // }
                        _ => unimplemented!()
                    }, 
                    udef: false,
                })  
            }
            (a, b) => Err(format!("[Array Select] cannot index {} by {}", b, a))
=======
                Ok(cterm(match ty {
                    Ty::Bool => CTermData::CBool(self.circ.load(i, idx)),
                    Ty::Int(s, w) => CTermData::CInt(s, w, self.circ.load(i, idx)),
                    // TODO: Flatten array so this case doesn't occur
                    // Ty::Array(_,t) => {
                    //     CTermData::CArray(*t, id)
                    // }
                    _ => unimplemented!(),
                }))
            }
            (a, b) => Err(format!("[Array Select] cannot index {} by {}", b, a)),
>>>>>>> 75572c6... C Frontend (#22)
        }
    }

    pub fn array_store(&mut self, array: CTerm, idx: CTerm, val: CTerm) -> Result<CTerm, String> {
        match (array.clone().term, idx.term) {
            (CTermData::CArray(_, id), CTermData::CInt(_, _, idx_term)) => {
                let i = id.unwrap_or_else(|| panic!("Unknown AllocID: {:#?}", array.clone()));
                let new_val = val.term.term(&self.circ);
                self.circ.store(i, idx_term, new_val);
<<<<<<< HEAD
                Ok(val.clone())
=======
                Ok(val)
>>>>>>> 75572c6... C Frontend (#22)
            }
            (a, b) => Err(format!("[Array Store] cannot index {} by {}", b, a)),
        }
    }

    fn mod_lval(&mut self, l: CLoc, t: CTerm) -> Result<CTerm, String> {
        let var = l.loc().clone();
        let old = self
            .circ
            .get_value(var.clone())
            .map_err(|e| format!("{}", e))?
            .unwrap_term();

        match l {
<<<<<<< HEAD
            CLoc::Var(_) => {
                self.circ
                    .assign(var, Val::Term(t.clone()))
                    .map_err(|e| format!("{}", e))
                    .map(|_| t)
            },
            CLoc::Idx(_, offset) => self.array_store(old, offset, t)
        }

    }

    fn lval(&mut self, expr: Box<Node<Expression>>) -> CLoc {
=======
            CLoc::Var(_) => self
                .circ
                .assign(var, Val::Term(t.clone()))
                .map_err(|e| format!("{}", e))
                .map(|_| t),
            CLoc::Idx(_, offset) => self.array_store(old, offset, t),
        }
    }

    fn lval(&mut self, expr: Node<Expression>) -> CLoc {
>>>>>>> 75572c6... C Frontend (#22)
        match expr.node {
            Expression::Identifier(_) => {
                let base_name = name_from_ident(&expr.node);
                CLoc::Var(Loc::local(base_name))
            }
            Expression::BinaryOperator(node) => {
                let bin_op = node.node;
                match bin_op.operator.node {
                    BinaryOperator::Index => {
                        let base_name = name_from_ident(&bin_op.lhs.node);
                        let idx = self.gen_expr(bin_op.rhs.node);
                        CLoc::Idx(Box::new(CLoc::Var(Loc::local(base_name))), idx)
                    }
                    _ => unimplemented!("Invalid left hand value"),
                }
            }
            _ => unimplemented!("Invalid left hand value"),
        }
    }
<<<<<<< HEAD
    
    fn fold_(&mut self, expr: CTerm) -> i64 {
        let term_ = fold(&expr.term.term(&self.circ));
        let cterm_ = CTerm {
            term: CTermData::CInt(true, 64, term_),
            udef: false,
        };
        let val = const_int(cterm_).ok().unwrap();
        val.to_i64().unwrap()
=======

    fn fold_(&mut self, expr: CTerm) -> i32 {
        let term_ = fold(&expr.term.term(&self.circ));
        let cterm_ = cterm(CTermData::CInt(true, 32, term_));
        let val = const_int(cterm_).ok().unwrap();
        val.to_i32().unwrap()
>>>>>>> 75572c6... C Frontend (#22)
    }

    fn inner_derived_type_(&mut self, base_ty: Ty, d: DerivedDeclarator) -> Ty {
        match d {
            DerivedDeclarator::Array(arr) => {
<<<<<<< HEAD
                if let ArraySize::VariableExpression(expr) =
                    &arr.node.size
                {
                    let expr_ = self.gen_expr(expr.node.clone());
                    let size = self.fold_(expr_) as usize;
                    return Ty::Array(
                        Some(size),
                        Box::new(base_ty),
                    )
                } 
                return Ty::Array(
                    None,
                    Box::new(base_ty),
                )
=======
                if let ArraySize::VariableExpression(expr) = &arr.node.size {
                    let expr_ = self.gen_expr(expr.node.clone());
                    let size = self.fold_(expr_) as usize;
                    return Ty::Array(Some(size), Box::new(base_ty));
                }
                Ty::Array(None, Box::new(base_ty))
>>>>>>> 75572c6... C Frontend (#22)
            }
            DerivedDeclarator::Pointer(_ptr) => {
                unimplemented!("pointers not implemented yet");
            }
            _ => panic!("Not implemented: {:#?}", d),
        }
    }

    fn derived_type_(&mut self, base_ty: Ty, derived: Vec<Node<DerivedDeclarator>>) -> Ty {
<<<<<<< HEAD
        if derived.len() == 0 {
=======
        if derived.is_empty() {
>>>>>>> 75572c6... C Frontend (#22)
            return base_ty;
        }
        let mut derived_ty = base_ty.clone();
        for d in derived {
            let next_ty = self.inner_derived_type_(base_ty.clone(), d.node.clone());
            match derived_ty {
<<<<<<< HEAD
                Ty::Array(s,_) => {
                    derived_ty = Ty::Array(s, Box::new(next_ty))
                }
=======
                Ty::Array(s, _) => derived_ty = Ty::Array(s, Box::new(next_ty)),
>>>>>>> 75572c6... C Frontend (#22)
                _ => derived_ty = next_ty,
            }
        }
        derived_ty
    }

<<<<<<< HEAD
    /// Interpret the party association of input parameters 
=======
    /// Interpret the party association of input parameters
>>>>>>> 75572c6... C Frontend (#22)
    pub fn interpret_visibility(&mut self, ext: &DeclarationSpecifier) -> Option<PartyId> {
        if let DeclarationSpecifier::Extension(nodes) = ext {
            assert!(nodes.len() == 1);
            let node = nodes.first().unwrap();
            if let Extension::Attribute(attr) = &node.node {
                let name = &attr.name;
                return match name.node.as_str() {
<<<<<<< HEAD
                    "public" => PUBLIC_VIS.clone(),
=======
                    "public" => PUBLIC_VIS,
>>>>>>> 75572c6... C Frontend (#22)
                    "private" => match self.mode {
                        Mode::Mpc(n_parties) => {
                            assert!(attr.arguments.len() == 1);
                            let arg = attr.arguments.first().unwrap();
                            let cons = self.gen_expr(arg.node.clone());
                            let num_val = const_int(cons).ok()?;
                            if num_val <= n_parties {
                                Some(num_val.to_u8()?)
                            } else {
                                self.err(format!(
                                    "Party number {} greater than the number of parties ({})",
                                    num_val, n_parties
                                ))
                            }
                        }
<<<<<<< HEAD
                        Mode::Proof => {
                            PROVER_VIS.clone()
                        }
=======
                        Mode::Proof => PROVER_VIS,
>>>>>>> 75572c6... C Frontend (#22)
                        _ => unimplemented!("Mode {} is not supported.", self.mode),
                    },
                    _ => panic!("Unknown visibility: {:#?}", name),
                };
            }
        }
        panic!("Bad visibility declaration.");
    }

    fn const_(&self, c: Constant) -> CTerm {
        match c {
            // TODO: move const integer function out to separate function
<<<<<<< HEAD
            Constant::Integer(i) => CTerm {
                term: CTermData::CInt(true, 64, bv_lit(i.number.parse::<i64>().unwrap(), 64)),
                udef: false,
            },
=======
            Constant::Integer(i) => cterm(CTermData::CInt(
                true,
                32,
                bv_lit(i.number.parse::<i32>().unwrap(), 32),
            )),
>>>>>>> 75572c6... C Frontend (#22)
            _ => unimplemented!("Constant {:#?} hasn't been implemented", c),
        }
    }

    fn get_bin_op(&self, op: BinaryOperator) -> fn(CTerm, CTerm) -> Result<CTerm, String> {
        match op {
            BinaryOperator::Plus => add,
            BinaryOperator::AssignPlus => add,
            BinaryOperator::AssignDivide => div,
            BinaryOperator::Minus => sub,
            BinaryOperator::Multiply => mul,
            BinaryOperator::Divide => div,
            BinaryOperator::Equals => eq,
            BinaryOperator::Greater => ugt,
            BinaryOperator::GreaterOrEqual => uge,
            BinaryOperator::Less => ult,
            BinaryOperator::LessOrEqual => ule,
            BinaryOperator::BitwiseAnd => bitand,
            BinaryOperator::BitwiseOr => bitor,
            BinaryOperator::BitwiseXor => bitxor,
            BinaryOperator::LogicalAnd => and,
            BinaryOperator::LogicalOr => or,
            BinaryOperator::Modulo => rem,
<<<<<<< HEAD
            BinaryOperator::ShiftLeft=> shl,
            BinaryOperator::ShiftRight=> shr,
            _ => unimplemented!("BinaryOperator {:#?} hasn't been implemented", op),
        }
    }
/*
=======
            BinaryOperator::ShiftLeft => shl,
            BinaryOperator::ShiftRight => shr,
            _ => unimplemented!("BinaryOperator {:#?} hasn't been implemented", op),
        }
    }

>>>>>>> 75572c6... C Frontend (#22)
    fn get_u_op(&self, op: UnaryOperator) -> fn(CTerm, CTerm) -> Result<CTerm, String> {
        match op {
            UnaryOperator::PostIncrement => add,
            _ => unimplemented!("UnaryOperator {:#?} hasn't been implemented", op),
        }
    }
<<<<<<< HEAD
*/
=======

>>>>>>> 75572c6... C Frontend (#22)
    fn gen_expr(&mut self, expr: Expression) -> CTerm {
        let res = match expr.clone() {
            Expression::Identifier(node) => Ok(self
                .unwrap(self.circ.get_value(Loc::local(node.node.name.clone())))
                .unwrap_term()),
            Expression::Constant(node) => Ok(self.const_(node.node)),
            Expression::BinaryOperator(node) => {
                let bin_op = node.node;
                match bin_op.operator.node {
                    BinaryOperator::Assign => {
                        let e = self.gen_expr(bin_op.rhs.node);
<<<<<<< HEAD
                        let lval = self.lval(bin_op.lhs);
                        let mod_res = self.mod_lval(lval, e.clone());
                        self.unwrap(mod_res);
                        Ok(e)
                    } 
=======
                        let lval = self.lval(*bin_op.lhs);
                        let mod_res = self.mod_lval(lval, e.clone());
                        self.unwrap(mod_res);
                        Ok(e)
                    }
>>>>>>> 75572c6... C Frontend (#22)
                    BinaryOperator::AssignPlus | BinaryOperator::AssignDivide => {
                        let f = self.get_bin_op(bin_op.operator.node);
                        let i = self.gen_expr(bin_op.lhs.node.clone());
                        let rhs = self.gen_expr(bin_op.rhs.node);
                        let e = f(i, rhs).unwrap();
<<<<<<< HEAD
                        let lval = self.lval(bin_op.lhs);
=======
                        let lval = self.lval(*bin_op.lhs);
>>>>>>> 75572c6... C Frontend (#22)
                        let mod_res = self.mod_lval(lval, e.clone());
                        self.unwrap(mod_res);
                        Ok(e)
                    }
                    BinaryOperator::Index => {
                        let a = self.gen_expr(bin_op.lhs.node);
                        let b = self.gen_expr(bin_op.rhs.node);
<<<<<<< HEAD
                        self.array_select(a,b)
                    }
                    _ => {
                        let f = self.get_bin_op(bin_op.operator.node);
                        let mut a = self.gen_expr(bin_op.lhs.node);
                        let mut b = self.gen_expr(bin_op.rhs.node);
                        
                        // TODO: fix hack, const int check for shifting 
                        if f == shl || f == shr {
                            let a_t = fold(&a.term.term(&self.circ));
                            a = CTerm {
                                term: CTermData::CInt(true, 64, a_t),
                                udef: false,
                            };

                            let b_t = fold(&b.term.term(&self.circ));
                            b = CTerm {
                                term: CTermData::CInt(true, 64, b_t),
                                udef: false,
                            };
=======
                        self.array_select(a, b)
                    }
                    _ => {
                        let f = self.get_bin_op(bin_op.operator.node.clone());
                        let mut a = self.gen_expr(bin_op.lhs.node);
                        let mut b = self.gen_expr(bin_op.rhs.node);

                        // TODO: fix hack, const int check for shifting
                        if bin_op.operator.node == BinaryOperator::ShiftLeft
                            || bin_op.operator.node == BinaryOperator::ShiftRight
                        {
                            let a_t = fold(&a.term.term(&self.circ));
                            a = cterm(CTermData::CInt(true, 32, a_t));

                            let b_t = fold(&b.term.term(&self.circ));
                            b = cterm(CTermData::CInt(true, 32, b_t));
>>>>>>> 75572c6... C Frontend (#22)
                        }
                        f(a, b)
                    }
                }
            }
            Expression::UnaryOperator(node) => {
                let u_op = node.node;
                match u_op.operator.node {
                    UnaryOperator::PostIncrement => {
<<<<<<< HEAD
                        //let f = self.get_u_op(u_op.operator.node);
                        let i = self.gen_expr(u_op.operand.node.clone());
                        let one = CTerm {
                            term: CTermData::CInt(true, 64, bv_lit(1, 64)),
                            udef: false
                        };
                        let e = add(i, one).unwrap();
                        let lval = self.lval(u_op.operand);
                        let mod_res = self.mod_lval(lval, e.clone());
                        self.unwrap(mod_res);
                        Ok(e)                        
                    }
                    UnaryOperator::Minus => {
                        let i = self.gen_expr(u_op.operand.node.clone());
                        let zero = CTerm {
                            term: CTermData::CInt(true, 64, bv_lit(0, 64)),
                            udef: false
                        };
                        let one = CTerm {
                            term: CTermData::CInt(true, 64, bv_lit(1, 64)),
                            udef: false
                        };
                        let minusone = sub(zero, one).unwrap();
                        let e = mul(i, minusone).unwrap();
=======
                        let f = self.get_u_op(u_op.operator.node);
                        let i = self.gen_expr(u_op.operand.node.clone());
                        let one = cterm(CTermData::CInt(true, 32, bv_lit(1, 32)));
                        let e = f(i, one).unwrap();
                        let lval = self.lval(*u_op.operand);
                        let mod_res = self.mod_lval(lval, e.clone());
                        self.unwrap(mod_res);
>>>>>>> 75572c6... C Frontend (#22)
                        Ok(e)
                    }
                    _ => unimplemented!("UnaryOperator {:#?} hasn't been implemented", u_op),
                }
            }
            Expression::Cast(node) => {
                let CastExpression {
                    type_name,
                    expression,
                } = node.node;
                let to_ty = s_type_(type_name.node.specifiers);
                let expr = self.gen_expr(expression.node);
                Ok(cast(to_ty, expr))
            }
            _ => unimplemented!("Expr {:#?} hasn't been implemented", expr),
        };
        self.unwrap(res)
    }

    fn gen_init(&mut self, derived_ty: Ty, init: Initializer) -> CTerm {
        match init {
            Initializer::Expression(e) => self.gen_expr(e.node),
            Initializer::List(l) => {
                // TODO: check length of values to initialized number
                let mut values: Vec<CTerm> = Vec::new();
<<<<<<< HEAD
                let inner_type = inner_ty(derived_ty.clone());
                for li in l.clone() {
                    let expr = self.gen_init(inner_type.clone(), li.node.initializer.node.clone());
                    values.push(expr)
                }
                let id = self.circ.zero_allocate(values.len(), 64, num_bits(inner_type.clone()));

                for (i,v) in values.iter().enumerate() {
                    let offset = bv_lit(i, 64);
=======
                let inner_type = derived_ty.inner_ty();
                for li in l {
                    let expr = self.gen_init(inner_type.clone(), li.node.initializer.node.clone());
                    values.push(expr)
                }
                let id = self
                    .circ
                    .zero_allocate(values.len(), 32, inner_type.num_bits());

                for (i, v) in values.iter().enumerate() {
                    let offset = bv_lit(i, 32);
>>>>>>> 75572c6... C Frontend (#22)
                    let v_ = v.term.term(&self.circ);
                    self.circ.store(id, offset, v_);
                }

<<<<<<< HEAD
                CTerm {
                    term: CTermData::CArray(inner_type, Some(id)), 
                    udef: false,
                }
=======
                cterm(CTermData::CArray(inner_type, Some(id)))
>>>>>>> 75572c6... C Frontend (#22)
            }
        }
    }

    fn gen_decl(&mut self, decl: Declaration) -> CTerm {
        let decl_info = get_decl_info(decl.clone());
        let d = decl.declarators.first().unwrap().node.clone();
        let base_ty: Ty = decl_info.ty;
        let derived = &d.declarator.node.derived;
        let derived_ty = self.derived_type_(base_ty, derived.to_vec());
        let expr: CTerm;
        if let Some(init) = d.initializer {
            expr = self.gen_init(derived_ty.clone(), init.node);
        } else {
            expr = match derived_ty {
                Ty::Array(size, ref ty) => {
<<<<<<< HEAD
                    let id = self.circ.zero_allocate(size.unwrap(), 64, num_bits(*ty.clone()));
                    CTerm {
                        term: CTermData::CArray(*ty.clone(), Some(id)), 
                        udef: false,
                    }
                }
                _ => derived_ty.default()
=======
                    let id = self.circ.zero_allocate(size.unwrap(), 32, ty.num_bits());
                    cterm(CTermData::CArray(*ty.clone(), Some(id)))
                }
                _ => derived_ty.default(),
>>>>>>> 75572c6... C Frontend (#22)
            }
        }

        let res = self.circ.declare_init(
            decl_info.name,
            derived_ty.clone(),
<<<<<<< HEAD
            Val::Term(cast(Some(derived_ty.clone()), expr.clone())),
=======
            Val::Term(cast(Some(derived_ty), expr.clone())),
>>>>>>> 75572c6... C Frontend (#22)
        );
        self.unwrap(res);
        expr
    }

<<<<<<< HEAD
    fn get_const_iters(&mut self, for_stmt: ForStatement) -> Option<ConstIteration> {
        let init: Option<ConstIteration> = match for_stmt.initializer.node {
            ForInitializer::Declaration(d) => {
                let decl_info = get_decl_info(d.node.clone());
                let name = decl_info.name; 
                let expr = self.gen_decl(d.node.clone());
                let val = self.fold_(expr);
                Some(
                    ConstIteration {
                        name,
                        val
                    }
                )
            }
            ForInitializer::Expression(e) => {
                if let Expression::BinaryOperator(bin_op) = e.node {
                    let name = name_from_ident(&bin_op.node.lhs.node);
                    let expr = self.gen_expr(bin_op.node.rhs.node);
                    let val = self.fold_(expr.clone());
=======
    //TODO: This function is not quite right because the loop body could modify the iteration variable.
    fn get_const_iters(&mut self, for_stmt: ForStatement) -> ConstIteration {
        let init: Option<ConstIteration> = match for_stmt.initializer.node {
            ForInitializer::Declaration(d) => {
                let expr = self.gen_decl(d.node);
                let val = self.fold_(expr);
                Some(ConstIteration { val })
            }
            ForInitializer::Expression(e) => {
                if let Expression::BinaryOperator(bin_op) = e.node {
                    let expr = self.gen_expr(bin_op.node.rhs.node);
                    let val = self.fold_(expr);
>>>>>>> 75572c6... C Frontend (#22)
                    // let ass_res = self.circ.assign(
                    //     Loc::local(name.clone()),
                    //     Val::Term(expr.clone()),
                    // );
                    // self.unwrap(ass_res);
<<<<<<< HEAD
                    Some(
                        ConstIteration {
                            name,
                            val
                        }
                    )
=======
                    Some(ConstIteration { val })
>>>>>>> 75572c6... C Frontend (#22)
                } else {
                    None
                }
            }
<<<<<<< HEAD
            _ => None
=======
            _ => None,
>>>>>>> 75572c6... C Frontend (#22)
        };

        let cond: Option<ConstIteration> = match for_stmt.condition.unwrap().node {
            Expression::BinaryOperator(bin_op) => {
<<<<<<< HEAD
                let name = name_from_ident(&bin_op.node.lhs.node);
                let expr = self.gen_expr(bin_op.node.rhs.node);
                let val = self.fold_(expr);
                match bin_op.node.operator.node {
                    BinaryOperator::Less => {
                        Some(
                            ConstIteration {
                                name,
                                val: val,
                            }
                        )
                    } 
                    BinaryOperator::LessOrEqual => {
                        Some(
                            ConstIteration {
                                name,
                                val: val + 1,
                            }
                        )
                    }
                    _ => None
                }                        
            }
            _ => None
        };
        
        let step: Option<ConstIteration> = match for_stmt.step.unwrap().node {
            Expression::UnaryOperator(u_op) => {
                let name = name_from_ident(&u_op.node.operand.node);
                match u_op.node.operator.node {
                    UnaryOperator::PostIncrement | UnaryOperator::PreIncrement => {
                        Some(
                            ConstIteration {
                                name,
                                val: 1,
                            }
                        )
                    }
                    _ => None
                }
            }
            Expression::BinaryOperator(bin_op) => {
                let name = name_from_ident(&bin_op.node.lhs.node);
                match bin_op.node.operator.node {
                    BinaryOperator::AssignPlus => {
                        let expr = self.gen_expr(bin_op.node.rhs.node);
                        let val = self.fold_(expr);
                        Some(
                            ConstIteration {
                                name,
                                val,
                            }
                        )
                    }
                    _ => None
                }
            }
            _ => None
=======
                let expr = self.gen_expr(bin_op.node.rhs.node);
                let val = self.fold_(expr);
                match bin_op.node.operator.node {
                    BinaryOperator::Less => Some(ConstIteration { val }),
                    BinaryOperator::LessOrEqual => Some(ConstIteration { val: val + 1 }),
                    _ => None,
                }
            }
            _ => None,
        };

        let step: Option<ConstIteration> = match for_stmt.step.unwrap().node {
            Expression::UnaryOperator(u_op) => match u_op.node.operator.node {
                UnaryOperator::PostIncrement | UnaryOperator::PreIncrement => {
                    Some(ConstIteration { val: 1 })
                }
                _ => None,
            },
            Expression::BinaryOperator(bin_op) => match bin_op.node.operator.node {
                BinaryOperator::AssignPlus => {
                    let expr = self.gen_expr(bin_op.node.rhs.node);
                    let val = self.fold_(expr);
                    Some(ConstIteration { val })
                }
                _ => None,
            },
            _ => None,
>>>>>>> 75572c6... C Frontend (#22)
        };

        // TODO: error checking here
        let init_ = init.unwrap();
        let cond_ = cond.unwrap();
        let incr_ = step.unwrap();

<<<<<<< HEAD
        let init_name = init_.name;
        let start = init_.val;
        let mut cond_name = cond_.name;
        let end = cond_.val;
        let incr_name = incr_.name;
        let incr = incr_.val;

        // TODO: fix naming
        if cond_name == "" {
            cond_name = init_name.clone();
        }

        if init_name == cond_name && cond_name == incr_name {
            return Some(
                ConstIteration {
                    name: init_name,
                    val: ((end - start - 1) / incr) + 1
                }
            );
        }
        None
=======
        let start = init_.val;
        let end = cond_.val;
        let incr = incr_.val;

        ConstIteration {
            val: ((end - start - 1) / incr) + 1,
        }
>>>>>>> 75572c6... C Frontend (#22)
    }

    fn gen_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Compound(nodes) => {
                for node in nodes {
                    match node.node {
                        BlockItem::Declaration(decl) => {
                            self.gen_decl(decl.node);
                        }
                        BlockItem::Statement(stmt) => {
                            self.gen_stmt(stmt.node);
                        }
                        BlockItem::StaticAssert(_sa) => {
                            unimplemented!("Static Assert not supported yet")
                        }
                    }
                }
            }
            Statement::If(node) => {
                let cond = self.gen_expr(node.node.condition.node);
                let t_term = cond.term.term(&self.circ);
                let t_res = self.circ.enter_condition(t_term);
                self.unwrap(t_res);
                self.gen_stmt(node.node.then_statement.node);
                self.circ.exit_condition();
<<<<<<< HEAD
                
=======

>>>>>>> 75572c6... C Frontend (#22)
                if let Some(f_cond) = node.node.else_statement {
                    let f_term = term!(Op::Not; cond.term.term(&self.circ));
                    let f_res = self.circ.enter_condition(f_term);
                    self.unwrap(f_res);
                    self.gen_stmt(f_cond.node);
                    self.circ.exit_condition();
                }
            }
            Statement::Return(ret) => {
                match ret {
                    Some(expr) => {
                        let ret = self.gen_expr(expr.node);
                        let ret_res = self.circ.return_(Some(ret));
                        self.unwrap(ret_res);
                    }
                    None => {
                        let ret_res = self.circ.return_(None);
                        self.unwrap(ret_res);
                    }
                };
            }
            Statement::Expression(expr) => {
<<<<<<< HEAD
                //println!("{:#?}",expr);
		let e = expr.unwrap().node;
                self.gen_expr(e);
            }
            Statement::For(for_stmt) => {
                // TODO: add enter_breakable
                self.circ.enter_scope();
                let const_iters = self.get_const_iters(for_stmt.node.clone());
                // Loop 5 times if const not specified
                let bound = const_iters.unwrap_or(ConstIteration {
                    name: "".to_string(),
                    val: 5
                }).val;
=======
                let e = expr.unwrap().node;
                self.gen_expr(e);
            }
            Statement::For(for_stmt) => {
                // TODO: Add enter_breakable
                self.circ.enter_scope();
                let const_iters = self.get_const_iters(for_stmt.node.clone());
                // TODO: Loop 5 times if const not specified
                let bound = const_iters.val;
>>>>>>> 75572c6... C Frontend (#22)

                for _ in 0..bound {
                    self.circ.enter_scope();
                    self.gen_stmt(for_stmt.node.statement.node.clone());
                    self.circ.exit_scope();
                    self.gen_expr(for_stmt.node.step.as_ref().unwrap().node.clone());
                }
                self.circ.exit_scope();
            }
            _ => unimplemented!("Statement {:#?} hasn't been implemented", stmt),
        }
    }

    fn gen(&mut self) {
        let TranslationUnit(nodes) = self.tu.clone();
        for n in nodes.iter() {
            match n.node {
                ExternalDeclaration::Declaration(ref decl) => {
                    debug!("{:#?}", decl);
                }
                ExternalDeclaration::FunctionDefinition(ref fn_def) => {
                    debug!("{:#?}", fn_def.node.clone());
<<<<<<< HEAD
                    //println!("function: {:#?}", fn_def.node);
                    let fn_info = ast_utils::get_fn_info(&fn_def.node);
                    println!("function name: {:#?}", fn_info.name);
		    self.circ.enter_fn(fn_info.name.to_owned(), fn_info.ret_ty.clone());
=======
                    let fn_info = ast_utils::get_fn_info(&fn_def.node);
                    self.circ
                        .enter_fn(fn_info.name.to_owned(), fn_info.ret_ty.clone());
>>>>>>> 75572c6... C Frontend (#22)
                    for arg in fn_info.args.iter() {
                        // TODO: self.gen_decl(arg);
                        let p = &arg.specifiers[0];
                        let vis = self.interpret_visibility(&p.node);
                        let base_ty = d_type_(arg.specifiers[1..].to_vec());
                        let d = &arg.declarator.as_ref().unwrap().node;
                        let derived_ty = self.derived_type_(base_ty.unwrap(), d.derived.to_vec());
                        let name = name_from_decl(d);
                        let res = self.circ.declare(name.clone(), &derived_ty, true, vis);
                        self.unwrap(res);
                    }
                    self.gen_stmt(fn_info.body.clone());
                    if let Some(r) = self.circ.exit_fn() {
                        match self.mode {
                            Mode::Mpc(_) => {
                                let ret_term = r.unwrap_term();
                                let ret_terms = ret_term.term.terms();
                                self.circ
                                    .cir_ctx()
                                    .cs
                                    .borrow_mut()
                                    .outputs
                                    .extend(ret_terms);
                            }
                            Mode::Proof => {
                                let ty = fn_info.ret_ty.as_ref().unwrap();
                                let name = "return".to_owned();
                                let term = r.unwrap_term();
<<<<<<< HEAD
                                let _r = self.circ.declare(name.clone(), &ty, false, PROVER_VIS);
                                self.circ
                                    .assign_with_assertions(name, term, &ty, PUBLIC_VIS);
                            }
                            _ => unimplemented!("Mode: {}", self.mode)
=======
                                let _r = self.circ.declare(name.clone(), ty, false, PROVER_VIS);
                                self.circ.assign_with_assertions(name, term, ty, PUBLIC_VIS);
                                unimplemented!();
                            }
                            _ => unimplemented!("Mode: {}", self.mode),
>>>>>>> 75572c6... C Frontend (#22)
                        }
                    }
                }
                _ => unimplemented!("Haven't implemented node: {:?}", n.node),
            }
        }
    }
}
