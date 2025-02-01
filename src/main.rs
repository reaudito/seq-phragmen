use std::collections::{HashMap, HashSet};
use std::f64::INFINITY;

#[derive(Debug, Clone)]
struct Edge {
    voterid: String,
    canid: String,
    index: usize,
    voterindex: usize,
    canindex: usize,
}

impl Edge {
    fn new(voterid: String, canid: String) -> Self {
        Edge {
            voterid,
            canid,
            index: 0,
            voterindex: 0,
            canindex: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Voter {
    voterid: String,
    budget: f64,
    edges: Vec<Edge>,
    index: usize,
}

impl Voter {
    fn new(votetuple: (String, f64, Vec<String>)) -> Self {
        let voterid = votetuple.0;
        let budget = votetuple.1;
        let edges = votetuple
            .2
            .into_iter()
            .map(|canid| Edge::new(voterid.clone(), canid))
            .collect();
        Voter {
            voterid,
            budget,
            edges,
            index: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Candidate {
    canid: String,
    index: usize,
}

impl Candidate {
    fn new(canid: String, index: usize) -> Self {
        Candidate { canid, index }
    }
}

#[derive(Debug, Clone)]
struct Assignment {
    voterlist: Vec<Voter>,
    candidates: Vec<Candidate>,
    edgelist: Vec<Edge>,
    voterload: Vec<f64>,
    edgeload: Vec<f64>,
    edgeweight: Vec<f64>,
    cansupport: Vec<f64>,
    canelected: Vec<bool>,
    electedcandidates: HashSet<usize>,
    canapproval: Vec<f64>,
    canscore: Vec<f64>,
    canscorenumerator: Vec<f64>,
    canscoredenominator: Vec<f64>,
}

impl Assignment {
    fn new(
        voterlist: Vec<Voter>,
        candidates: Vec<Candidate>,
        copyassignment: Option<&Assignment>,
    ) -> Self {
        if let Some(copy) = copyassignment {
            Assignment {
                voterlist: voterlist.clone(),
                candidates: candidates.clone(),
                edgelist: copy.edgelist.clone(),
                voterload: copy.voterload.clone(),
                edgeload: copy.edgeload.clone(),
                edgeweight: copy.edgeweight.clone(),
                cansupport: copy.cansupport.clone(),
                canelected: copy.canelected.clone(),
                electedcandidates: copy.electedcandidates.clone(),
                canapproval: copy.canapproval.clone(),
                canscore: copy.canscore.clone(),
                canscorenumerator: copy.canscorenumerator.clone(),
                canscoredenominator: copy.canscoredenominator.clone(),
            }
        } else {
            let edgelist = voterlist
                .iter()
                .flat_map(|v| v.edges.clone())
                .collect::<Vec<_>>();
            let numvoters = voterlist.len();
            let numcandidates = candidates.len();
            let numedges = edgelist.len();
            let mut canapproval = vec![0.0; numcandidates];
            for voter in &voterlist {
                for edge in &voter.edges {
                    canapproval[edge.canindex] += voter.budget;
                }
            }
            Assignment {
                voterlist,
                candidates,
                edgelist,
                voterload: vec![0.0; numvoters],
                edgeload: vec![0.0; numedges],
                edgeweight: vec![0.0; numedges],
                cansupport: vec![0.0; numcandidates],
                canelected: vec![false; numcandidates],
                electedcandidates: HashSet::new(),
                canapproval,
                canscore: vec![0.0; numcandidates],
                canscorenumerator: vec![0.0; numcandidates],
                canscoredenominator: vec![1.0; numcandidates],
            }
        }
    }

    fn setload(&mut self, edge: &Edge, load: f64) {
        let oldload = self.edgeload[edge.index];
        self.edgeload[edge.index] = load;
        self.voterload[edge.voterindex] += load - oldload;
    }

    fn setweight(&mut self, edge: &Edge, weight: f64) {
        let oldweight = self.edgeweight[edge.index];
        self.edgeweight[edge.index] = weight;
        self.cansupport[edge.canindex] += weight - oldweight;
    }

    fn setscore(&mut self, candidate: &Candidate, score: f64) {
        self.canscore[candidate.index] = score;
    }

    fn loadstoweights(&mut self) {
        for voter_index in 0..self.voterlist.len() {
            let voter = self.voterlist[voter_index].clone();
            let voter_load = self.voterload[voter_index];
            if voter_load > 0.0 {
                for edge in &voter.edges {
                    let edge_load = self.edgeload[edge.index];
                    let weight = voter.budget * edge_load / voter_load;
                    self.setweight(&edge.clone(), weight.clone());
                }
            }
        }
    }

    fn weightstoloads(&mut self) {
        for edge_index in 0..self.edgelist.len() {
            let edge = self.edgelist[edge_index].clone();
            let edge_weight = self.edgeweight[edge_index];
            let can_support = self.cansupport[edge.canindex];
            if can_support > 0.0 {
                self.setload(&edge, edge_weight / can_support);
            }
        }
    }

    fn elect(&mut self, candidate: &Candidate) {
        self.canelected[candidate.index] = true;
        self.electedcandidates.insert(candidate.index);
    }

    fn unelect(&mut self, candidate: &Candidate) {
        self.canelected[candidate.index] = false;
        self.electedcandidates.remove(&candidate.index);
    }
}

fn setuplists(votelist: Vec<(String, f64, Vec<String>)>) -> (Vec<Voter>, Vec<Candidate>) {
    let mut voterlist = Vec::new();
    let mut candidatedict = HashMap::new();
    let mut candidatearray = Vec::new();
    let mut numcandidates = 0;
    let mut numvoters = 0;
    let mut numedges = 0;

    for votetuple in votelist {
        let mut voter = Voter::new(votetuple);
        voter.index = numvoters;
        numvoters += 1;
        for edge in &mut voter.edges {
            edge.index = numedges;
            edge.voterindex = voter.index;
            numedges += 1;
            let canid = edge.canid.clone();
            if let Some(&canindex) = candidatedict.get(&canid) {
                edge.canindex = canindex;
            } else {
                candidatedict.insert(canid.clone(), numcandidates);
                let newcandidate = Candidate::new(canid, numcandidates);
                candidatearray.push(newcandidate);
                edge.canindex = numcandidates;
                numcandidates += 1;
            }
        }
        voterlist.push(voter);
    }
    (voterlist, candidatearray)
}

fn seq_phragmen(votelist: Vec<(String, f64, Vec<String>)>, numtoelect: usize) -> Assignment {
    let (nomlist, candidates) = setuplists(votelist);
    let candidates_clone = candidates.clone();
    let mut a = Assignment::new(nomlist, candidates_clone, None);

    for _ in 0..numtoelect {
        for canindex in 0..candidates.len() {
            if !a.canelected[canindex] {
                a.canscore[canindex] = 1.0 / a.canapproval[canindex];
            }
        }
        for nom in &a.voterlist {
            for edge in &nom.edges {
                if !a.canelected[edge.canindex] {
                    a.canscore[edge.canindex] +=
                        nom.budget * a.voterload[nom.index] / a.canapproval[edge.canindex];
                }
            }
        }
        let mut bestcandidate = 0;
        let mut bestscore = INFINITY;
        for canindex in 0..candidates.len() {
            if !a.canelected[canindex] && a.canscore[canindex] < bestscore {
                bestscore = a.canscore[canindex];
                bestcandidate = canindex;
            }
        }
        let electedcandidate = &candidates[bestcandidate];
        a.canelected[bestcandidate] = true;
        a.elect(electedcandidate);
        for nom_index in 0..a.voterlist.len() {
            let nom = a.voterlist[nom_index].clone();
            for edge in &nom.edges {
                if edge.canindex == bestcandidate {
                    let load = a.canscore[bestcandidate] - a.voterload[nom_index];
                    a.setload(edge, load);
                }
            }
        }
    }
    a.loadstoweights();
    a
}

fn main() {
    let votelist = vec![
        (
            "A".to_string(),
            10.0,
            vec!["X".to_string(), "Y".to_string()],
        ),
        (
            "B".to_string(),
            20.0,
            vec!["X".to_string(), "Z".to_string()],
        ),
        (
            "C".to_string(),
            30.0,
            vec!["Y".to_string(), "Z".to_string()],
        ),
        (
            "C".to_string(),
            50.0,
            vec!["Z".to_string()],
        ),
    ];
    let a = seq_phragmen(votelist, 2);
    println!("{:?}", a.electedcandidates);

    let elected_names: Vec<String> = a
        .electedcandidates
        .iter()
        .map(|&index| a.candidates[index].canid.clone())
        .collect();
    println!("Elected candidates: {:?}", elected_names);
}
