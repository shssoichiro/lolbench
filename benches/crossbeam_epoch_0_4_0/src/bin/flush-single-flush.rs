//{"name":"flush :: single_flush","crate":"crossbeam_epoch_0_4_0","checksum":{"method":"sha256-generic-array","value":[125,232,119,109,220,11,241,40,51,46,94,52,108,158,89,205,26,203,68,65,40,149,185,200,18,171,240,7,96,174,236,189]}}
extern crate crossbeam_epoch_0_4_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; crossbeam_epoch_0_4_0 :: flush :: single_flush ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; crossbeam_epoch_0_4_0 :: flush :: single_flush ( & mut crit ) ; }