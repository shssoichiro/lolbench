//{"name":"sort :: demo_quick_sort_mostly_ascending","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[59,248,250,133,253,115,255,142,214,213,233,5,22,29,35,237,238,83,70,30,4,131,152,236,210,14,246,3,27,102,14,251]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: demo_quick_sort_mostly_ascending ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: demo_quick_sort_mostly_ascending ( & mut crit ) ; }