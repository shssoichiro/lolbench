//{"name":"uflat08_txt3","crate":"snap_0_2_4","checksum":{"method":"sha256-generic-array","value":[196,41,99,83,217,208,135,123,51,2,120,119,89,82,70,68,155,193,0,10,123,28,122,39,137,119,228,167,150,11,191,50]}}
extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; snap_0_2_4 :: uflat08_txt3 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; snap_0_2_4 :: uflat08_txt3 ( & mut crit ) ; }