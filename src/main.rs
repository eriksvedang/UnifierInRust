#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Debug;

fn main() {

    let a : Expr<i32> = Expr::lvar("a");
    let b : Expr<i32> = Expr::lvar("b");
    let forty_two = Expr::Value(42);
    let hundred = Expr::Value(100);
    let ab = Expr::list(vec![Expr::lvar("a"), Expr::lvar("b")]);
    let nums = Expr::List(vec![forty_two.clone(), hundred.clone()]);
    let mixed1 = Expr::List(vec![forty_two.clone(), a.clone(), hundred.clone()]);
    let mixed2 = Expr::List(vec![b.clone(), Expr::list(vec![Expr::Value(666), Expr::Value(666)]), hundred.clone()]);

    run(a.clone(), a.clone());
    run(a.clone(), b.clone());
    run(a.clone(), forty_two.clone());
    run(hundred.clone(), forty_two.clone());
    run(ab.clone(), nums.clone());
    run(mixed1.clone(), mixed2.clone());
    
}

fn run<T : Debug + Eq + Clone>(a: Expr<T>, b: Expr<T>) {
    println!("Unifying {:?} with {:?} => {:?}", a.clone(), b.clone(), unify(a, b, empty()));
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
enum Expr<T> {
    LVar(String),
    Value(T),
    List(Vec<Expr<T>>)
}

impl<T> Expr<T> {
    fn lvar(name: &str) -> Expr<T> {
        Expr::LVar(name.to_string())
    }

    fn list(list: Vec<Expr<T>>) -> Expr<T> {
        Expr::List(list)
    }
}

type Bindings<T> = HashMap<String,Expr<T>>;
type Unification<T> = Result<Bindings<T>, String>;

fn empty<T>() -> Unification<T> {
    Ok(HashMap::new())
}
    
fn unify<T : Clone + Eq + Debug>(a: Expr<T>, b: Expr<T>, unification: Unification<T>) -> Unification<T> {
    match unification {
        Err(msg) => Err(msg),
        Ok(bindings) => {
            match (a.clone(), b.clone()) {
                _ if a == b => Ok(bindings),
                (Expr::LVar(name), expr) => Ok(extend_bindings(bindings, name, expr)),
                (expr, Expr::LVar(name)) => Ok(extend_bindings(bindings, name, expr)),
                (Expr::List(list_a), Expr::List(list_b)) => unify_lists(list_a, list_b, bindings),
                _ => Err(format!("Can't unify {:?} with {:?}.", a, b))
            }
        }
    }
}

fn unify_lists<T : Clone + Eq + Debug>(list_a: Vec<Expr<T>>, list_b: Vec<Expr<T>>, bindings: Bindings<T>) -> Unification<T> {
    if list_a.len() != list_b.len() {
        Err(format!("Can't unify {:?} with {:?}, lists are different length", list_a, list_b))
    }
    else {
        let mut current_bindings = bindings;
        for i in 0..list_a.len() {
            match unify(list_a[i].clone(), list_b[i].clone(), Ok(current_bindings)) {
                Ok(new_bindings) => current_bindings = new_bindings,
                Err(msg) => return Err(msg)
            }
        }
        return Ok(current_bindings);
    }
}

fn extend_bindings<T : Clone>(bindings: Bindings<T>, name: String, expr: Expr<T>) -> Bindings<T> {
    let mut new_bindings = bindings.clone();
    new_bindings.insert(name, expr);
    return new_bindings;
}
