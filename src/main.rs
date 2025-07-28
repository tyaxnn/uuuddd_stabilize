use uuuddd_stabilize::{
    cal_e::{output_all_energy_vs_n,phase_diagram_j,phase_diagram_lambda},
    model::Param,
};

fn main() {
    let param = Param::new(0.3, 0.25, 0.0);
    let graph_mesh = 200; // Mesh size for k-space sampling

    output_all_energy_vs_n(&param, graph_mesh);
    //phase_diagram_j(graph_mesh,0.01);
    phase_diagram_lambda(graph_mesh,0.175);
}
