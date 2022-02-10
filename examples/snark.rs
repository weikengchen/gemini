use ark_ec::bls12::Bls12;
use ark_ec::{AffineCurve, PairingEngine};
use ark_gemini::iterable::dummy::{dummy_r1cs_stream, DummyStreamer};
use ark_gemini::kzg::CommitterKeyStream;
use ark_gemini::snark::Proof;
use ark_std::rand::Rng;
use ark_std::test_rng;
use clap::Parser;

type PE = Bls12<ark_bls12_381::Parameters>;
type G1 = <Bls12<ark_bls12_381::Parameters> as PairingEngine>::G1Affine;
type G2 = <Bls12<ark_bls12_381::Parameters> as PairingEngine>::G2Affine;


/// Simple option handling for instance size and prover mode.
#[derive(Parser, Debug)]
#[clap(name = "snark")]
struct SnarkConfig {
    /// Size of the instance to be run (logarithmic)
    #[clap(short, long)]
    instance_logsize: usize,

    #[clap(long)]
    time_prover: bool,
}

fn elastic_snark_main(rng: &mut impl Rng, instance_logsize: usize) {
    let instance_size = 1 << instance_logsize;

    let g1 = G1::prime_subgroup_generator();
    let g2 = G2::prime_subgroup_generator();
    let r1cs_stream = dummy_r1cs_stream(rng, instance_size);
    let ck = CommitterKeyStream::<PE, _> {
        powers_of_g: DummyStreamer::new(g1, instance_size + 1),
        powers_of_g2: vec![g2; 4],
    };
    println!("Proving an instance of log size  {}", instance_logsize);
    Proof::new_elastic(r1cs_stream, ck);
}

fn time_snark_main(rng: &mut impl Rng, instance_logsize: usize) {
    let num_constraints = 1 << instance_logsize;
    let num_variables = 1 << instance_logsize;

    // let circuit = ark_gemini::circuit::random_circuit(rng, num_constraints, num_variables);
    // let r1cs = ark_gemini::circuit::generate_relation(circuit);
    let r1cs = ark_gemini::circuit::dummy_r1cs(rng, num_constraints);
    let ck = ark_gemini::kzg::CommitterKey::<ark_bls12_381::Bls12_381>::new(
        num_constraints + num_variables,
        5,
        rng,
    );

    println!("Proving an instance of log size  {}", instance_logsize);
    Proof::new_time(&r1cs, &ck);
}

fn main() {
    let rng = &mut test_rng();
    let snark_config = SnarkConfig::parse();
    env_logger::init();
    memory_traces();

    if snark_config.time_prover {
        time_snark_main(rng, snark_config.instance_logsize)
    } else {
        elastic_snark_main(rng, snark_config.instance_logsize)
    }
}
