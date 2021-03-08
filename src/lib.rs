#![feature(generators, generator_trait)]
#![feature(llvm_asm)]

pub mod task;
pub mod executor;
pub mod policy;
pub mod invoke;
pub mod cycles;
pub mod ext;
mod sys;

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::io::stderr;
    use std::collections::BTreeMap;
    use std::thread::sleep;

    #[test]
    fn it_works() {
        extern crate self_meter;

        use std::io::{Write, stderr};
        use std::time::Duration;
        use std::thread::sleep;
        use std::collections::BTreeMap;


        let mut meter = self_meter::Meter::new(Duration::new(1, 0)).unwrap();
        meter.track_current_thread("executor");
        loop {
            meter.scan()
                .map_err(|e| writeln!(&mut stderr(), "Scan error: {}", e)).ok();
            println!("Report: {:#?}", meter.report());
            println!("Threads: {:#?}",
                     meter.thread_report().map(|x| x.collect::<BTreeMap<_,_>>()));
            let mut x = 0;
            for _ in 0..10000000 {
                x = u64::wrapping_mul(x, 7);
            }
            sleep(Duration::new(1, 0));
        }

//         println!("Using platform {:?}", spork.platform());
//         println!("CPU cores: {:?}x @ {:?} Hz", spork.num_cores(), spork.clock_speed());
//
// // get process stats
//         let p_stats = match spork.stats(StatType::Process) {
//             Ok(s) => s,
//             Err(e) => panic!("Error polling process stats! {:?}", e)
//         };
//         println!("Process stats: {:?}", p_stats);
    }
}

