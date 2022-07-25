# Overview 

Merkle tree implemention in Rust.

public function:

* `MerkleTree::new` : Construct a merkle tree.
* `MerkleTree::merkle_proof` : Generate merkle proof for the given item.
* `MerkleTree::verify_proof` : Verify the given proof is true or not.
  
# Basic example

```
    let items = ["abc", "bcd", "cde", "def", "efg"];
    let tree = MerkleTree::new(items.to_vec());
    let proof = tree.merkle_proof(items[1]);
    let leaf = keccak256(items[1].as_bytes());
    let result = MerkleTree::verify_proof(tree.get_root(), proof, leaf);
    println!("Verify result: {:?}", result);
```

# Design
## Construct merkle tree
Sort the item list after hash them, then construct from layer one to last layer, finally reverse the layers. The layer[0][0] is the root hash.

## Generate proof
Get the corresponding item of each layer by the given item. For example, if item list have five item, it should have four layers and the tree height is three.
If we want to verify sorted item[3] after hash them, we need get layer[3][2], layer[2][0] and layer[1][1] respectively,

## Verify the proof
Hash the given item and each item of the proof list, we can get the final hash value, then verify whether root hash is equal or not.

## Reference: https://ethbook.abyteahead.com/ch4/merkle.html
