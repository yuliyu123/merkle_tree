#[cfg(test)]
mod tests {

    use merkle_tree::{keccak256, MerkleTree};

    #[test]
    fn test_merkle_proof_1() {
        let items = ["foo", "bar", "baz", "qux"];
        let tree = MerkleTree::new(items.to_vec());

        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 2);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_merkle_proof_2() {
        let items = ["foo", "bar", "baz", "qux", "quux", "quuux"];
        let tree = MerkleTree::new(items.to_vec());

        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 3);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_merkle_proof_3() {
        let items = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        let tree = MerkleTree::new(items.to_vec());

        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 4);
            assert_eq!(result, true);
        }
    }

    // home problem dataset.
    #[test]
    fn test_merkle_proof_4() {
        let items = ["abc", "bcd", "cde", "def", "efg"];
        let tree = MerkleTree::new(items.to_vec());
        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 3);
            assert_eq!(result, true);
        }
    }
}
