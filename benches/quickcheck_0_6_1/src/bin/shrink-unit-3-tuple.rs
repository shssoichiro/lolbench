//{"name":"shrink_unit_3_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[66,85,36,145,25,58,80,179,80,164,26,182,152,74,21,150,253,230,193,230,3,138,170,180,243,142,46,40,119,170,113,208]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_unit_3_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_unit_3_tuple ( & mut crit ) ; }