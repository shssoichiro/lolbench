//{"name":"misc :: anchored_literal_short_non_match","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[231,69,182,189,14,118,179,83,200,233,41,251,190,94,138,76,47,98,224,231,90,79,161,79,251,254,86,3,117,229,115,244]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: anchored_literal_short_non_match ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: anchored_literal_short_non_match ( & mut crit ) ; }