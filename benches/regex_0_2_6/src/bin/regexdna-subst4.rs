//{"name":"regexdna :: subst4","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[209,173,96,170,204,243,49,113,78,168,92,176,34,64,35,171,21,120,99,199,224,252,15,228,141,15,207,164,148,206,52,171]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: regexdna :: subst4 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: regexdna :: subst4 ( & mut crit ) ; }