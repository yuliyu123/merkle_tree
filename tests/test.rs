#[cfg(test)]
mod tests {

    use merkle_tree::{keccak256, MerkleTree};

    #[test]
    fn test_merkle_proof() {
        let items = ["foo", "bar", "baz", "qux"];
        let tree = MerkleTree::new(items.to_vec());
        let proof = tree.merkle_proof(items[0]);
        let length = proof.len();
        let leaf = keccak256(items[0].as_bytes());
        let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
        assert_eq!(length, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_merkle_proof_2() {
        let items = ["foo", "bar", "baz", "qux"];
        let tree = MerkleTree::new(items.to_vec());
        let proof = tree.merkle_proof(items[3]);
        let length = proof.len();
        let leaf = keccak256(items[3].as_bytes());
        let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
        assert_eq!(length, 2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_merkle_proof_3() {
        let items = ["foo", "bar", "baz", "qux", "quux", "quuux"];
        let tree = MerkleTree::new(items.to_vec());
        let proof = tree.merkle_proof(items[5]);
        let length = proof.len();
        let leaf = keccak256(items[5].as_bytes());
        let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
        assert_eq!(length, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_merkle_proof_4() {
        let items = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        let tree = MerkleTree::new(items.to_vec());
        let proof = tree.merkle_proof(items[1]);
        let length = proof.len();
        let leaf = keccak256(items[1].as_bytes());
        let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
        assert_eq!(length, 4);
        assert_eq!(result, true);
    }

    // home problem dataset.
    #[test]
    fn test_merkle_proof_5() {
        let items = ["abc", "bcd", "cde", "def", "efg"];
        let tree = MerkleTree::new(items.to_vec());
        let proof = tree.merkle_proof(items[1]);
        let leaf = keccak256(items[1].as_bytes());
        let result = MerkleTree::verify_proof(tree.merkle_root(), proof.clone(), leaf);
        assert_eq!(result, true);
    }
}
