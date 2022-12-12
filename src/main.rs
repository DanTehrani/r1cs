#![allow(non_snake_case)]

fn dot_product(a: &[u32], b: &[u32]) -> u32 {
    // Calculate the dot product of two vectors.
    assert_eq!(a.len(), b.len());
    let mut product = 0;
    for i in 0..a.len() {
        product += a[i] * b[i];
    }
    product
}

type Constraint = Vec<u32>;

fn main() {
    let mut A: Vec<Constraint> = vec![];
    let mut B: Vec<Constraint> = vec![];
    let mut C: Vec<Constraint> = vec![];

    // x ** 2 + y = ~out
    // sym_1 = x * x
    // ~out = sym_1 + y
    // '~one',' y', 'x', 'sym_1', '~out'

    // sym_1 = x * x
    A.push(vec![0, 0, 1, 0, 0]);
    B.push(vec![0, 0, 1, 0, 0]);
    C.push(vec![0, 0, 0, 1, 0]);

    // ~out = sym_1 + y
    A.push(vec![0, 1, 0, 1, 0]);
    B.push(vec![1, 0, 0, 0, 0]);
    C.push(vec![0, 0, 0, 0, 1]);

    let witness = vec![1, 2, 2, 4, 6];

    for i in 0..A.len() {
        let a_gate = &A[i];
        let b_gate = &B[i];
        let c_gate = &C[i];

        let a = dot_product(a_gate, &witness);
        let b = dot_product(b_gate, &witness);
        let c = dot_product(c_gate, &witness);

        if a * b != c {
            panic!("Constraint {} failed", i);
        }
    }

    println!("All constraints passed");
}
