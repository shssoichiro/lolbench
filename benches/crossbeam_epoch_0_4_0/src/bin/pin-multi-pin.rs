//{"name":"pin :: multi_pin","crate":"crossbeam_epoch_0_4_0","checksum":{"method":"sha256-generic-array","value":[224,138,110,95,96,47,191,152,43,227,5,151,153,157,233,3,80,89,15,160,234,254,143,24,215,37,237,97,3,138,111,128]}}
extern crate crossbeam_epoch_0_4_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; crossbeam_epoch_0_4_0 :: pin :: multi_pin ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; crossbeam_epoch_0_4_0 :: pin :: multi_pin ( & mut crit ) ; }