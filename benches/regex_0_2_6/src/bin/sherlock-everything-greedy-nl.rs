//{"name":"sherlock :: everything_greedy_nl","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[77,112,131,248,101,12,219,176,191,32,180,34,29,52,51,213,77,189,140,240,99,44,142,245,74,45,92,69,130,17,224,119]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: everything_greedy_nl ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: everything_greedy_nl ( & mut crit ) ; }