//{"name":"sherlock :: name_alt4_nocase","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[205,19,107,98,188,11,117,113,87,52,152,170,163,24,120,28,129,108,157,114,43,22,215,87,58,127,83,29,206,40,93,153]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: name_alt4_nocase ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: name_alt4_nocase ( & mut crit ) ; }