use byteorder::{ByteOrder, LittleEndian};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BTreeError {
    #[error("the requested index is out of bounds")]
    IndexOutOfBounds
}

struct BNode {
    data: [u8]
}

impl BNode {
    fn b_type(&self) -> u16 {
        LittleEndian::read_u16(&self.data[0..])
    }

    fn n_keys(&self) -> u16 {
        LittleEndian::read_u16(&self.data[2..])
    }

    fn set_header(&mut self, btype: u16, nkeys: u16) {
        LittleEndian::write_u16_into(&[btype, nkeys], &mut self.data[0..])
    }

    fn get_ptr(&self, idx: u16) -> Result<u64, BTreeError> {
        if idx >= self.n_keys() {
            return Err(BTreeError::IndexOutOfBounds);
        }
        let pos: usize = (HEADER as u16 + 8*idx) as usize;
        Ok(LittleEndian::read_u64(&self.data[pos..]))
    }

    fn set_ptr(&mut self, idx: u16, val: u64) -> Result<(), BTreeError>{
        if idx >= self.n_keys() {
            return Err(BTreeError::IndexOutOfBounds);
        }
        let pos: usize = (HEADER as u16 + 8*idx) as usize;
        LittleEndian::write_u64(&mut self.data[pos..], val);
        Ok(())
    }

    /*
    // offset list
func offsetPos(node BNode, idx uint16) uint16 {
    assert(1 <= idx && idx <= node.nkeys())
    return HEADER + 8*node.nkeys() + 2*(idx-1)
}
func (node BNode) getOffset(idx uint16) uint16 {
    if idx == 0 {
        return 0
    }
    return binary.LittleEndian.Uint16(node.data[offsetPos(node, idx):])
}
func (node BNode) setOffset(idx uint16, offset uint16) {
    binary.LittleEndian.PutUint16(node.data[offsetPos(node, idx):], offset)
}

// key-values
func (node BNode) kvPos(idx uint16) uint16 {
    assert(idx <= node.nkeys())
    return HEADER + 8*node.nkeys() + 2*node.nkeys() + node.getOffset(idx)
}
func (node BNode) getKey(idx uint16) []byte {
    assert(idx < node.nkeys())
    pos := node.kvPos(idx)
    klen := binary.LittleEndian.Uint16(node.data[pos:])
    return node.data[pos+4:][:klen]
}
func (node BNode) getVal(idx uint16) []byte {
    assert(idx < node.nkeys())
    pos := node.kvPos(idx)
    klen := binary.LittleEndian.Uint16(node.data[pos+0:])
    vlen := binary.LittleEndian.Uint16(node.data[pos+2:])
    return node.data[pos+4+klen:][:vlen]
}

// node size in bytes
func (node BNode) nbytes() uint16 {
    return node.kvPos(node.nkeys())
}
    */
}

const BNODE_NODE: u8 = 1; // internal nodes without values
const BNODE_LEAF: u8 = 2; // internal nodes without values

const HEADER: u8 = 4;

const BTREE_PAGE_SIZE: usize = 4096;
const BTREE_MAX_KEY_SIZE: usize = 1000;
const BTREE_MAX_VAL_SIZE: usize = 3000;

struct BTree {
    // pointer (a nonzero page number)
    root: u64,
    // callbacks for managing on-disk pages
    get: fn(u64) -> BNode, // dereference a pointer
    new: fn(BNode) -> u64, // allocate a new page
    del: fn(u64)  // deallocate a page
}

