//{"name":"regexdna :: variant7","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[78,42,87,247,61,237,112,20,23,60,82,176,226,68,93,0,236,71,1,34,59,20,16,32,195,7,190,128,162,98,121,31]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: regexdna :: variant7 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: regexdna :: variant7 ( & mut crit ) ; }