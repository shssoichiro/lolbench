//{"name":"shrink_string_6_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[201,209,170,252,91,14,127,7,237,180,250,20,141,227,38,147,76,48,54,226,48,185,141,98,130,38,45,39,241,63,68,69]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_string_6_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_string_6_tuple ( & mut crit ) ; }