//{"name":"zflat00_html","crate":"snap_0_2_4","checksum":{"method":"sha256-generic-array","value":[96,59,239,21,134,156,168,28,229,20,63,203,68,6,63,14,50,55,219,156,192,151,120,92,98,38,164,115,150,37,171,102]}}
extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; snap_0_2_4 :: zflat00_html ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; snap_0_2_4 :: zflat00_html ( & mut crit ) ; }