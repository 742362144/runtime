#![feature(in_band_lifetimes)]
#![feature(generators, generator_trait)]

use std::sync::mpsc::Sender;
use std::pin::Pin;
use std::ops::{Generator, GeneratorState};
use std::sync::{Mutex, Arc};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread;
use std::time::Duration;
use libloading::os::unix::{Library, Symbol};

use rustlearn::prelude::*;
use rustlearn::datasets::iris;
use rustlearn::cross_validation::CrossValidation;
use rustlearn::linear_models::sgdclassifier::Hyperparameters;
use rustlearn::metrics::accuracy_score;

use crate::policy::Policy;

pub fn init(policy: Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
// pub fn init() -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
//     print_hello();
    println!("{}", "enter");
    // let ctx = tctx.clone();
    // let tx = ctx.lock();

    println!("{}", "gen init");
    let mut p = policy.clone();
    Box::pin(move || {
        let i:u64 = 1;
        p.lock().unwrap().set("A", "111");
        yield i;
        let mut j = 0;
        while j < 100 {
            p.lock().unwrap().get("A");
            j = j + 1;
        }

        1111
    })
}

pub fn khop(policy: Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
// pub fn init() -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
//     print_hello();
    println!("{}", "enter");
    // let ctx = tctx.clone();
    // let tx = ctx.lock();

    println!("{}", "gen init");
    let mut p = policy.clone();
    Box::pin(move || {
        let i:u64 = 1;
        p.lock().unwrap().set("A", "111");
        yield i;
        let mut j = 0;
        while j < 100 {
            p.lock().unwrap().get("A");
            j = j + 1;
        }

        1111
    })
}

pub fn rg(policy: Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
    let mut p = policy.clone();
    Box::pin(move || {
        let (X, y) = iris::load_data();

        let num_splits = 10;
        let num_epochs = 5;

        let mut accuracy = 0.0;

        for (train_idx, test_idx) in CrossValidation::new(X.rows(), num_splits) {

            let X_train = X.get_rows(&train_idx);
            let y_train = y.get_rows(&train_idx);
            let X_test = X.get_rows(&test_idx);
            let y_test = y.get_rows(&test_idx);

            let mut model = Hyperparameters::new(X.cols())
                .learning_rate(0.5)
                .l2_penalty(0.0)
                .l1_penalty(0.0)
                .one_vs_rest();

            for _ in 0..num_epochs {
                model.fit(&X_train, &y_train).unwrap();
            }

            let prediction = model.predict(&X_test).unwrap();
            accuracy += accuracy_score(&y_test, &prediction);
        }

        accuracy /= num_splits as f32;

        1111
    })






}


// pub fn init(policy: Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
//     // println!("{}", "test");
//     // let mut ctx = ThreadSafeContext::new();
//     // let b = Arc::new(&ctx);
//     // // type Proc = unsafe extern "C" fn(Rc<Db>) -> Pin<Box<Generator<Yield=u64, Return=InvokeResult>>>;
//     // type Proc = unsafe extern "C" fn(Arc<&ThreadSafeContext<DetachedFromClient>>) -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//
//     type Proc = unsafe extern fn(Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//     // type Proc = unsafe extern "C" fn(Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//     let library_path = String::from("/home/coder/IdeaProjects/add/target/debug/libadd.so");
//     println!("Loading add() from {}", library_path);
//
//     let lib = Library::new(library_path).unwrap();
//
//     unsafe {
//         let func: Symbol<Proc> = lib.get(b"init").unwrap();
//         func(policy)
//     }
// }