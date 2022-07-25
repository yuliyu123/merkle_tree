use merkle_tree::{keccak256, MerkleTree};

fn main() {
    let items = ["abc", "bcd", "cde", "def", "efg"];
    let tree = MerkleTree::new(items.to_vec());
    let root = tree.merkle_root();
    let proof = tree.merkle_proof(items[1]);
    let leaf = keccak256(items[1].as_bytes());
    let result = MerkleTree::verify_proof(root, proof, leaf);
    println!("Verify result: {:?}", result);
}
