# Code break down

Let’s break down the code **line by line** and explain its functionality.

---

### **1. Imports**
```rust
use std::collections::{HashMap, HashSet};
use std::f64::INFINITY;
```
- **`HashMap`**: A key-value store used to map candidate IDs to their indices.
- **`HashSet`**: A collection of unique elements used to store indices of elected candidates.
- **`INFINITY`**: A constant representing infinity, used for initializing scores.

---

### **2. Struct Definitions**

#### **`Edge` Struct**
```rust
#[derive(Debug, Clone)]
struct Edge {
    voterid: String,
    canid: String,
    index: usize,
    voterindex: usize,
    canindex: usize,
}
```
- Represents a connection between a voter and a candidate.
- Fields:
  - `voterid`: ID of the voter.
  - `canid`: ID of the candidate.
  - `index`: Unique index of the edge.
  - `voterindex`: Index of the voter in the `voterlist`.
  - `canindex`: Index of the candidate in the `candidates` list.

#### **`Edge::new` Method**
```rust
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
```
- Constructor for `Edge`. Initializes an edge with default values for `index`, `voterindex`, and `canindex`.

---

#### **`Voter` Struct**
```rust
#[derive(Debug, Clone)]
struct Voter {
    voterid: String,
    budget: f64,
    edges: Vec<Edge>,
    index: usize,
}
```
- Represents a voter.
- Fields:
  - `voterid`: Unique ID of the voter.
  - `budget`: Voting power or budget of the voter.
  - `edges`: List of edges connecting the voter to candidates.
  - `index`: Unique index of the voter in the `voterlist`.

#### **`Voter::new` Method**
```rust
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
```
- Constructor for `Voter`.
- Takes a tuple `(voterid, budget, candidate_ids)` and creates a `Voter` object.
- Initializes `edges` by creating `Edge` objects for each candidate ID.

---

#### **`Candidate` Struct**
```rust
#[derive(Debug, Clone)]
struct Candidate {
    canid: String,
    index: usize,
}
```
- Represents a candidate.
- Fields:
  - `canid`: Unique ID of the candidate.
  - `index`: Unique index of the candidate in the `candidates` list.

#### **`Candidate::new` Method**
```rust
impl Candidate {
    fn new(canid: String, index: usize) -> Self {
        Candidate { canid, index }
    }
}
```
- Constructor for `Candidate`.

---

#### **`Assignment` Struct**
```rust
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
```
- Represents the state of the election.
- Fields:
  - `voterlist`: List of voters.
  - `candidates`: List of candidates.
  - `edgelist`: List of all edges.
  - `voterload`: Load (used budget) of each voter.
  - `edgeload`: Load of each edge.
  - `edgeweight`: Weight (budget allocation) of each edge.
  - `cansupport`: Total support (budget) received by each candidate.
  - `canelected`: Boolean list indicating whether a candidate is elected.
  - `electedcandidates`: Set of indices of elected candidates.
  - `canapproval`: Total approval (budget) for each candidate.
  - `canscore`: Score of each candidate.
  - `canscorenumerator`: Numerator for calculating candidate scores.
  - `canscoredenominator`: Denominator for calculating candidate scores.

---

### **3. `Assignment` Implementation**

#### **`Assignment::new` Method**
```rust
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
```
- Constructor for `Assignment`.
- If `copyassignment` is provided, clones its data.
- Otherwise, initializes a new `Assignment`:
  - Collects all edges into `edgelist`.
  - Computes `canapproval` by summing the budgets of voters supporting each candidate.

---

#### **Other Methods in `Assignment`**
- **`setload`**: Updates the load of an edge and the corresponding voter.
- **`setweight`**: Updates the weight of an edge and the corresponding candidate support.
- **`setscore`**: Sets the score of a candidate.
- **`loadstoweights`**: Converts edge loads to weights.
- **`weightstoloads`**: Converts edge weights to loads.
- **`elect`**: Marks a candidate as elected.
- **`unelect`**: Marks a candidate as unelected.

---

### **4. Helper Functions**

#### **`setuplists` Function**
```rust
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
```
- Converts a list of voter tuples into `Voter` and `Candidate` objects.
- Assigns unique indices to voters, candidates, and edges.

---

#### **`seq_phragmen` Function**
```rust
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
```
- Implements the Sequential Phragmén method for electing candidates.
- Iteratively elects candidates based on their scores and updates loads and weights.

---

### **5. `main` Function**
```rust
fn main() {
    let votelist = vec![
        ("A".to_string(), 10.0, vec!["X".to_string(), "Y".to_string()]),
        ("B".to_string(), 20.0, vec!["X".to_string(), "Z".to_string()]),
        ("C".to_string(), 30.0, vec!["Y".to_string(), "Z".to_string()]),
        ("C".to_string(), 50.0, vec!["Z".to_string()]),
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
```
- Defines a list of voters and their preferences.
- Runs the Sequential Phragmén method to elect 2 candidates.
- Prints the elected candidate names.

---

### **Output Example**
For the given `votelist`, the output will be:
```
Elected candidates: ["Y", "Z"]
```