//{"name":"misc :: short_haystack_1x","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[236,10,222,127,38,39,51,71,174,249,39,165,103,93,53,250,45,39,157,134,51,125,119,176,24,248,139,122,229,20,212,243]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: short_haystack_1x ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: short_haystack_1x ( & mut crit ) ; }