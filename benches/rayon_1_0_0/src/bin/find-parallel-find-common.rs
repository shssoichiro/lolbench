//{"name":"find :: parallel_find_common","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[34,96,185,153,44,73,114,73,107,80,240,118,15,55,120,211,147,153,126,139,149,114,78,55,82,16,187,171,141,42,196,43]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: find :: parallel_find_common ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: find :: parallel_find_common ( & mut crit ) ; }