//{"name":"serde_citm_dom","crate":"json_benchmark_c7d3d9b","checksum":{"method":"sha256-generic-array","value":[133,66,6,47,4,44,44,29,226,101,251,74,113,174,186,99,162,7,192,205,11,110,101,6,46,141,213,189,131,203,114,107]}}
extern crate json_benchmark_c7d3d9b ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; json_benchmark_c7d3d9b :: serde_citm_dom ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; json_benchmark_c7d3d9b :: serde_citm_dom ( & mut crit ) ; }