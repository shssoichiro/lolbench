//{"name":"map_collect::i_to_i :: with_linked_list_collect","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[67,80,144,53,57,36,73,96,170,32,212,119,17,217,61,214,239,109,93,213,9,29,212,163,35,29,44,105,106,4,13,59]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: map_collect :: i_to_i :: with_linked_list_collect ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: map_collect :: i_to_i :: with_linked_list_collect ( & mut crit ) ; }