//{"name":"map_collect::i_to_i ::\n                    with_linked_list_map_reduce_vec_sized","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[95,13,75,97,117,201,87,205,62,139,149,121,233,97,32,46,86,169,57,66,248,241,17,244,195,236,114,61,190,207,96,245]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: map_collect :: i_to_i :: with_linked_list_map_reduce_vec_sized ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: map_collect :: i_to_i :: with_linked_list_map_reduce_vec_sized ( & mut crit ) ; }