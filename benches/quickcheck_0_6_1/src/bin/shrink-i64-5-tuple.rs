//{"name":"shrink_i64_5_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[136,8,155,53,171,163,136,102,155,255,133,180,144,15,196,94,68,4,152,141,245,150,255,97,169,176,95,30,212,60,185,139]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_i64_5_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_i64_5_tuple ( & mut crit ) ; }