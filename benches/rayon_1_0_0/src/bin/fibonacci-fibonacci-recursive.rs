//{"name":"fibonacci :: fibonacci_recursive","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[50,31,104,251,226,78,91,182,153,219,254,25,239,210,250,83,95,61,46,162,178,211,232,112,74,122,223,51,225,219,29,21]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: fibonacci :: fibonacci_recursive ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: fibonacci :: fibonacci_recursive ( & mut crit ) ; }