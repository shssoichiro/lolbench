//{"name":"sort :: demo_merge_sort_mostly_descending","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[90,227,24,110,57,219,24,13,255,218,186,145,246,255,145,11,157,251,56,21,121,64,144,252,164,167,186,22,193,111,199,127]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: demo_merge_sort_mostly_descending ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: demo_merge_sort_mostly_descending ( & mut crit ) ; }