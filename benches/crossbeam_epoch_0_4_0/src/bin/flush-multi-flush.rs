//{"name":"flush :: multi_flush","crate":"crossbeam_epoch_0_4_0","checksum":{"method":"sha256-generic-array","value":[188,3,7,22,233,196,192,164,209,54,237,74,70,70,223,132,13,19,216,179,231,193,137,28,142,51,175,193,43,245,103,170]}}
extern crate crossbeam_epoch_0_4_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; crossbeam_epoch_0_4_0 :: flush :: multi_flush ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; crossbeam_epoch_0_4_0 :: flush :: multi_flush ( & mut crit ) ; }