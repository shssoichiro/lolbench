//{"name":"zflat10_pb","crate":"snap_0_2_4","checksum":{"method":"sha256-generic-array","value":[173,28,128,65,169,19,11,16,238,77,150,21,153,18,226,174,86,142,164,73,177,238,221,161,17,211,34,179,23,124,67,158]}}
extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; snap_0_2_4 :: zflat10_pb ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; snap_0_2_4 :: zflat10_pb ( & mut crit ) ; }