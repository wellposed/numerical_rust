// pub trait RankedIx{ //where
//     const RANK :usize ; }

// Rank is a property of an instance... right?
pub trait Layout<const RANK: usize> {
    // add code here
    type Address: Copy + Clone + Sized; // should this have a copy or clone? is Sized always true?
                                        // const RANK : usize ;

    type TransposedLayout: Layout<RANK>;
    fn compare_index(
        &self,
        lhs: Index<{ RANK }>,
        rhs: Index<{ RANK }>,
    ) -> Option<std::cmp::Ordering>;
    // type Index;
    // const
    // type Index : RankedIx ; // should always be []
    // forall v : Index,  v.length = RANK ; all else is an error
    // type Index= [usize;Self::RANK];
    fn first_address(&self) -> Self::Address;
    fn last_address(&self) -> Self::Address;

    /*

    */
    fn address2index(&self, addr: Self::Address) -> Index<RANK>;
    //  seek index is conceptually like next address,
    // but rather than computing the successor Index, it find the
    //

    // next_address gives the successor "manifest" entry in layout comparison over
    // wrt the coordinate space
    fn next_address(&self, addr: Self::Address) -> Option<Self::Address>;

    // more general than 'next_address', address_shift takes a base address,
    // and does 
    fn address_shift(&self, addr: Self::Address, shift: i64) -> Option<Self::Address>;
   

    fn seek_index(
        &self,
        ix: Index<RANK>,
        guess: Option<Self::Address>,
    ) -> Option<(Index<RANK>, Self::Address)>;

    fn next_index(&self, ix: Index<RANK>) -> Option<(Index<RANK>, Self::Address)> {
        self.seek_index(ix, None)
    }

    // fn next_index_until(&self ) // we want something that limits the span  of the scan!

    fn index2address(&self, ix: Index<RANK>) -> Option<Self::Address>;

    // returns the number of manifest entries, inclusive interval, defined only for valid addresses
    fn pop_count(&self, lb: Self::Address, ub: Self::Address) -> usize;

    /*
    might want to add addr2count : &self X

        */
}

pub struct LayoutIterator<T: Layout<RANK>, const RANK: usize> {
    pub layout: T,
    pub addr: T::Address,
}

pub type Index<const RANK: usize> = [usize; RANK];
