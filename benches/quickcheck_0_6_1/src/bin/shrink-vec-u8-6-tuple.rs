//{"name":"shrink_vec_u8_6_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[182,43,62,243,124,169,83,215,26,30,5,58,111,174,79,112,45,54,48,14,29,56,240,77,151,241,99,136,94,62,63,107]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_vec_u8_6_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_vec_u8_6_tuple ( & mut crit ) ; }