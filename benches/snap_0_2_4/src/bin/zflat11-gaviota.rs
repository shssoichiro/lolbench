//{"name":"zflat11_gaviota","crate":"snap_0_2_4","checksum":{"method":"sha256-generic-array","value":[219,72,73,146,122,220,8,251,221,145,124,181,29,173,192,89,158,8,96,195,4,97,10,206,56,98,40,229,201,10,132,55]}}
extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; snap_0_2_4 :: zflat11_gaviota ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; snap_0_2_4 :: zflat11_gaviota ( & mut crit ) ; }