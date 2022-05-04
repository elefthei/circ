//! Lowering IR to ABY DSL
//! [EzPC Compiler](https://github.com/mpc-msri/EzPC/blob/da94a982709123c8186d27c9c93e27f243d85f0e/EzPC/EzPC/ABY_example/common/ezpc.h)

//! Inv gates need to typecast circuit object to boolean circuit
//! [Link to comment in EzPC Compiler](https://github.com/mpc-msri/EzPC/blob/da94a982709123c8186d27c9c93e27f243d85f0e/EzPC/EzPC/codegen.ml)

use std::fmt;
use crate::ir::term::*;
#[cfg(feature = "lp")]
use crate::target::aby::assignment::ilp::assign;
use crate::target::aby::assignment::SharingMap;
use crate::target::aby::utils::*;
use std::fmt;
use std::path::Path;

use super::assignment::assign_all_boolean;
use super::assignment::assign_all_yao;
use super::assignment::assign_arithmetic_and_boolean;
use super::assignment::assign_arithmetic_and_yao;
use super::assignment::assign_greedy;

const PUBLIC: u8 = 2;

#[derive(Clone)]
enum EmbeddedTerm {
    Bool(String),
    Bv(String),
}

impl fmt::Display for EmbeddedTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

struct ToABY {
    md: ComputationMetadata,
    inputs: TermSet,
    cache: TermMap<EmbeddedTerm>,
    term_to_share_cnt: TermMap<i32>,
    s_map: SharingMap,
    share_cnt: i32,
    bytecode_path: String,
    share_map_path: String,
    bytecode_output: Vec<String>,
    share_map_output: Vec<String>,
}

impl Drop for ToABY {
    fn drop(&mut self) {
        use std::mem::take;
        // drop everything that uses a Term
        drop(take(&mut self.md));
        self.inputs.clear();
        self.cache.clear();
        self.term_to_share_cnt.clear();
        self.s_map.clear();
        // clean up
        garbage_collect();
    }
}

impl ToABY {
    fn new(s_map: SharingMap, md: ComputationMetadata, path: &Path, lang: &str) -> Self {
        Self {
            md,
            inputs: TermSet::new(),
            cache: TermMap::new(),
            term_to_share_cnt: TermMap::new(),
            s_map,
            share_cnt: 0,
            bytecode_path: get_path(path, lang, "bytecode"),
            share_map_path: get_path(path, lang, "share_map"),
            bytecode_output: Vec::new(),
            share_map_output: Vec::new(),
        }
    }

    fn map_terms_to_shares(&mut self, term_: Term) {
        for t in PostOrderIter::new(term_) {
            self.term_to_share_cnt.insert(t, self.share_cnt);
            self.share_cnt += 1;
        }
    }

<<<<<<< HEAD
<<<<<<< HEAD
    fn add_conv_gate(&self, p_t: Term, c_t: Term, c_circ: String) -> String {
        let p_share = self.s_map.get(&p_t).unwrap();
        let c_share = self.s_map.get(&c_t).unwrap();

        match (c_share, p_share) {
            (ShareType::Arithmetic, ShareType::Arithmetic) => c_circ,
            (ShareType::Boolean, ShareType::Boolean) => c_circ,
            (ShareType::Yao, ShareType::Yao) => c_circ,
            (ShareType::Arithmetic, ShareType::Boolean) => {
                format!("bcirc->PutY2BGate(ycirc->PutA2YGate({}))", c_circ)
            }
            (ShareType::Arithmetic, ShareType::Yao) => format!("ycirc->PutA2YGate({})", c_circ),
            (ShareType::Boolean, ShareType::Arithmetic) => format!("acirc->PutB2AGate({})", c_circ),
            (ShareType::Boolean, ShareType::Yao) => format!("ycirc->PutB2YGate({})", c_circ),
            (ShareType::Yao, ShareType::Arithmetic) => {
                format!("acirc->PutB2AGate(bcirc->PutY2BGate({}))", c_circ)
            }
            (ShareType::Yao, ShareType::Boolean) => format!("bcirc->PutY2BGate({})", c_circ),
        }
    }

    fn add_cons_gate(&self, t: Term) -> String {
        let name = ToABY::get_var_name(t.clone(), true);
        let s_circ = self.get_sharetype_circ(t);
        format!(
            "s_{} = {}->PutCONSGate((uint32_t){}, bitlen);\n",
            name, s_circ, name
        )
    }

    fn add_in_gate(&self, t: Term, role: String) -> String {
        let name = ToABY::get_var_name(t.clone(), true);
        let s_circ = self.get_sharetype_circ(t);
        format!(
            "\ts_{} = {}->PutINGate({}, bitlen, {});\n",
            name, s_circ, name, role
        )
    }

    fn add_dummy_gate(&self, t: Term) -> String {
        let name = ToABY::get_var_name(t.clone(), true);
        let s_circ = self.get_sharetype_circ(t);
        format!("\ts_{} = {}->PutDummyINGate(bitlen);\n", name, s_circ)
    }

    /// Initialize private and public inputs from each party
    /// Party inputs are stored in *self.inputs*
    fn init_inputs(&mut self) {
        let mut server_inputs = TermSet::new();
        let mut client_inputs = TermSet::new();
        let mut public_inputs = TermSet::new();


        // Parse input parameters from command line as uint32_t variables
        // Initialize shares for each party
        for (t, party) in self.inputs.iter() {
            let name = ToABY::get_var_name(t.clone(), false);
            let name_ = ToABY::get_var_name(t.clone(), true);

=======
    fn write_mapping_file(&self, term_: Term) {
=======
    fn write_mapping_file(&mut self, term_: Term) {
>>>>>>> 13f9a09... Updated ABY VM to include `IN` bytecode instruction (#65)
        for t in PostOrderIter::new(term_) {
            let share_type = self.s_map.get(&t).unwrap();
            let share_str = share_type.char();
            let share_cnt = self.term_to_share_cnt.get(&t).unwrap();
<<<<<<< HEAD
>>>>>>> 8fed29b... ABY VM and Interpreter (#47)
            write_line_to_file(
                &self.share_map_path,
                &format!("{} {}\n", *share_cnt, share_str),
            );
=======
            let line = format!("{} {}\n", *share_cnt, share_str);
            self.share_map_output.push(line);
>>>>>>> 13f9a09... Updated ABY VM to include `IN` bytecode instruction (#65)
        }
    }

    fn get_var_name(t: &Term) -> String {
        match &t.op {
            Op::Var(name, _) => {
                let new_name = name.to_string().replace('.', "_");
                let n = new_name.split('_').collect::<Vec<&str>>();

                match n.len() {
                    5 => n[3].to_string(),
                    6.. => {
                        let l = n.len() - 1;
                        format!("{}_{}", n[l - 2], n[l])
                    }
                    _ => {
                        panic!("Invalid variable name: {}", name);
                    }
                }
            }
            _ => panic!("Term {} is not of type Var", t),
        }
    }

    fn get_share_name(&mut self, t: &Term) -> String {
        let share_cnt = self.term_to_share_cnt.get(t).unwrap();
        format!("s_{}", share_cnt)
    }

    fn unwrap_vis(&self, name: &str) -> u8 {
        match self.md.input_vis.get(name).unwrap() {
            Some(role) => *role,
            None => PUBLIC,
        }
    }

    fn embed_eq(&mut self, t: Term, a_term: Term, b_term: Term) {
        let share = self.get_share_name(&t);
        let s = self.term_to_share_cnt.get(&t).unwrap();
        let a = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
        let b = self.term_to_share_cnt.get(&t.cs[1]).unwrap();
        let op = "EQ";
        let line = format!("2 1 {} {} {} {}\n", a, b, s, op);
        self.bytecode_output.push(line);
        match check(&a_term) {
            Sort::Bool => {
                self.check_bool(&a_term);
                self.check_bool(&b_term);
                self.cache.insert(t, EmbeddedTerm::Bool(share));
            }
            Sort::BitVector(_) => {
                self.check_bv(&a_term);
                self.check_bv(&b_term);
                self.cache.insert(t, EmbeddedTerm::Bool(share));
            }
            e => panic!("Unimplemented sort for Eq: {:?}", e),
        }
    }

    /// Given term `t`, type-check `t` is of type Bool
    fn check_bool(&self, t: &Term) {
        match self
            .cache
            .get(t)
            .unwrap_or_else(|| panic!("Missing wire for {:?}", t))
        {
            EmbeddedTerm::Bool(_) => (),
            _ => panic!("Non-bool for {:?}", t),
        }
    }

    fn embed_bool(&mut self, t: Term) {
        let share = self.get_share_name(&t);
        let s = self.term_to_share_cnt.get(&t).unwrap();
        match &t.op {
            Op::Var(name, Sort::Bool) => {
                if !self.inputs.contains(&t) && self.md.input_vis.contains_key(name) {
                    let term_name = ToABY::get_var_name(&t);
                    let vis = self.unwrap_vis(name);
                    let share_cnt = self.term_to_share_cnt.get(&t).unwrap();
                    let op = "IN";

                    if vis == PUBLIC {
                        let bitlen = 1;
                        let line = format!(
                            "3 1 {} {} {} {} {}\n",
                            term_name, vis, bitlen, share_cnt, op
                        );
                        self.bytecode_output.insert(0, line);
                    } else {
                        let line = format!("2 1 {} {} {} {}\n", term_name, vis, share_cnt, op);
                        self.bytecode_output.insert(0, line);
                    }
                    self.inputs.insert(t.clone());
                }

                if !self.cache.contains_key(&t) {
                    self.cache.insert(
                        t.clone(),
                        EmbeddedTerm::Bool(format!("s_{}", ToABY::get_var_name(&t))),
                    );
                }
            }
            Op::Const(Value::Bool(b)) => {
                let op = "CONS_bool";
                let line = format!("1 1 {} {} {}\n", *b as i32, s, op);
                self.bytecode_output.push(line);
                self.cache.insert(t.clone(), EmbeddedTerm::Bool(share));
            }
            Op::Eq => {
                self.embed_eq(t.clone(), t.cs[0].clone(), t.cs[1].clone());
            }
            Op::Ite => {
                let op = "MUX";

                self.check_bool(&t.cs[0]);
                self.check_bool(&t.cs[1]);
                self.check_bool(&t.cs[2]);

                let sel = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                let a = self.term_to_share_cnt.get(&t.cs[1]).unwrap();
                let b = self.term_to_share_cnt.get(&t.cs[2]).unwrap();

                let line = format!("3 1 {} {} {} {} {}\n", sel, a, b, s, op);
                self.bytecode_output.push(line);

                self.cache.insert(t.clone(), EmbeddedTerm::Bool(share));
            }
            Op::Not => {
                let op = "NOT";

                self.check_bool(&t.cs[0]);

                let a = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                let line = format!("1 1 {} {} {}\n", a, s, op);
                self.bytecode_output.push(line);

                self.cache.insert(t.clone(), EmbeddedTerm::Bool(share));
            }
            Op::BoolNaryOp(o) => {
                if t.cs.len() == 1 {
                    // HACK: Conditionals might not contain two variables
                    // If t.cs len is 1, just output that term
                    // This is to bypass adding an AND gate with a single conditional term
                    // Refer to pub fn condition() in src/circify/mod.rs
                    self.check_bool(&t.cs[0]);
                    self.cache.insert(t.clone(), EmbeddedTerm::Bool(share));
                } else {
                    self.check_bool(&t.cs[0]);
                    self.check_bool(&t.cs[1]);

                    let op = match o {
                        BoolNaryOp::Or => "OR",
                        BoolNaryOp::And => "AND",
                        BoolNaryOp::Xor => "XOR",
                    };

                    let a = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                    let b = self.term_to_share_cnt.get(&t.cs[1]).unwrap();
                    let line = format!("2 1 {} {} {} {}\n", a, b, s, op);
                    self.bytecode_output.push(line);

                    self.cache.insert(t.clone(), EmbeddedTerm::Bool(share));
                }
            }
            Op::BvBinPred(o) => {
                let op = match o {
                    BvBinPred::Ugt => "GT",
                    BvBinPred::Ult => "LT",
                    BvBinPred::Uge => "GE",
                    BvBinPred::Ule => "LE",
                    _ => panic!("Non-field in bool BvBinPred: {}", o),
                };

                self.check_bv(&t.cs[0]);
                self.check_bv(&t.cs[1]);

                let a = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                let b = self.term_to_share_cnt.get(&t.cs[1]).unwrap();
                let line = format!("2 1 {} {} {} {}\n", a, b, s, op);
                self.bytecode_output.push(line);

                self.cache.insert(t.clone(), EmbeddedTerm::Bool(share));
            }
            _ => panic!("Non-field in embed_bool: {}", t),
        }
    }

    /// Given term `t`, type-check `t` is of type Bv
    fn check_bv(&self, t: &Term) {
        match self
            .cache
            .get(t)
            .unwrap_or_else(|| panic!("Missing wire for {:?}", t))
        {
            EmbeddedTerm::Bv(_) => (),
            _ => panic!("Non-bv for {:?}", t),
        }
    }

    fn embed_bv(&mut self, t: Term) {
        let share = self.get_share_name(&t);
        let s = self.term_to_share_cnt.get(&t).unwrap();
        match &t.op {
            Op::Var(name, Sort::BitVector(_)) => {
                if !self.inputs.contains(&t) && self.md.input_vis.contains_key(name) {
                    let term_name = ToABY::get_var_name(&t);
                    let vis = self.unwrap_vis(name);
                    let share_cnt = self.term_to_share_cnt.get(&t).unwrap();
                    let op = "IN";

                    if vis == PUBLIC {
                        let bitlen = 32;
                        let line = format!(
                            "3 1 {} {} {} {} {}\n",
                            term_name, vis, bitlen, share_cnt, op
                        );
                        self.bytecode_output.insert(0, line);
                    } else {
                        let line = format!("2 1 {} {} {} {}\n", term_name, vis, share_cnt, op);
                        self.bytecode_output.insert(0, line);
                    }
                    self.inputs.insert(t.clone());
                }

                if !self.cache.contains_key(&t) {
                    self.cache.insert(
                        t.clone(),
                        EmbeddedTerm::Bv(format!("s_{}", ToABY::get_var_name(&t))),
                    );
                }
            }
            Op::Const(Value::BitVector(b)) => {
                let op = "CONS_bv";
                let line = format!("1 1 {} {} {}\n", b.as_sint(), s, op);
                self.bytecode_output.push(line);
                self.cache.insert(t.clone(), EmbeddedTerm::Bv(share));
            }
            Op::Ite => {
                let op = "MUX";

                self.check_bool(&t.cs[0]);
                self.check_bv(&t.cs[1]);
                self.check_bv(&t.cs[2]);

                let sel = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                let a = self.term_to_share_cnt.get(&t.cs[1]).unwrap();
                let b = self.term_to_share_cnt.get(&t.cs[2]).unwrap();

                let line = format!("3 1 {} {} {} {} {}\n", sel, a, b, s, op);
                self.bytecode_output.push(line);

                self.cache.insert(t.clone(), EmbeddedTerm::Bv(share));
            }
            Op::BvNaryOp(o) => {
                let op = match o {
                    BvNaryOp::Xor => "XOR",
                    BvNaryOp::Or => "OR",
                    BvNaryOp::And => "AND",
                    BvNaryOp::Add => "ADD",
                    BvNaryOp::Mul => "MUL",
                };

                self.check_bv(&t.cs[0]);
                self.check_bv(&t.cs[1]);

                let a = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                let b = self.term_to_share_cnt.get(&t.cs[1]).unwrap();

                let line = format!("2 1 {} {} {} {}\n", a, b, s, op);
                self.bytecode_output.push(line);

                self.cache.insert(
                    t.clone(),
                    EmbeddedTerm::Bv(share),
                );
                write_line_to_file(&self.circuit_fname, &s);

                self.cache.insert(t.clone(), EmbeddedTerm::Bv(share));
            }
            Op::BvBinOp(o) => {
                let op = match o {
                    BvBinOp::Sub => "SUB",
                    BvBinOp::Udiv => "DIV",
                    BvBinOp::Urem => "REM",
                    BvBinOp::Shl => "SHL",
                    BvBinOp::Lshr => "LSHR",
                    BvBinOp::Ashr => "ASHR",
                };

                self.check_bv(&t.cs[0]);
                self.check_bv(&t.cs[1]);

                let a = self.term_to_share_cnt.get(&t.cs[0]).unwrap();
                let b = self.term_to_share_cnt.get(&t.cs[1]).unwrap();

                let line = format!("2 1 {} {} {} {}\n", a, b, s, op);
                self.bytecode_output.push(line);

                self.cache.insert(t.clone(), EmbeddedTerm::Bv(share));
            }
            // TODO
            Op::BvExtract(_start, _end) => {}
            _ => panic!("Non-field in embed_bv: {:?}", t),
        }
    }

<<<<<<< HEAD
    /// Given a Circuit `circ`, wrap `circ` in an OUT gate to extract the value of
    /// the circuit to a share
    ///
    /// Return a String of the resulting Circuit
    fn format_output_circuit(&self, t: Term) -> String {
        match self.cache.get(&t) {
            Some(EmbeddedTerm::Bool(s)) | Some(EmbeddedTerm::Bv(s)) => {
                format!(
                    "add_to_output_queue(out_q, {}->PutOUTGate({}, ALL), role, std::cout);\n",
                    self.get_sharetype_circ(t),
                    s
                )
            }
            None => panic!("Term not found in cache: {:#?}", t),
        }
    }

    fn embed(&mut self, t: Term) -> String {
        for c in PostOrderIter::new(t.clone()) {
=======
    fn embed(&mut self, t: Term) {
        for c in PostOrderIter::new(t) {
>>>>>>> 8fed29b... ABY VM and Interpreter (#47)
            match check(&c) {
                Sort::Bool => {
                    self.embed_bool(c);
                }
                Sort::BitVector(_) => {
                    self.embed_bv(c);
                }
                e => panic!("Unsupported sort in embed: {:?}", e),
            }
        }
    }

    /// Given a term `t`, lower `t` to ABY Circuits
    fn lower(&mut self, t: Term) {
        self.embed(t.clone());

        let op = "OUT";
        let s = self.term_to_share_cnt.get(&t).unwrap();
        let line = format!("1 0 {} {}\n", s, op);
        self.bytecode_output.push(line);

        // write lines to file
        write_lines_to_file(&self.bytecode_path, &self.bytecode_output);
        write_lines_to_file(&self.share_map_path, &self.share_map_output);
    }
}

/// Convert this (IR) `ir` to ABY.
pub fn to_aby(ir: Computation, path: &Path, lang: &str, cm: &str, ss: &str) {
    let Computation {
        outputs: terms,
        metadata: md,
        ..
    } = ir.clone();

    let s_map: SharingMap = match ss {
        "b" => assign_all_boolean(&ir, cm),
        "y" => assign_all_yao(&ir, cm),
        "a+b" => assign_arithmetic_and_boolean(&ir, cm),
        "a+y" => assign_arithmetic_and_yao(&ir, cm),
        "greedy" => assign_greedy(&ir, cm),
        #[cfg(feature = "lp")]
        "lp" => assign(&ir, cm),
        #[cfg(feature = "lp")]
        "glp" => assign(&ir, cm),
        _ => {
            panic!("Unsupported sharing scheme: {}", ss);
        }
    };

    let mut converter = ToABY::new(s_map, md, path, lang);

    for t in terms {
        // println!("terms: {}", t);
        converter.map_terms_to_shares(t.clone());
        converter.write_mapping_file(t.clone());
        converter.lower(t.clone());
    }
}
