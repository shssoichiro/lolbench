//{"name":"str_split :: parallel_space_fn","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[33,251,164,83,148,173,8,60,182,115,186,51,236,172,61,32,234,2,251,244,172,236,59,64,159,86,135,85,24,249,164,239]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: str_split :: parallel_space_fn ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: str_split :: parallel_space_fn ( & mut crit ) ; }