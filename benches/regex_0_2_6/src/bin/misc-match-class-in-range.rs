//{"name":"misc :: match_class_in_range","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[117,98,135,208,245,239,7,167,173,33,152,116,25,99,95,114,143,39,22,208,167,240,69,253,110,218,157,176,158,20,146,113]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: match_class_in_range ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: match_class_in_range ( & mut crit ) ; }