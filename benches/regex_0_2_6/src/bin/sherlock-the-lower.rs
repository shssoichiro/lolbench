//{"name":"sherlock :: the_lower","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[7,254,106,91,156,14,13,53,255,218,22,126,62,21,174,238,195,121,63,184,12,119,249,165,207,92,37,29,161,182,208,151]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: the_lower ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: the_lower ( & mut crit ) ; }