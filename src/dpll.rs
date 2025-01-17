pub mod cnf_formula;
use cnf_formula::*;

#[allow(dead_code)]
pub fn find_propogatable(f: &Formula) -> Option<(Variable, bool)> {
    for c in f {
        if c.len() == 1 {
            let atom = &c[0];
            match atom {
                Atom::Base(var) => return Some((*var, true)),
                Atom::Not(var) => return Some((*var, false)),
            }
        }
    }
    None
}


#[allow(dead_code)]
pub fn propogate_unit(f: &mut Formula, v: Variable, b: bool) {
    if b == true {
        let basev: Atom = Atom::Base(v);
        let notv: Atom = Atom::Not(v);
        f.retain_mut(|x| {
            if x.contains(&basev) {
                false
            } else {
                if let Some(pos) = x.iter().position(|x| *x == notv) {
                    x.remove(pos);
                }
                true
            }
        });
    } else {
        let basev: Atom = Atom::Not(v);
        let notv: Atom = Atom::Base(v);
        f.retain_mut(|x| {
            if x.contains(&basev) {
                false
            } else {
                if let Some(pos) = x.iter().position(|x| *x == notv) {
                    x.remove(pos);
                }
                true
            }
        });
    }
}

#[allow(dead_code)]
pub fn find_pure_var(f: &Formula) -> Option<Variable> {
    let vars = get_vars(f);
    for i in vars {
        if is_pure(f, i) {
            return Some(i);
        }
    }
    None
}

#[allow(dead_code)]
pub fn assign_pure_var(f: &mut Formula, v: Variable) {
    f.retain(|c| !has_var_clause(&c, v));
}

#[allow(dead_code)]
pub fn unit_propogate(f: &mut Formula) {
    match find_propogatable(f) {
        Option::None => return,
        Option::Some((v, b)) => {
            propogate_unit(f, v, b);
            unit_propogate(f)
        }
    }
}

#[allow(dead_code)]
pub fn assign_pure_vars(f: &mut Formula) {
    match find_pure_var(f) {
        Option::None => return,
        Option::Some(v) => {
            assign_pure_var(f, v);
            assign_pure_vars(f);
        }
    }
}

pub fn dpll(f: &mut Formula) -> bool {
    unit_propogate(f);
    assign_pure_vars(f);
    if f.is_empty() {
        return true;
    }
    if f.iter().any(|c| c.is_empty()) {
        return false;
    }
    let v = get_vars(f)[0];
    let f_clone = &mut f.clone();
    let f_clone2 = &mut f.clone();

    let base: Clause = [Atom::Base(v)].to_vec();
    let not: Clause = [Atom::Not(v)].to_vec();

    f_clone.push(base);
    f_clone2.push(not);

    return dpll(f_clone) || dpll(f_clone2);
}