//{"name":"misc :: short_haystack_2x","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[154,75,63,164,245,200,245,227,78,135,188,172,159,234,166,199,99,125,122,252,97,168,85,136,42,239,237,253,65,209,31,64]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: short_haystack_2x ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: short_haystack_2x ( & mut crit ) ; }