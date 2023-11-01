use crate::definitions::*;

#[derive(Debug)]
pub struct Hypergraph {
    pub out_nets: Vec<Vec<Net>>,
    pub num_nodes: Int,
    pub num_nets: Int,
}

impl From<Vec<Net>> for Hypergraph {
    fn from(net_list: Vec<Net>) -> Self {
        // determine number of nodes
        let num_nodes = 1 + net_list.iter().map(|net: &Net| net.pins.iter().max().unwrap()).max().unwrap();
        let num_nets = net_list.len();
        let mut out_nets: Vec<Vec<Net>> = vec![Vec::new(); num_nodes];
        
        // for each net e add an outgoing net e' for each pin contained in e
        for net in net_list.iter() {
            for i in 0..net.pins.len() {
                let v = net.pins[i];
                let mut e = net.pins.clone();

                // remove v from net
                e.remove(i);
                out_nets[v].push(Net::new(e, net.id));
            }
        }
        Hypergraph{out_nets, num_nodes, num_nets}
    }
}