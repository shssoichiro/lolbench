//{"name":"factorial :: factorial_par_iter","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[178,164,187,172,62,186,54,239,235,91,108,12,181,205,35,156,140,235,217,250,45,201,59,140,143,241,246,125,178,165,64,198]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: factorial :: factorial_par_iter ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: factorial :: factorial_par_iter ( & mut crit ) ; }