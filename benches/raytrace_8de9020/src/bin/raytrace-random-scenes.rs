//{"name":"raytrace_random_scenes","crate":"raytrace_8de9020","checksum":{"method":"sha256-generic-array","value":[178,35,158,236,140,162,67,99,67,82,115,208,119,178,136,137,28,168,50,40,203,233,227,48,150,57,220,205,83,46,248,38]}}
extern crate raytrace_8de9020 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; raytrace_8de9020 :: raytrace_random_scenes ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; raytrace_8de9020 :: raytrace_random_scenes ( & mut crit ) ; }