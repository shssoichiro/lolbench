//{"name":"rust_parse :: parse_small","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[71,73,78,79,86,231,193,210,237,10,225,227,199,1,83,113,242,149,65,22,176,23,79,82,134,8,141,254,34,175,56,190]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: rust_parse :: parse_small ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: rust_parse :: parse_small ( & mut crit ) ; }