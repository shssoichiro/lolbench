//{"name":"zflat01_urls","crate":"snap_0_2_4","checksum":{"method":"sha256-generic-array","value":[128,182,70,129,189,45,45,174,19,135,85,142,86,31,230,120,148,137,114,195,97,166,133,28,134,90,10,43,6,42,121,142]}}
extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; snap_0_2_4 :: zflat01_urls ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; snap_0_2_4 :: zflat01_urls ( & mut crit ) ; }