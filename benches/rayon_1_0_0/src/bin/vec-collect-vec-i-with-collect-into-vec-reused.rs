//{"name":"vec_collect::vec_i :: with_collect_into_vec_reused","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[190,74,89,113,0,140,213,128,180,98,13,66,138,223,97,203,136,39,151,2,119,48,64,104,220,202,110,215,250,85,149,166]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: vec_collect :: vec_i :: with_collect_into_vec_reused ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: vec_collect :: vec_i :: with_collect_into_vec_reused ( & mut crit ) ; }