//{"name":"bench_e2e_rt_q9_5_1024k","crate":"brotli_1_1_3","checksum":{"method":"sha256-generic-array","value":[108,91,67,131,112,17,35,120,135,109,170,133,80,202,94,226,245,151,51,115,149,12,244,182,19,34,14,160,173,149,168,1]}}
extern crate brotli_1_1_3 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; brotli_1_1_3 :: bench_e2e_rt_q9_5_1024k ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; brotli_1_1_3 :: bench_e2e_rt_q9_5_1024k ( & mut crit ) ; }