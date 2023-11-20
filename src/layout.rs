// pub trait RankedIx{ //where
//     const RANK :usize ; }

// Rank is a property of an instance... right?
pub trait Layout<const RANK: usize> {
    // add code here
    type Address; // should this have a copy or clone?
                  // const RANK : usize ;

    fn compare_index(&self, lhs: Index<{ RANK }>, rhs: Index<{ RANK }>) -> std::cmp::Ordering;
    // type Index;
    // type Index : RankedIx ; // should always be []
    // forall v : Index,  v.length = RANK ; all else is an error
    // type Index= [usize;Self::RANK];
    fn first_address(&self) -> Self::Address;
    fn last_address(&self) -> Self::Address;
    fn next_address(&self, addr: Self::Address) -> Option<Self::Address>;
    /*

    */
    fn address2index(&self, addr: Self::Address) -> Option<Index<{ RANK }>>;
    //  seek index is conceptually like next address,
    // but rather than computing the successor Index, it find the
    //
    fn seek_index(
        &self,
        ix: Index<{ RANK }>,
        guess: Option<Self::Address>,
    ) -> Option<(Index<{ RANK }>, Self::Address)>;
}

pub type Index<const RANK: usize> = [usize; RANK];

pub fn next_index<const RANK: usize, T: Layout<{ RANK }>>() -> () {
    return ();
}
