//{"name":"sherlock :: name_sherlock_holmes_nocase","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[2,22,123,11,180,43,163,244,244,105,252,139,67,88,64,221,245,158,107,157,175,113,244,169,20,170,176,17,67,58,220,102]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: name_sherlock_holmes_nocase ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: name_sherlock_holmes_nocase ( & mut crit ) ; }